from __future__ import annotations

import atexit
import copy
import inspect
import json
import os
import stat
import subprocess
import sys
import unittest
import uuid
from pathlib import Path
from types import SimpleNamespace
from unittest import mock


ROOT = Path(__file__).resolve().parents[1]
REPO_ROOT = ROOT.parents[3]
sys.dont_write_bytecode = True
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

import diagnostic_executor as executor
import frozen_profile_diff as frozen_profile_diff
from diagnostic_executor import (
    ExecutorFailure,
    LaunchCapture,
    P44CustodyBinding,
    TerminalPulse47Once,
    _digest,
    _execution_event,
    _run_qualification_executor,
    _validate_p45_bridge_result,
    build_platform_dispatch,
    canonical_platform_id,
    invoke_terminal_pulse47_once,
    profile_diff_argv,
    resolve_python_launcher,
    validate_descriptor_root,
    windows_to_wsl_absolute,
)
from p31_contract_verifier import P31Failure, verify_bound_contract
from p35_p37_custody import CustodyFailure, canonical_lf, verify_p35_p37_custody
from sealed_dependencies import load_terminal_dependencies
from synthetic_fixture import (
    SCRATCH_CLEANUP_DELAYS,
    cleanup_synthetic_runtime_root,
    create_descriptor_root,
    create_synthetic_custodies,
    synthetic_p44_summary,
)


RUN_ROOT = REPO_ROOT / "target" / "pulse-51-test-runtime"


def _clean_sealed_python_residue() -> None:
    for path in (ROOT / ".qualification-work", ROOT / "tests" / ".run"):
        if path.exists():
            cleanup_synthetic_runtime_root(path)
    for path in sorted(ROOT.rglob("__pycache__"), key=lambda value: len(value.parts), reverse=True):
        cleanup_synthetic_runtime_root(path)


_clean_sealed_python_residue()
atexit.register(_clean_sealed_python_residue)


def p27_summary() -> dict[str, object]:
    return {
        "schema": "exact-two-preflight-cycle-v1",
        "outcome": "pass",
        "pair_ids": ["preflight-pair-000", "preflight-pair-001"],
        "pair_count": 2,
        "windows_record_count": 2,
        "ubuntu_record_count": 2,
        "process_record_count": 4,
        "pair_seal_count": 2,
        "durable_write_count": 6,
        "fresh_process_reload_count": 2,
        "fresh_verifiers": {"windows": {}, "ubuntu": {}},
        "residue_count": 0,
        "retries": 0,
    }


class SyntheticProcessRunner:
    """Only replaces the final subprocess boundary after exact dispatch construction."""

    def __init__(
        self,
        runtime_root: Path,
        *,
        mismatch_platform: str | None = None,
        mismatch_ordinal: str | None = None,
    ) -> None:
        self.runtime_root = runtime_root
        self.mismatch_platform = mismatch_platform
        self.mismatch_ordinal = mismatch_ordinal
        self.calls: list[executor.Dispatch] = []

    def __call__(self, dispatch: executor.Dispatch) -> LaunchCapture:
        self.calls.append(dispatch)
        self.assert_dispatch(dispatch)
        interpreter = resolve_python_launcher(
            dispatch.platform,
            which=lambda name: sys.executable if name in {"python", "python3"} else None,
        )
        environment = os.environ.copy()
        environment["PYTHONDONTWRITEBYTECODE"] = "1"
        if self.mismatch_platform is not None:
            environment["P51_SYNTHETIC_MISMATCH_PLATFORM"] = self.mismatch_platform
            environment["P51_SYNTHETIC_MISMATCH_ORDINAL"] = self.mismatch_ordinal or ""
        completed = subprocess.run(
            [
                *interpreter,
                str(ROOT / "fixtures" / "fake_ferris.py"),
                "--synthetic-platform",
                dispatch.platform,
                *dispatch.application_argv,
            ],
            cwd=dispatch.host_cwd,
            check=False,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=environment,
        )
        return LaunchCapture(completed.returncode, completed.stdout, completed.stderr)

    def assert_dispatch(self, dispatch: executor.Dispatch) -> None:
        self.assert_exact_argv(dispatch.application_argv)
        if dispatch.host_cwd != self.runtime_root:
            raise AssertionError("synthetic runner received an unexpected host cwd")
        for value in (
            dispatch.executable,
            Path(dispatch.application_argv[2]),
            Path(dispatch.application_argv[4]),
        ):
            try:
                value.relative_to(self.runtime_root)
            except ValueError as error:
                raise AssertionError("synthetic runner received an escaped path") from error
        if dispatch.platform == "windows-x86_64":
            if dispatch.wsl_cwd is not None or dispatch.command != (
                str(dispatch.executable),
                *dispatch.application_argv,
            ):
                raise AssertionError("synthetic runner received a non-native Windows command")
            return
        if dispatch.platform != "ubuntu-24.04-x86_64":
            raise AssertionError("synthetic runner received an unsupported platform")
        expected = (
            "wsl.exe",
            "--distribution",
            "Ubuntu-24.04",
            "--cd",
            windows_to_wsl_absolute(self.runtime_root, self.runtime_root),
            "--exec",
            windows_to_wsl_absolute(dispatch.executable, self.runtime_root),
            "profile-diff",
            "--before",
            windows_to_wsl_absolute(Path(dispatch.application_argv[2]), self.runtime_root),
            "--after",
            windows_to_wsl_absolute(Path(dispatch.application_argv[4]), self.runtime_root),
            "--format",
            dispatch.application_argv[6],
        )
        if dispatch.wsl_cwd != expected[4] or dispatch.command != expected:
            raise AssertionError("synthetic runner received an incorrect WSL command")

    @staticmethod
    def assert_exact_argv(argv: tuple[str, ...] | list[str]) -> None:
        if (
            len(argv) != 7
            or argv[0] != "profile-diff"
            or argv[1] != "--before"
            or argv[3] != "--after"
            or argv[5] != "--format"
            or argv[6] not in {"json", "human"}
        ):
            raise AssertionError(f"unexpected argv shape: {argv!r}")


