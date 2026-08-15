from __future__ import annotations

import copy
import inspect
import os
import sys
import threading
import unittest
import uuid
from pathlib import Path
from unittest import mock


RELEASE = Path(__file__).resolve().parents[1]
sys.path.insert(0, os.fspath(RELEASE))
import retained_build_custody as p56  # noqa: E402


class _LazyArguments:
    def __iter__(self) -> object:
        raise AssertionError("launch must reject lazy iterables before side effects")


class RetainedBuildCustodyTests(unittest.TestCase):
    def setUp(self) -> None:
        parent = Path.home() if sys.platform == "linux" else RELEASE.parents[3]
        self.work = parent / f".ferris-p56-test-{uuid.uuid4().hex}"
        self.work.mkdir()
        self.addCleanup(lambda: p56._remove_tree(self.work) if self.work.exists() else None)

    def _tool(self) -> dict[str, object]:
        digest = "sha256:" + "1" * 64
        return {
            "command_identity_sha256": digest,
            "file_sha256": digest,
            "path_role": "absolute-path-verified-privately",
            "size": 1,
        }

    def _identity(self, platform: str) -> dict[str, object]:
        digest = "sha256:" + "1" * 64
        tool = self._tool()
        linker: dict[str, object]
        route: str
        if platform == "windows-x86_64":
            route = "rust-toolchain-shipped-rust-lld"
            linker = {
                "driver": tool,
                "path_under_sysroot": "lib/rustlib/x86_64-pc-windows-msvc/bin/rust-lld.exe",
                "route": route,
            }
        else:
            route = "bound-ubuntu-cc-collect2-gnu-ld-trace"
            linker = {
                "actual_trace_selected_inputs": [
                    {"identity": {"file_sha256": digest, "size": 1}, "role": "actual-ld-trace-selected-input"}
                    for _ in range(3)
                ],
                "cc_driver": tool,
                "collect2": tool,
                "gnu_ld": tool,
                "gnu_ld_search_identity_sha256": digest,
                "route": route,
                "startup_objects": [{"identity": tool, "name": name} for name in ("Scrt1.o", "crti.o", "crtbeginS.o", "crtendS.o", "crtn.o")],
            }
        return {
            "checkout": {
                "core_autocrlf": False,
                "exact_commit": p56.CUTOFF,
                "fresh_clean_checkout": True,
            },
            "controls": {
                "cargo_incremental": "0",
                "command": ["cargo", "build", "--release", "--locked", "--package", "ferris-cli", "--bin", "ferris", "--message-format=json"],
                "linker_route": route,
                "linker_rustflags": ["--remap-path-prefix=$CHECKOUT=/ferris", "-C", "synthetic"],
                "remap_path_prefix": "$CHECKOUT=/ferris",
            },
            "git": {"binary": tool, "version_sha256": digest},
            "toolchain": {
                "cargo_direct": tool,
                "environment": {
                    "algorithm": "sha256-name-value-v1",
                    "values": [{"name": "RUSTUP_TOOLCHAIN", "value_sha256": digest, "value_utf8_bytes": 1}],
                },
                "host": "synthetic-host",
                "linker": linker,
                "rustc_direct": tool,
                "rustup_selector": tool,
                "selected_toolchain": "synthetic",
                "sysroot_target_libdir": "lib/rustlib/synthetic/lib",
                "target_libdir_tree": {"aggregate": digest, "file_count": 1, "total_bytes": 1},
            },
        }

    def _pair(self, root: Path, platform: str = "windows-x86_64") -> tuple[p56.Binding, p56.Binding]:
        root.mkdir()
        suffix = ".exe" if platform == "windows-x86_64" else ""
        content = b"not an executable; synthetic evidence only\n"
        artifact = p56.Binding(f"ferris-{platform}-{p56.CUTOFF}{suffix}", len(content), p56.sha256_bytes(content))
        (root / artifact.path).write_bytes(content)
        receipt_bytes, receipt, _ = p56._receipt(platform, artifact, self._identity(platform), self._identity(platform))
        (root / receipt.path).write_bytes(receipt_bytes)
        return artifact, receipt

    def _synthetic_program(self) -> tuple[bytes, list[str], bytes]:
        if os.name == "nt":
            source = Path(os.environ["ComSpec"])
            return source.read_bytes(), ["/d", "/c", "exit", "/b", "0"], b""
        source = Path("/bin/sh")
        return (
            source.read_bytes(),
            ["-c", "printf '%s|%s' \"$1\" \"$2\"", "synthetic", "profile-diff", "--format=json"],
            b"profile-diff|--format=json",
        )

    def _synthetic_handle(self, uses: int = 1) -> tuple[p56.CustodyHandle, bytes]:
        content, arguments, expected = self._synthetic_program()
        handle = p56._test_only_register_synthetic_handle(p56._host_platform(), self.work, content, uses)
        self.addCleanup(lambda: p56._LIVE_HANDLES.pop(handle, None))
        return handle, expected

    def test_receipt_semantically_binds_retained_pair_as_evidence(self) -> None:
        root = self.work / "pair"
        artifact, receipt = self._pair(root)
        observed_artifact, observed_receipt, document = p56._verify_receipt_and_pair(root, "windows-x86_64")
        self.assertEqual((observed_artifact, observed_receipt), (artifact, receipt))
        self.assertTrue(document["payload"]["safety"]["public_receipt_is_evidence_only"])
        evidence = p56.verify_custody(root, "windows-x86_64")
        self.assertFalse(evidence["receipt_authorizes_launch"])

    def test_forged_or_recomputed_public_receipt_cannot_create_handle(self) -> None:
        root = self.work / "pair"
        self._pair(root)
        evidence = p56.verify_custody(root, "windows-x86_64")
        self.assertIn("receipt_id", evidence)
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-FORGERY"):
            p56.launch_verified(root, p56._host_platform(), [])  # type: ignore[arg-type]
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-FORGERY"):
            p56.CustodyHandle(object(), b"x" * 32)

    def test_rejects_replayed_receipt_extra_file_and_binary_replacement(self) -> None:
        root = self.work / "pair"
        artifact, receipt = self._pair(root)
        document = p56._parse_receipt_bytes((root / receipt.path).read_bytes(), "test")
        document["payload"]["artifact"]["retained"] = False  # type: ignore[index]
        (root / receipt.path).write_bytes(p56.canonical_bytes(document) + b"\n")
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-CUSTODY-VERIFY"):
            p56.verify_custody(root, "windows-x86_64")
        self._pair(root / "replacement")
        (root / receipt.path).unlink()
        _, receipt = self._pair(root / "fresh")
        (root / artifact.path).write_bytes(b"replacement")
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-CUSTODY-VERIFY"):
            p56.verify_custody(root, "windows-x86_64")

    def test_receipt_hash_and_parse_use_one_receipt_descriptor(self) -> None:
        root = self.work / "pair"
        _, receipt = self._pair(root)
        original_open = os.open
        opens = 0

        def counting_open(path: object, flags: int, *args: object) -> int:
            nonlocal opens
            if os.fspath(path) == os.fspath(root / receipt.path):
                opens += 1
            return original_open(path, flags, *args)  # type: ignore[arg-type]

        with mock.patch.object(p56.os, "open", side_effect=counting_open):
            p56.verify_custody(root, "windows-x86_64")
        self.assertEqual(opens, 1)

    def test_transaction_publishes_exactly_two_files_with_one_rename(self) -> None:
        work = self.work / "work"
        work.mkdir()
        source = self.work / "source"
        artifact, receipt = self._pair(source)
        artifact_bytes = (source / artifact.path).read_bytes()
        receipt_bytes = (source / receipt.path).read_bytes()
        final = self.work / "final"
        summary = p56._publish_two_file_custody(
            work, final, "windows-x86_64", artifact_bytes, artifact, receipt_bytes, receipt
        )
        self.assertFalse(work.exists())
        self.assertEqual(sorted(path.name for path in final.iterdir()), [artifact.path, receipt.path])
        self.assertEqual(summary["custody"]["rename_attempts"], 1)
        self.assertNotIn(str(final), p56.canonical_bytes(summary).decode("ascii"))

    def test_live_handle_rejects_copies_and_dataclass_forgery(self) -> None:
        handle, _ = self._synthetic_handle()
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-FORGERY"):
            copy.copy(handle)
        forged = object.__new__(p56.CustodyHandle)
        object.__setattr__(forged, "_CustodyHandle__token", handle._CustodyHandle__token)
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-EXPIRED"):
            p56.launch_verified(forged, p56._host_platform(), [])

    def test_lazy_arguments_are_rejected_before_launch_side_effects(self) -> None:
        handle, _ = self._synthetic_handle(uses=2)
        record = p56._LIVE_HANDLES[handle]
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-LAUNCH-ARGUMENTS"):
            p56.launch_verified(handle, p56._host_platform(), _LazyArguments())  # type: ignore[arg-type]
        self.assertEqual(handle.remaining_uses, 2)
        self.assertEqual(list(record.launch_parent.iterdir()), [])

    def test_child_environment_drops_injected_loader_and_python_values(self) -> None:
        handle, _ = self._synthetic_handle()
        record = p56._LIVE_HANDLES[handle]
        injected = {"LD_PRELOAD": "evil.so", "PYTHONPATH": "evil", "PYTHONHOME": "evil", "RUSTC_WRAPPER": "evil"}
        with mock.patch.dict(os.environ, injected, clear=False):
            environment = p56._child_environment(record)
        self.assertFalse(set(injected) & set(environment))
        self.assertEqual(environment["HOME"], os.fspath(record.runtime_root / "home"))

    def test_live_handle_decrements_before_each_launch_and_expires(self) -> None:
        handle, expected = self._synthetic_handle(uses=2)
        record = p56._LIVE_HANDLES[handle]
        _content, arguments, _expected = self._synthetic_program()
        first = p56.launch_verified(handle, p56._host_platform(), arguments)
        self.assertEqual(first.returncode, 0)
        self.assertEqual(first.stdout, expected)
        self.assertEqual(handle.remaining_uses, 1)
        second = p56.launch_verified(handle, p56._host_platform(), arguments)
        self.assertEqual(second.returncode, 0)
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-EXPIRED"):
            _ = handle.remaining_uses
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-EXPIRED"):
            p56.launch_verified(handle, p56._host_platform(), arguments)
        self.assertFalse(record.runtime_root.exists())
        self.assertFalse(record.custody_root.exists())

    def test_close_custody_invalidates_early_and_cleans_exact_owned_roots(self) -> None:
        handle, _ = self._synthetic_handle(uses=2)
        record = p56._LIVE_HANDLES[handle]
        p56.close_custody(handle)
        self.assertFalse(record.runtime_root.exists())
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-EXPIRED"):
            p56.close_custody(handle)
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-EXPIRED"):
            p56.launch_verified(handle, p56._host_platform(), [])

    def test_close_custody_refuses_while_a_launch_is_active(self) -> None:
        handle, _ = self._synthetic_handle()
        _content, arguments, _expected = self._synthetic_program()
        started = threading.Event()
        release = threading.Event()
        outcomes: list[object] = []
        test_case = self

        class BlockingProcess:
            returncode = 0

            def communicate(self) -> tuple[bytes, bytes]:
                started.set()
                test_case.assertTrue(release.wait(5))
                return b"", b""

        def launch() -> None:
            try:
                outcomes.append(p56.launch_verified(handle, p56._host_platform(), arguments))
            except BaseException as error:  # pragma: no cover - asserted below
                outcomes.append(error)

        with mock.patch.object(p56.subprocess, "Popen", return_value=BlockingProcess()):
            thread = threading.Thread(target=launch)
            thread.start()
            self.assertTrue(started.wait(5))
            with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-ACTIVE"):
                p56.close_custody(handle)
            release.set()
            thread.join(5)
        self.assertFalse(thread.is_alive())
        self.assertEqual(len(outcomes), 1)
        self.assertIsInstance(outcomes[0], p56.subprocess.CompletedProcess)
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-EXPIRED"):
            p56.close_custody(handle)

    def test_concurrent_last_uses_clean_runtime_exactly_once(self) -> None:
        handle, _ = self._synthetic_handle(uses=2)
        record = p56._LIVE_HANDLES[handle]
        _content, arguments, _expected = self._synthetic_program()
        started = 0
        started_lock = threading.Lock()
        both_started = threading.Event()
        release = threading.Event()
        outcomes: list[object] = []

        class BlockingProcess:
            returncode = 0

            def communicate(self) -> tuple[bytes, bytes]:
                nonlocal started
                with started_lock:
                    started += 1
                    if started == 2:
                        both_started.set()
                if not release.wait(5):
                    raise AssertionError("concurrent launch did not release")
                return b"", b""

        def launch() -> None:
            try:
                outcomes.append(p56.launch_verified(handle, p56._host_platform(), arguments))
            except BaseException as error:  # pragma: no cover - asserted below
                outcomes.append(error)

        with (
            mock.patch.object(p56.subprocess, "Popen", side_effect=lambda *args, **kwargs: BlockingProcess()),
            mock.patch.object(p56, "_clean_live_custody", wraps=p56._clean_live_custody) as cleanup,
        ):
            threads = [threading.Thread(target=launch) for _ in range(2)]
            for thread in threads:
                thread.start()
            self.assertTrue(both_started.wait(5))
            release.set()
            for thread in threads:
                thread.join(5)
                self.assertFalse(thread.is_alive())
        self.assertEqual(len(outcomes), 2)
        self.assertTrue(all(isinstance(item, p56.subprocess.CompletedProcess) for item in outcomes))
        self.assertEqual(cleanup.call_count, 1)
        self.assertFalse(record.runtime_root.exists())

    def test_cleanup_failure_after_completion_is_fatal_and_retires_handle(self) -> None:
        handle, _ = self._synthetic_handle()
        record = p56._LIVE_HANDLES[handle]
        _content, arguments, _expected = self._synthetic_program()
        with mock.patch.object(p56, "_remove_owned_tree", side_effect=OSError("synthetic cleanup failure")):
            with self.assertRaisesRegex(p56.ReleaseFailure, "P56-INDETERMINATE-CLEANUP"):
                p56.launch_verified(handle, p56._host_platform(), arguments)
        self.assertTrue(record.runtime_root.exists())
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-EXPIRED"):
            _ = handle.remaining_uses

    def test_substituted_runtime_root_is_refused_and_handle_is_retired(self) -> None:
        handle, _ = self._synthetic_handle()
        record = p56._LIVE_HANDLES[handle]
        p56._remove_tree(record.runtime_root)
        record.runtime_root.mkdir()
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-INDETERMINATE-CLEANUP"):
            p56.close_custody(handle)
        self.assertTrue(record.runtime_root.exists())
        with self.assertRaisesRegex(p56.ReleaseFailure, "P56-HANDLE-EXPIRED"):
            p56.launch_verified(handle, p56._host_platform(), [])

    @unittest.skipUnless(sys.platform == "linux", "Linux descriptor execution control")
    def test_mutating_launch_path_after_open_cannot_change_executed_inode(self) -> None:
        handle, expected = self._synthetic_handle()
        _content, arguments, _expected = self._synthetic_program()
        original_open = p56._open_linux_verified_image

        def mutate_after_open(path: Path, binding: p56.Binding, content: bytes) -> tuple[int, str]:
            descriptor, executable_fd = original_open(path, binding, content)
            path.unlink()
            path.write_bytes(b"not an executable")
            return descriptor, executable_fd

        with mock.patch.object(p56, "_open_linux_verified_image", side_effect=mutate_after_open):
            result = p56.launch_verified(handle, p56._host_platform(), arguments)
        self.assertEqual(result.returncode, 0)
        self.assertEqual(result.stdout, expected)

    def test_production_apis_have_no_root_to_launch_or_callback_seam(self) -> None:
        self.assertEqual(
            set(inspect.signature(p56.publish_retained_build_and_custody).parameters),
            {"platform", "runtime_parent"},
        )
        self.assertEqual(
            set(inspect.signature(p56.launch_verified).parameters),
            {"handle", "platform", "arguments"},
        )
        self.assertEqual(set(inspect.signature(p56.close_custody).parameters), {"handle"})
        source = inspect.getsource(p56)
        self.assertNotIn("prepare_verified_launch", source)
        self.assertNotIn("runner", inspect.signature(p56.launch_verified).parameters)
        self.assertIn("close_custody(handle)", (RELEASE / "README.md").read_text(encoding="utf-8"))

    def test_windows_os_handle_transfers_once_to_open_osfhandle(self) -> None:
        source = inspect.getsource(p56._open_windows_verified_image)
        transfer = source.index("descriptor = msvcrt.open_osfhandle")
        self.assertNotIn("CloseHandle(handle)", source[transfer:])


if __name__ == "__main__":
    unittest.main()
