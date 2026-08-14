from __future__ import annotations

import copy
import importlib.util
import io
import json
import os
import shutil
import sys
import unittest
import uuid
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock


sys.dont_write_bytecode = True
RELEASE = Path(__file__).resolve().parents[1]
ROOT = Path(__file__).resolve().parents[5]
P43_RELEASE = (
    RELEASE.parent / "pulse-43-ordered-result-publisher-release" / "ordered_result_publisher.py"
)
sys.path.insert(0, str(RELEASE))
import binary_custody_event_bridge as bridge  # noqa: E402


def load_p43():
    specification = importlib.util.spec_from_file_location("pulse_45_p43_test", P43_RELEASE)
    assert specification is not None and specification.loader is not None
    module = importlib.util.module_from_spec(specification)
    sys.modules[specification.name] = module
    specification.loader.exec_module(module)
    return module


class BinaryCustodyEventBridgeTests(unittest.TestCase):
    def setUp(self) -> None:
        self.runtime = ROOT / "target" / f"pulse-45-python-{uuid.uuid4().hex}"
        self.runtime.mkdir(parents=True)
        self.published = self.fixture("pulse-44-published-summary.json")
        self.failed = self.fixture("pulse-44-failed-summary.json")
        self.calls: list[tuple[object, ...]] = []

    def tearDown(self) -> None:
        if self.runtime.exists() or os.path.lexists(self.runtime):
            shutil.rmtree(self.runtime, ignore_errors=True)

    def fixture(self, name: str) -> dict[str, object]:
        return json.loads((RELEASE / "fixtures" / name).read_text(encoding="utf-8"))

    def invoke(self, platform: str, result: object) -> dict[str, object]:
        def p44(*args: object) -> object:
            self.calls.append(args)
            return copy.deepcopy(result)

        return bridge.bridge_pulse_44(
            ROOT,
            "29517d732db13cc2ffa304684b344f3538ab587d",
            platform,
            self.runtime / "work",
            self.runtime / "final",
            invoker=p44,
        )

    def assert_success(self, result: dict[str, object], platform: str, gate: str) -> None:
        self.assertEqual(result["outcome"], "passed")
        self.assertEqual(result["bridge"]["platform"], platform)
        self.assertEqual(result["bridge"]["invocation_count"], 1)
        self.assertEqual(result["bridge"]["retries"], 0)
        self.assertEqual(
            result["ordered_execution_event"],
            {
                "classification": "ordered-execution",
                "event_kind": "gate-complete",
                "gate_id": gate,
                "outcome": "passed",
                "schema": "ferris.pulse-43-ordered-result-event/v1",
            },
        )

    def assert_p45_failure(self, result: dict[str, object], code: str) -> None:
        self.assertEqual(result["outcome"], "failed")
        self.assertEqual(result["failure_code"], code)
        self.assertEqual(result["ordered_execution_event"]["event_kind"], "terminal-stop")
        self.assertEqual(result["ordered_execution_event"]["outcome"], "failed")
        self.assertNotIn("source_failure", result)

    def test_windows_success_mapping(self) -> None:
        result = self.invoke("windows-x86_64", self.published)
        self.assert_success(result, "windows-x86_64", "windows-retained-binary-custody")
        self.assertEqual(len(self.calls), 1)

    def test_ubuntu_success_mapping(self) -> None:
        result = self.invoke("ubuntu-24.04-x86_64", self.published)
        self.assert_success(result, "ubuntu-24.04-x86_64", "ubuntu-retained-binary-custody")
        self.assertEqual(len(self.calls), 1)

    def test_success_requires_every_final_custody_field(self) -> None:
        mutations = (
            ("final_files_present", False),
            ("work_verified", "1/2"),
            ("stage_verified", "1/2"),
            ("final_verified", "1/2"),
            ("state", "absent"),
            ("rename_attempts", 0),
            ("rename_attempts", True),
            ("retries", 1),
            ("retries", False),
            ("files", "1/2"),
        )
        for name, value in mutations:
            with self.subTest(name=name):
                invalid = copy.deepcopy(self.published)
                invalid["custody"][name] = value
                self.assert_p45_failure(
                    self.invoke("windows-x86_64", invalid), "P45-P44-SUMMARY-MALFORMED"
                )
        invalid_outcome = copy.deepcopy(self.published)
        invalid_outcome["outcome"] = "failed"
        self.assert_p45_failure(
            self.invoke("windows-x86_64", invalid_outcome), "P45-P44-SUMMARY-MALFORMED"
        )

    def test_wrong_pulse_44_terminal_event_is_not_translated(self) -> None:
        invalid = copy.deepcopy(self.published)
        invalid["ordered_execution_event"]["outcome"] = "passed"
        self.assert_p45_failure(
            self.invoke("windows-x86_64", invalid), "P45-P44-SUMMARY-MALFORMED"
        )

    def test_success_requires_coherent_completed_sync_postures(self) -> None:
        mutations = [
            ("stage", "failed", True, "sync-operation-failed", bridge.SYNC_MECHANISM),
            (
                "final_parent",
                "not-attempted",
                False,
                "not-attempted",
                "not-attempted",
            ),
            (
                "rollback_parent",
                "unsupported",
                True,
                "unsupported-by-platform-or-filesystem",
                bridge.SYNC_MECHANISM,
            ),
            ("stage", "synced", False, None, bridge.SYNC_MECHANISM),
        ]
        for field, status, attempted, error_category, mechanism in mutations:
            with self.subTest(field=field, status=status):
                invalid = copy.deepcopy(self.published)
                invalid["custody"]["sync"][field] = {
                    "attempted": attempted,
                    "error_category": error_category,
                    "mechanism": mechanism,
                    "status": status,
                }
                self.assert_p45_failure(
                    self.invoke("windows-x86_64", invalid),
                    "P45-P44-SUMMARY-MALFORMED",
                )

    def test_malformed_and_success_shaped_partial_summaries_fail_closed(self) -> None:
        malformed = (
            {},
            {"outcome": "published"},
            {"unexpected": True, **self.published},
            {
                "custody": self.published["custody"],
                "ordered_execution_event": self.published["ordered_execution_event"],
                "outcome": "published",
                "schema": "ferris.pulse-44-retained-binary-custody-summary/v1",
                "failure_code": "P44-EXTRA",
            },
        )
        for summary in malformed:
            with self.subTest(summary=summary):
                self.assert_p45_failure(
                    self.invoke("ubuntu-24.04-x86_64", summary), "P45-P44-SUMMARY-MALFORMED"
                )

    def test_failure_postures_preserve_pulse_44_failure(self) -> None:
        for state in ("absent", "rolled-back", "indeterminate"):
            with self.subTest(state=state):
                source = copy.deepcopy(self.failed)
                source["custody"]["state"] = state
                if state == "rolled-back":
                    source["custody"]["rename_attempts"] = 1
                    source["custody"]["final_verified"] = "2/2"
                    source["custody"]["stage_verified"] = "2/2"
                    source["custody"]["work_verified"] = "2/2"
                result = self.invoke("windows-x86_64", source)
                self.assertEqual(result["outcome"], "failed")
                self.assertEqual(result["failure_code"], "P44-BUILD-FREEZE-FAILURE")
                self.assertEqual(
                    result["source_failure"],
                    {
                        "custody_state": state,
                        "failure_code": "P44-BUILD-FREEZE-FAILURE",
                    },
                )
                self.assertEqual(result["ordered_execution_event"]["event_kind"], "terminal-stop")
                self.assertEqual(result["ordered_execution_event"]["outcome"], "failed")

    def test_invoker_runs_exactly_once(self) -> None:
        result = self.invoke("windows-x86_64", self.published)
        self.assertEqual(len(self.calls), 1)
        self.assertEqual(
            self.calls[0],
            (
                ROOT,
                "29517d732db13cc2ffa304684b344f3538ab587d",
                "windows-x86_64",
                self.runtime / "work",
                self.runtime / "final",
            ),
        )
        self.assertEqual(result["bridge"]["invocation_count"], 1)

    def test_thrown_invocation_is_a_bounded_terminal_failure(self) -> None:
        def raises(*_: object) -> object:
            raise RuntimeError("do not disclose invocation details")

        result = bridge.bridge_pulse_44(
            ROOT,
            "29517d732db13cc2ffa304684b344f3538ab587d",
            "windows-x86_64",
            self.runtime / "work",
            self.runtime / "final",
            invoker=raises,
        )
        self.assert_p45_failure(result, "P45-P44-INVOCATION-FAILURE")

    def test_invalid_platform_does_not_invoke_pulse_44(self) -> None:
        def unreachable(*_: object) -> object:
            self.fail("Pulse 44 must not be invoked for an unsupported platform")

        with self.assertRaisesRegex(bridge.BridgeFailure, "P45-UNSUPPORTED-PLATFORM"):
            bridge.bridge_pulse_44(
                ROOT,
                "29517d732db13cc2ffa304684b344f3538ab587d",
                "macos-x86_64",
                self.runtime / "work",
                self.runtime / "final",
                invoker=unreachable,
            )

    def test_output_is_path_free(self) -> None:
        result = self.invoke("windows-x86_64", self.published)
        rendered = bridge.canonical_bytes(result).decode("ascii")
        self.assertNotIn(str(ROOT), rendered)
        self.assertNotIn(str(self.runtime), rendered)
        self.assertNotIn("executable", rendered)
        self.assertNotIn("receipt.json", rendered)

    def test_cli_passes_exact_arguments_once_to_real_adapter_boundary(self) -> None:
        calls: list[tuple[object, ...]] = []

        def real_adapter(*args: object) -> object:
            calls.append(args)
            return copy.deepcopy(self.published)

        stdout = io.StringIO()
        with mock.patch.object(bridge, "invoke_real_pulse_44", side_effect=real_adapter):
            with redirect_stdout(stdout):
                status = bridge.main(
                    [
                        "--repo",
                        str(ROOT),
                        "--cutoff",
                        "29517d732db13cc2ffa304684b344f3538ab587d",
                        "--platform",
                        "ubuntu-24.04-x86_64",
                        "--work-root",
                        str(self.runtime / "work"),
                        "--final-root",
                        str(self.runtime / "final"),
                    ]
                )
        self.assertEqual(status, 0)
        self.assertEqual(
            calls,
            [
                (
                    ROOT,
                    "29517d732db13cc2ffa304684b344f3538ab587d",
                    "ubuntu-24.04-x86_64",
                    str(self.runtime / "work"),
                    str(self.runtime / "final"),
                )
            ],
        )
        self.assertEqual(json.loads(stdout.getvalue())["outcome"], "passed")

    def test_pulse_43_can_continue_after_both_platform_gates_but_not_after_failure(self) -> None:
        p43 = load_p43()
        catalog = {
            "schema": "ferris.pulse-43-ordered-gate-catalog/v1",
            "gate_ids": [
                "windows-retained-binary-custody",
                "ubuntu-retained-binary-custody",
                "later-public-gate",
            ],
        }
        windows = self.invoke("windows-x86_64", self.published)
        ubuntu = self.invoke("ubuntu-24.04-x86_64", self.published)
        later = {
            "classification": "ordered-execution",
            "event_kind": "terminal-stop",
            "gate_id": "later-public-gate",
            "outcome": "completed",
            "schema": "ferris.pulse-43-ordered-result-event/v1",
        }
        complete, _ = p43.build_result(
            catalog,
            [
                windows["ordered_execution_event"],
                ubuntu["ordered_execution_event"],
                later,
            ],
        )
        self.assertEqual(complete["ordered_execution"]["completed_gate_count"], 3)
        for platform in ("windows-x86_64", "ubuntu-24.04-x86_64"):
            with self.subTest(platform=platform):
                failure = self.invoke(platform, self.failed)
                preceding = (
                    []
                    if platform == "windows-x86_64"
                    else [windows["ordered_execution_event"]]
                )
                with self.assertRaisesRegex(p43.PublicFailure, "P43-ORDERED-AFTER-TERMINAL"):
                    p43.build_result(
                        catalog,
                        [*preceding, failure["ordered_execution_event"], later],
                    )

    def test_exact_pulse_44_release_identity_is_loadable_before_invocation(self) -> None:
        self.assertTrue(callable(bridge._load_exact_pulse_44_invoker()))


if __name__ == "__main__":
    unittest.main()