class MalformedOutputRunner(SyntheticProcessRunner):
    def __call__(self, dispatch: executor.Dispatch) -> LaunchCapture:
        capture = super().__call__(dispatch)
        if dispatch.platform == "windows-x86_64":
            return LaunchCapture(capture.returncode, b"not frozen output\n", capture.stderr)
        return capture


def _leaf_paths(value: object, path: tuple[str, ...] = ()):
    if type(value) is dict:
        for key, child in value.items():
            yield from _leaf_paths(child, (*path, key))
    else:
        yield path


def _mutate_leaf(value: dict[str, object], path: tuple[str, ...]) -> None:
    target: object = value
    for key in path[:-1]:
        assert type(target) is dict
        target = target[key]
    assert type(target) is dict
    original = target[path[-1]]
    if type(original) is bool:
        target[path[-1]] = not original
    elif type(original) is int:
        target[path[-1]] = original + 1
    elif original is None:
        target[path[-1]] = "unexpected"
    else:
        target[path[-1]] = "unexpected"


class DiagnosticExecutorTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        _clean_sealed_python_residue()
        if RUN_ROOT.exists():
            cleanup_synthetic_runtime_root(RUN_ROOT)
        RUN_ROOT.mkdir(parents=True)

    @classmethod
    def tearDownClass(cls) -> None:
        if RUN_ROOT.exists():
            cleanup_synthetic_runtime_root(RUN_ROOT)
        _clean_sealed_python_residue()

    def setUp(self) -> None:
        self.sandbox = RUN_ROOT / uuid.uuid4().hex
        self.sandbox.mkdir(parents=True)
        self.descriptor_root = create_descriptor_root(self.sandbox / "descriptors")
        self.custodies, self.expectations = create_synthetic_custodies(
            self.sandbox / "custodies", ROOT / "fixtures" / "fake_ferris.py"
        )
        self.p27_calls = 0

    def tearDown(self) -> None:
        cleanup_synthetic_runtime_root(self.sandbox)
        if RUN_ROOT.exists():
            with os.scandir(RUN_ROOT) as directory:
                empty = next(directory, None) is None
            if empty:
                RUN_ROOT.rmdir()

    def _p27(self, path: Path) -> dict[str, object]:
        self.p27_calls += 1
        self.assertFalse(path.exists())
        path.mkdir()
        return p27_summary()

    def _run(
        self,
        runner: SyntheticProcessRunner | None = None,
        *,
        custodies: dict[str, P44CustodyBinding] | None = None,
        p27_runner=None,
    ):
        return _run_qualification_executor(
            REPO_ROOT,
            self.descriptor_root,
            self.sandbox,
            self.sandbox / "p27-cycle",
            self.custodies if custodies is None else custodies,
            p27_runner=self._p27 if p27_runner is None else p27_runner,
            process_runner=runner or SyntheticProcessRunner(self.sandbox),
            expectations=self.expectations,
        )

    def test_runs_exact_dispatches_and_full_outputs_per_platform(self) -> None:
        runner = SyntheticProcessRunner(self.sandbox)
        result = self._run(runner)
        self.assertEqual(result.private_record["outcome"], "completed")
        self.assertEqual(
            result.private_record["process_counts"],
            {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69},
        )
        self.assertEqual(len(result.private_record["no_launch_records"]), 2)
        self.assertEqual(len(runner.calls), 138)
        self.assertEqual(self.p27_calls, 1)
        self.assertTrue((self.sandbox / "p27-cycle").is_dir())
        self.assertEqual(
            result.events[-1],
            _execution_event("bounded-process-exit-search", "terminal-stop", "completed"),
        )
        first_windows = result.private_record["platform_records"]["windows-x86_64"][0]
        first_ubuntu = result.private_record["platform_records"]["ubuntu-24.04-x86_64"][0]
        self.assertIn("normalized_sha256", first_windows["result"])
        self.assertIn("raw_sha256", first_ubuntu["result"])
        self.assertEqual(
            first_windows["result"]["semantic_projection_sha256"],
            first_ubuntu["result"]["semantic_projection_sha256"],
        )
        public_text = json.dumps({"catalog": result.catalog, "events": result.events})
        for forbidden in ("case_id", "ordinal", "artifacts", "seed", "private", "wsl2"):
            self.assertNotIn(forbidden, public_text)

    def test_first_identity_mismatch_stops_later_dispositions(self) -> None:
        runner = SyntheticProcessRunner(
            self.sandbox, mismatch_platform="ubuntu-24.04-x86_64", mismatch_ordinal="001"
        )
        result = self._run(runner)
        self.assertEqual(result.private_record["outcome"], "failed")
        self.assertEqual(result.private_record["failure_code"], "P51-RESULT-DIFF-ID")
        self.assertEqual(
            result.private_record["process_counts"],
            {"windows-x86_64": 1, "ubuntu-24.04-x86_64": 0},
        )
        self.assertEqual(len(runner.calls), 2)

    def test_rejects_malformed_full_process_output(self) -> None:
        result = self._run(MalformedOutputRunner(self.sandbox))
        self.assertEqual(result.private_record["failure_code"], "P51-HUMAN-GRAMMAR")
        self.assertEqual(result.private_record["process_counts"]["windows-x86_64"], 0)

    def test_rejects_every_identity_and_record_semantic_mutation(self) -> None:
        _root, descriptors = validate_descriptor_root(self.descriptor_root)
        descriptor = next(
            item
            for item in descriptors
            if item.output_format == "json"
            and item.expected["result_class"] == "difference"
        )
        executable = self.custodies["windows-x86_64"].final_root / self.expectations[
            "windows-x86_64"
        ].logical_filename
        runner = SyntheticProcessRunner(self.sandbox)
        capture = runner(
            build_platform_dispatch("windows-x86_64", executable, descriptor, self.sandbox)
        )
        envelope = json.loads(capture.stdout)
        semantics = executor._descriptor_semantics(descriptor)
        executor._json_normalized(capture, descriptor.expected, semantics)

        def reidentity(value: dict[str, object]) -> None:
            value["result_identity"] = frozen_profile_diff.result_identity(value)

        def rejected(
            label: str,
            mutate,
            code: str,
            *,
            returncode: int = 1,
        ) -> None:
            with self.subTest(label=label):
                value = copy.deepcopy(envelope)
                mutate(value)
                altered = LaunchCapture(
                    returncode,
                    json.dumps(value, ensure_ascii=True, indent=2).encode("ascii") + b"\n",
                    b"",
                )
                with self.assertRaisesRegex(ExecutorFailure, code):
                    executor._json_normalized(altered, descriptor.expected, semantics)

        def wrong_diff_id(value: dict[str, object]) -> None:
            record = value["record"]
            assert type(record) is dict
            record["diff_id"] = "profile-diff:" + "f" * 64
            reidentity(value)

        def wrong_selection(value: dict[str, object]) -> None:
            value["selection_identity"] = "selection:" + "f" * 64
            reidentity(value)

        def wrong_invocation(value: dict[str, object]) -> None:
            value["invocation_identity"] = "invocation:" + "f" * 64
            reidentity(value)

        def wrong_result(value: dict[str, object]) -> None:
            value["result_identity"] = "result:" + "f" * 64

        def wrong_profile_id(value: dict[str, object]) -> None:
            record = value["record"]
            assert type(record) is dict
            before = record["before"]
            assert type(before) is dict
            before["profile_id"] = "forged-profile"
            record["diff_id"] = frozen_profile_diff.diff_identity(record)
            reidentity(value)

        def wrong_consumer(value: dict[str, object]) -> None:
            record = value["record"]
            assert type(record) is dict
            after = record["after"]
            assert type(after) is dict
            after["consumer"] = "forged-consumer"
            record["diff_id"] = frozen_profile_diff.diff_identity(record)
            reidentity(value)

        def wrong_change_set(value: dict[str, object]) -> None:
            record = value["record"]
            assert type(record) is dict
            changes = record["changes"]
            assert type(changes) is list
            changes.append(
                {
                    "path": "/sections/identity/zzzz",
                    "change_kind": "changed",
                    "before_value_digest": "sha256:" + "0" * 64,
                    "after_value_digest": "sha256:" + "1" * 64,
                }
            )
            record["diff_id"] = frozen_profile_diff.diff_identity(record)
            reidentity(value)

        def wrong_change_order(value: dict[str, object]) -> None:
            record = value["record"]
            assert type(record) is dict
            changes = record["changes"]
            assert type(changes) is list
            changes.reverse()
            record["diff_id"] = frozen_profile_diff.diff_identity(record)
            reidentity(value)

        def wrong_section_partition(value: dict[str, object]) -> None:
            record = value["record"]
            assert type(record) is dict
            changed = record["changed_sections"]
            unchanged = record["unchanged_sections"]
            assert type(changed) is list and type(unchanged) is list
            changed[:] = ["features"]
            unchanged.remove("features")
            unchanged.append("identity")
            unchanged.sort()
            record["diff_id"] = frozen_profile_diff.diff_identity(record)
            reidentity(value)

        def wrong_result_class(value: dict[str, object]) -> None:
            value["result_class"] = "success"
            value["process_exit_code"] = 0
            reidentity(value)

        rejected("diff-id", wrong_diff_id, "P51-RESULT-DIFF-ID")
        rejected("selection-identity", wrong_selection, "P51-RESULT-SELECTION-IDENTITY")
        rejected("invocation-identity", wrong_invocation, "P51-RESULT-INVOCATION-IDENTITY")
        rejected("result-identity", wrong_result, "P51-RESULT-IDENTITY")
        rejected("profile-id", wrong_profile_id, "P51-RESULT-SEMANTICS")
        rejected("consumer", wrong_consumer, "P51-RESULT-SEMANTICS")
        rejected("change-set", wrong_change_set, "P51-RESULT-SEMANTICS")
        rejected("change-order", wrong_change_order, "P51-RESULT-RECORD")
        rejected("section-partition", wrong_section_partition, "P51-RESULT-SEMANTICS")
        rejected("result-class", wrong_result_class, "P51-RESULT-EXPECTATION", returncode=0)

    def test_frozen_identity_vectors_match_the_public_contract(self) -> None:
        vectors = json.loads(
            (
                REPO_ROOT
                / "docs"
                / "simulations"
                / "profile-diff-held-out"
                / "fixtures"
                / "identity-vectors.json"
            ).read_bytes()
        )
        for vector in vectors["content_digests"]:
            self.assertEqual(
                frozen_profile_diff.canonical_profile_json(vector["source_value"]).decode(),
                vector["canonical_json"],
                vector["id"],
            )
            self.assertEqual(
                frozen_profile_diff.profile_content_digest(vector["source_value"]),
                vector["expected"],
                vector["id"],
            )
        for vector in vectors["value_digests"]:
            self.assertEqual(
                frozen_profile_diff.canonical_value_json(vector["source_value"]).decode(),
                vector["canonical_json"],
                vector["id"],
            )
            self.assertEqual(
                frozen_profile_diff.value_digest(vector["source_value"]),
                vector["expected"],
                vector["id"],
            )
        for vector in vectors["selection_identities"]:
            if vector["branch"] == "content":
                observed = frozen_profile_diff.selection_identity_from_content(
                    vector["before_content_digest"], vector["after_content_digest"]
                )
            elif vector["branch"] == "pre_read":
                observed = frozen_profile_diff.selection_identity_from_requests(
                    vector["before_path"], vector["after_path"]
                )
            else:
                observed = frozen_profile_diff.selection_identity_from_second_input(
                    vector["before_content_digest"], vector["after_path"]
                )
            self.assertEqual(observed, vector["expected"], vector["id"])
        for vector in vectors["invocation_identities"]:
            self.assertEqual(
                frozen_profile_diff.invocation_identity(vector["selection_identity"]),
                vector["expected"],
                vector["id"],
            )
        for vector in vectors["diff_identities"]:
            self.assertEqual(
                frozen_profile_diff.diff_identity(vector["identity_payload"]),
                vector["expected"],
                vector["id"],
            )
        for vector in vectors["result_identities"]:
            self.assertEqual(
                frozen_profile_diff.result_identity(vector["identity_payload"]),
                vector["expected"],
                vector["id"],
            )

    def test_validates_frozen_p31_artifacts_and_all_mutations(self) -> None:
        summary = verify_bound_contract(REPO_ROOT)
        self.assertEqual(summary["artifact_count"], 9)
        self.assertEqual(summary["public_input_checks"], 39)
        self.assertEqual(summary["mutation_control_count"], 33)
        with self.assertRaisesRegex(P31Failure, "P51-P31-ARTIFACT-UNAVAILABLE"):
            verify_bound_contract(self.sandbox / "missing-repository")

    def test_binds_complete_p35_tree_and_missing_git_is_bounded(self) -> None:
        summary = verify_p35_p37_custody(REPO_ROOT)
        self.assertEqual(summary["bound_file_count"], 11)
        self.assertEqual(summary["p35_release_tree_file_count"], 10)
        self.assertEqual(summary["machine_schema_count"], 1)
        self.assertEqual(_digest(canonical_lf(b"line\r\n")), _digest(b"line\n"))
        with self.assertRaisesRegex(CustodyFailure, "P51-P35-BARE-CR"):
            canonical_lf(b"line\rnext\n")
        with self.assertRaisesRegex(CustodyFailure, "P51-P35-GIT-CLEAN-UNAVAILABLE"):
            verify_p35_p37_custody(REPO_ROOT, git="p51-no-such-git")

    def test_rejects_path_escape_and_scandir_failure_before_process_launch(self) -> None:
        manifest_path = self.descriptor_root / "case-manifest.json"
        manifest = json.loads(manifest_path.read_bytes())
        manifest["cases"][0]["before"]["request"]["request_template"] = "../{target}"
        raw = json.dumps(manifest, ensure_ascii=True, separators=(",", ":"), sort_keys=True).encode(
            "ascii"
        )
        manifest_path.write_bytes(raw)
        coverage_path = self.descriptor_root / "coverage-manifest.json"
        coverage = json.loads(coverage_path.read_bytes())
        coverage["case_manifest_sha256"] = _digest(raw)
        coverage_path.write_bytes(json.dumps(coverage, separators=(",", ":"), sort_keys=True).encode("ascii"))
        with self.assertRaisesRegex(ExecutorFailure, "P51-DESCRIPTOR-REQUEST"):
            validate_descriptor_root(self.descriptor_root)

        restored = create_descriptor_root(self.sandbox / "replacement-descriptors")
        with mock.patch.object(executor.os, "scandir", side_effect=OSError("denied")):
            with self.assertRaisesRegex(ExecutorFailure, "P51-DESCRIPTOR-ROOT"):
                validate_descriptor_root(restored)

    def test_rejects_wrong_70_69_1_topology(self) -> None:
        manifest_path = self.descriptor_root / "case-manifest.json"
        manifest = json.loads(manifest_path.read_bytes())
        manifest["cases"].pop()
        manifest["case_count"] = 69
        raw = json.dumps(manifest, ensure_ascii=True, separators=(",", ":"), sort_keys=True).encode(
            "ascii"
        )
        manifest_path.write_bytes(raw)
        coverage_path = self.descriptor_root / "coverage-manifest.json"
        coverage = json.loads(coverage_path.read_bytes())
        coverage["case_manifest_sha256"] = _digest(raw)
        coverage["case_count"] = 69
        coverage_path.write_bytes(json.dumps(coverage, separators=(",", ":"), sort_keys=True).encode("ascii"))
        with self.assertRaisesRegex(ExecutorFailure, "P51-DESCRIPTOR-MANIFEST"):
            validate_descriptor_root(self.descriptor_root)

    def test_rejects_p33_binary_and_receipt_mutations_before_p45(self) -> None:
        windows = self.custodies["windows-x86_64"]
        executable = windows.final_root / self.expectations["windows-x86_64"].logical_filename
        executable.write_bytes(b"not-the-fixture")
        result = self._run()
        self.assertEqual(result.private_record["failure_code"], "P51-P33-BINARY-IDENTITY")
        self.assertEqual(self.p27_calls, 0)

        cleanup_synthetic_runtime_root(self.sandbox)
        self.setUp()
        windows = self.custodies["windows-x86_64"]
        receipt_path = windows.final_root / (
            self.expectations["windows-x86_64"].logical_filename + ".receipt.json"
        )
        receipt = json.loads(receipt_path.read_bytes())
        receipt["payload"]["build"]["rustc_version"] = "rustc wrong"
        receipt["payload_sha256"] = _digest(
            json.dumps(receipt["payload"], ensure_ascii=True, separators=(",", ":"), sort_keys=True).encode(
                "ascii"
            )
        )
        receipt_path.write_bytes(json.dumps(receipt, separators=(",", ":"), sort_keys=True).encode("ascii"))
        result = self._run()
        self.assertEqual(result.private_record["failure_code"], "P51-P33-TOOLCHAIN")
        self.assertEqual(self.p27_calls, 0)

    def test_p44_summary_and_p45_bridge_identities_reject_every_mutation(self) -> None:
        for path in _leaf_paths(synthetic_p44_summary()):
            with self.subTest(path=path):
                mutated = copy.deepcopy(self.custodies)
                summary = copy.deepcopy(synthetic_p44_summary())
                _mutate_leaf(summary, path)
                original = mutated["windows-x86_64"]
                mutated["windows-x86_64"] = P44CustodyBinding(
                    original.platform, original.final_root, original.work_root, summary
                )
                result = self._run(custodies=mutated)
                self.assertEqual(result.private_record["failure_code"], "P51-P45-CUSTODY")
                self.assertEqual(self.p27_calls, 0)

        for shape in ("missing", "extra"):
            with self.subTest(shape=shape):
                mutated = copy.deepcopy(self.custodies)
                summary = copy.deepcopy(synthetic_p44_summary())
                if shape == "missing":
                    del summary["custody"]["files"]
                else:
                    summary["unexpected"] = True
                original = mutated["windows-x86_64"]
                mutated["windows-x86_64"] = P44CustodyBinding(
                    original.platform, original.final_root, original.work_root, summary
                )
                result = self._run(custodies=mutated)
                self.assertEqual(result.private_record["failure_code"], "P51-P45-CUSTODY")

        _p43, p45, _p47 = load_terminal_dependencies(REPO_ROOT)
        valid = p45.bridge_pulse_44(
            REPO_ROOT,
            executor.P33_CUTOFF,
            "windows-x86_64",
            self.custodies["windows-x86_64"].work_root,
            self.custodies["windows-x86_64"].final_root,
            invoker=lambda *_: synthetic_p44_summary(),
        )
        for path in _leaf_paths(valid):
            with self.subTest(bridge_path=path):
                mutated = copy.deepcopy(valid)
                _mutate_leaf(mutated, path)
                with self.assertRaisesRegex(ExecutorFailure, "P51-P45-CUSTODY"):
                    _validate_p45_bridge_result(p45, mutated, "windows-x86_64")

    def test_p27_partial_root_cleanup_and_programmer_errors(self) -> None:
        cycle = self.sandbox / "p27-cycle"

        def partial(path: Path) -> dict[str, object]:
            path.mkdir()
            (path / "partial").write_text("x", encoding="ascii")
            raise OSError("synthetic P27 failure")

        result = self._run(p27_runner=partial)
        self.assertEqual(result.private_record["failure_code"], "P51-P27-EXECUTION")
        self.assertFalse(cycle.exists())

        def programmer(_: Path) -> dict[str, object]:
            raise TypeError("programmer fault")

        with self.assertRaises(TypeError):
            self._run(p27_runner=programmer)

    @unittest.skipUnless(os.name == "nt", "Windows sharing locks only")
    def test_synthetic_scratch_cleanup_retries_a_brief_real_file_lock(self) -> None:
        scratch = self.sandbox / "transient-scratch-lock"
        scratch.mkdir()
        locked = scratch / "synthetic-p44-receipt.json"
        locked.write_bytes(b"synthetic")
        holder_script = """
import ctypes
import sys
import time

kernel32 = ctypes.WinDLL("kernel32", use_last_error=True)
kernel32.CreateFileW.argtypes = (
    ctypes.c_wchar_p,
    ctypes.c_uint32,
    ctypes.c_uint32,
    ctypes.c_void_p,
    ctypes.c_uint32,
    ctypes.c_uint32,
    ctypes.c_void_p,
)
kernel32.CreateFileW.restype = ctypes.c_void_p
kernel32.CloseHandle.argtypes = (ctypes.c_void_p,)
kernel32.CloseHandle.restype = ctypes.c_int
handle = kernel32.CreateFileW(sys.argv[1], 0x80000000, 0x00000001, None, 3, 0, None)
if handle == ctypes.c_void_p(-1).value:
    raise OSError(ctypes.get_last_error(), "CreateFileW")
try:
    print("locked", flush=True)
    time.sleep(0.12)
finally:
    kernel32.CloseHandle(ctypes.c_void_p(handle))
"""
        with subprocess.Popen(
            [sys.executable, "-B", "-c", holder_script, str(locked)],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        ) as holder:
            assert holder.stdout is not None
            self.assertEqual(holder.stdout.readline(), "locked\n")
            attempts = cleanup_synthetic_runtime_root(scratch)
            stdout, stderr = holder.communicate(timeout=5)
            holder_returncode = holder.returncode
        self.assertEqual(stdout, "")
        self.assertEqual(stderr, "")
        self.assertEqual(holder_returncode, 0)
        self.assertGreaterEqual(attempts, 2)
        self.assertFalse(scratch.exists())

    def test_synthetic_scratch_cleanup_rejects_permanent_sharing_lock(self) -> None:
        scratch = self.sandbox / "permanent-scratch-lock"
        scratch.mkdir()
        (scratch / "synthetic-p44-receipt.json").write_bytes(b"synthetic")

        class SharingViolation(OSError):
            winerror = 32

        with mock.patch(
            "synthetic_fixture.shutil.rmtree", side_effect=SharingViolation("synthetic lock")
        ) as remove, mock.patch("synthetic_fixture.time.sleep") as sleep:
            with self.assertRaisesRegex(RuntimeError, "P51-SYNTHETIC-SCRATCH-CLEANUP"):
                cleanup_synthetic_runtime_root(scratch)
        self.assertEqual(remove.call_count, len(SCRATCH_CLEANUP_DELAYS) + 1)
        self.assertEqual(sleep.call_args_list, [mock.call(delay) for delay in SCRATCH_CLEANUP_DELAYS])
        self.assertTrue(scratch.exists())

    def test_real_platform_mapping_and_wsl_boundary_failures(self) -> None:
        self.assertEqual(canonical_platform_id("ubuntu-24.04-wsl2-x86_64"), "ubuntu-24.04-x86_64")
        mapped = dict(self.custodies)
        ubuntu = mapped.pop("ubuntu-24.04-x86_64")
        mapped["ubuntu-24.04-wsl2-x86_64"] = P44CustodyBinding(
            "ubuntu-24.04-wsl2-x86_64",
            ubuntu.final_root,
            ubuntu.work_root,
            ubuntu.summary,
        )
        result = self._run(custodies=mapped)
        self.assertEqual(result.private_record["outcome"], "completed")
        self.assertNotIn("wsl2", json.dumps({"catalog": result.catalog, "events": result.events}))

        _root, descriptors = validate_descriptor_root(self.descriptor_root)
        dispatch = build_platform_dispatch(
            "ubuntu-24.04-x86_64",
            ubuntu.final_root / self.expectations["ubuntu-24.04-x86_64"].logical_filename,
            descriptors[0],
            self.sandbox,
        )
        with mock.patch.object(executor.subprocess, "run", side_effect=OSError("wsl missing")):
            with self.assertRaisesRegex(ExecutorFailure, "P51-WSL-UNAVAILABLE"):
                executor._subprocess_process_runner(dispatch)

        with self.assertRaisesRegex(ExecutorFailure, "P51-WSL-PATH"):
            windows_to_wsl_absolute(Path(r"\\server\share\binary"), self.sandbox)

    def test_python_resolution_and_no_production_injection_surface(self) -> None:
        self.assertEqual(
            resolve_python_launcher(
                "ubuntu-24.04-x86_64",
                which=lambda name: {"python3": "python3-path", "python": "python-path"}.get(name),
            ),
            ("python3-path",),
        )
        self.assertEqual(
            resolve_python_launcher(
                "windows-x86_64",
                which=lambda name: {"py": "py-path"}.get(name),
            ),
            ("py-path", "-3"),
        )
        with self.assertRaisesRegex(ExecutorFailure, "P51-PYTHON-UNAVAILABLE"):
            resolve_python_launcher("windows-x86_64", which=lambda _: None)
        self.assertEqual(
            tuple(inspect.signature(executor.run_diagnostic_executor).parameters),
            (
                "repo_root",
                "descriptor_root",
                "private_runtime_root",
                "p27_cycle_root",
                "retained_custodies",
            ),
        )
        self.assertNotIn("ExecutionGrant", executor.__all__)

    def test_terminal_roots_are_safe_nonoverlapping_and_one_use(self) -> None:
        parent = self.sandbox / "terminal-parent"
        parent.mkdir()
        result = executor.ExecutorResult(
            {"schema": executor.P43_CATALOG_SCHEMA, "gate_ids": list(executor.P50_GATE_IDS)},
            [_execution_event("pulse-41-pulse-39-public-custody", "terminal-stop", "failed")],
            {},
        )

        def failure_for(left: Path, right: Path) -> dict[str, object]:
            terminal = TerminalPulse47Once(REPO_ROOT, parent)
            value = invoke_terminal_pulse47_once(terminal, result, left, right)
            self.assertIsInstance(value, dict)
            return value

        relative = failure_for(Path("relative"), parent / "witness")
        self.assertEqual(relative["failure_code"], "P47-WITNESS-FINAL-ROOT-INVALID")

        existing = parent / "existing"
        existing.mkdir()
        already = failure_for(existing, parent / "witness")
        self.assertEqual(already["failure_code"], "P47-WITNESS-FINAL-EXISTS")

        overlap = failure_for(parent / "same", parent / "same" / "nested")
        self.assertEqual(overlap["failure_code"], "P47-WITNESS-ROOTS-OVERLAP")

        unsafe_parent = self.sandbox / "not-a-directory"
        unsafe_parent.write_text("x", encoding="ascii")
        unsafe = TerminalPulse47Once(REPO_ROOT, unsafe_parent)
        unsafe_result = invoke_terminal_pulse47_once(
            unsafe, result, unsafe_parent / "p43", unsafe_parent / "witness"
        )
        self.assertEqual(unsafe_result["failure_code"], "P47-WITNESS-FINAL-PARENT-UNSAFE")

        once = TerminalPulse47Once(REPO_ROOT, parent)
        first = invoke_terminal_pulse47_once(once, result, Path("relative"), parent / "witness")
        self.assertEqual(first["failure_code"], "P47-WITNESS-FINAL-ROOT-INVALID")
        with self.assertRaisesRegex(ExecutorFailure, "P51-P47-ALREADY-INVOKED"):
            invoke_terminal_pulse47_once(once, result, parent / "p43", parent / "witness")

        unavailable = TerminalPulse47Once(REPO_ROOT, parent)
        with mock.patch.object(
            executor,
            "load_terminal_dependencies",
            side_effect=executor.DependencyFailure("P51-SEALED-IDENTITY"),
        ):
            bounded = invoke_terminal_pulse47_once(
                unavailable, result, parent / "p43-unavailable", parent / "witness-unavailable"
            )
        self.assertEqual(bounded["schema"], executor.P47_SUMMARY_SCHEMA)
        self.assertEqual(bounded["failure_code"], "P51-SEALED-IDENTITY")

    def test_terminal_rejects_a_symlinked_ancestor_inside_safe_parent(self) -> None:
        parent = self.sandbox / "terminal-parent"
        ancestor = parent / "symlinked-ancestor"
        ancestor.mkdir(parents=True)
        original_lstat = os.lstat

        def lstat(path: str | os.PathLike[str]):
            if Path(path) == ancestor:
                return SimpleNamespace(st_mode=stat.S_IFLNK)
            return original_lstat(path)

        with mock.patch.object(
            executor.Path,
            "resolve",
            autospec=True,
            side_effect=lambda value, strict=False: value,
        ):
            with mock.patch.object(executor.os, "lstat", side_effect=lstat):
                with self.assertRaisesRegex(
                    executor._TerminalPreconditionFailure,
                    "P47-WITNESS-FINAL-PARENT-UNSAFE",
                ):
                    executor._terminal_candidate(parent, ancestor / "p43")

    def test_argv_is_exact_for_launch_ready_and_rejects_no_launch(self) -> None:
        _root, descriptors = validate_descriptor_root(self.descriptor_root)
        argv = profile_diff_argv(descriptors[0])
        SyntheticProcessRunner.assert_exact_argv(tuple(argv))
        with self.assertRaisesRegex(ExecutorFailure, "P51-ARGV-NO-LAUNCH"):
            profile_diff_argv(descriptors[-1])


if __name__ == "__main__":
    unittest.main()
