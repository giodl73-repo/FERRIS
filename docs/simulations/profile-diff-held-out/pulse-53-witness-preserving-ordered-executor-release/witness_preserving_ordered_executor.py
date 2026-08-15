"""Pulse 53 witness-preserving ordered executor.

This sealed replacement reuses exact Pulse 52 custody, gate, materialization,
and dispatch helpers.  It changes only the one-use terminal classification so
that a verified Pulse 47 witness of an exact failed Pulse 43 publication is a
permanent public closeout artifact rather than disposable residue.
"""

from __future__ import annotations

import os
import re
import stat
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Mapping

from sealed_dependencies import SealedDependencyFailure, load_pulse52


P50_GATE_IDS = (
    "pulse-41-pulse-39-public-custody",
    "windows-retained-binary-custody",
    "ubuntu-retained-binary-custody",
    "exact-adapter-preflight",
    "pulse-31-public-input",
    "pulse-35-pulse-37-normalization",
    "bounded-materialization",
    "bounded-process-exit-search",
)
P43_CATALOG_SCHEMA = "ferris.pulse-43-ordered-gate-catalog/v1"
P43_EVENT_SCHEMA = "ferris.pulse-43-ordered-result-event/v1"
PRIVATE_LAUNCH_DIRECTORY = ".pulse52-private-launch"
TERMINAL_DIRECTORY = ".pulse52-terminal-publication"
P43_FINAL_DIRECTORY = "pulse-52-p43-result"
WITNESS_FINAL_DIRECTORY = "pulse-52-p47-witness"
TERMINAL_CLEANUP_DELAYS = (0.02, 0.05, 0.10, 0.20)
TERMINAL_CLEANUP_FATAL_SCHEMA = (
    "ferris.pulse-53-terminal-publication-cleanup-indeterminate/v1"
)
TRANSFER_DESCRIPTOR_SCHEMA = "ferris.pulse-53-public-transfer-descriptor/v1"


@dataclass(frozen=True)
class WitnessPreservingOrderedResult:
    """P43-safe events, terminal disposition, transfer descriptor, and private record."""

    catalog: dict[str, object]
    events: list[dict[str, object]]
    publication: dict[str, object]
    transfer_descriptor: dict[str, object] | None
    private_record: dict[str, object]


class TerminalPublicationCleanupIndeterminate(RuntimeError):
    """A non-returning, public-safe terminal cleanup failure."""

    code = "terminal-publication-cleanup-indeterminate"

    def __init__(self) -> None:
        self.public_posture = {
            "schema": TERMINAL_CLEANUP_FATAL_SCHEMA,
            "state": self.code,
            "cleanup_owner": "caller-public-custodian",
            "cleanup_posture": "unresolved",
        }
        super().__init__(self.code)


def _private_record(p52: object) -> dict[str, object]:
    record = p52._private_record()
    record["schema"] = "ferris.pulse-53-private-execution-record/v1"
    record["terminal_transfer"] = "not-created"
    return record


def _catalog(p52: object | None) -> dict[str, object]:
    if p52 is not None:
        return p52._catalog()
    return {"schema": P43_CATALOG_SCHEMA, "gate_ids": list(P50_GATE_IDS)}


def _terminal_publication(disposition: str, posture: dict[str, object]) -> dict[str, object]:
    return {
        "schema": "ferris.pulse-53-terminal-publication-disposition/v1",
        "disposition": disposition,
        "product_conclusion": None,
        "category_conclusion": None,
        "fix_conclusion": None,
        "posture": posture,
    }


def _not_attempted_publication() -> dict[str, object]:
    return _terminal_publication("not-attempted", {"state": "not-attempted"})


def _terminal_result(
    p52: object | None,
    p43: object | None,
    events: list[dict[str, object]],
    gate: str,
    code: str,
    private_record: dict[str, object],
) -> WitnessPreservingOrderedResult:
    event = (
        p52._event(gate, "terminal-stop", "failed")
        if p52 is not None
        else {
            "classification": "ordered-execution",
            "event_kind": "terminal-stop",
            "gate_id": gate,
            "outcome": "failed",
            "schema": P43_EVENT_SCHEMA,
        }
    )
    events.append(event)
    catalog = _catalog(p52)
    if p43 is not None:
        p43.validate_catalog(catalog)
        p43.validate_events(P50_GATE_IDS, events)
    private_record["failure_code"] = code
    private_record["outcome"] = "failed"
    private_record["execution_outcome"] = "failed"
    private_record["terminal_transfer"] = "not-created"
    return WitnessPreservingOrderedResult(
        catalog, events, _not_attempted_publication(), None, private_record
    )


def _hashes(value: object, keys: frozenset[str], p52: object) -> dict[str, str] | None:
    if type(value) is not dict or set(value) != set(keys):
        return None
    if not all(p52._digest(digest) for digest in value.values()):
        return None
    return {key: value[key] for key in sorted(keys)}


def _terminal_parent_shape(parent: Path, expected: frozenset[str]) -> bool:
    try:
        metadata = os.lstat(parent)
        if stat.S_ISLNK(metadata.st_mode) or not stat.S_ISDIR(metadata.st_mode):
            return False
        with os.scandir(parent) as entries:
            names = frozenset(entry.name for entry in entries)
    except OSError:
        return False
    return names == expected


def _stage_root(root: Path, pulse: str) -> Path:
    return root.parent / f".{root.name}.{pulse}-stage"


def _transfer_descriptor(
    kind: str,
    *,
    result_hashes: dict[str, str] | None,
    witness_hashes: dict[str, str],
) -> dict[str, object]:
    counts: dict[str, int] = {"witness": 2, "total": 2}
    hashes: dict[str, dict[str, str]] = {"witness": dict(witness_hashes)}
    if result_hashes is not None:
        counts = {"result": 2, "witness": 2, "total": 4}
        hashes = {"result": dict(result_hashes), "witness": dict(witness_hashes)}
    return {
        "schema": TRANSFER_DESCRIPTOR_SCHEMA,
        "expected_public_tree_kind": kind,
        "exact_file_counts": counts,
        "verified_raw_payload_hashes": hashes,
    }


def _published_result_descriptor(
    p52: object,
    p43: object,
    p47: object,
    summary: object,
    terminal_parent: Path,
    p43_root: Path,
    witness_root: Path,
) -> dict[str, object] | None:
    """Reuse P52's full published-shape check, then build a path-free transfer view."""

    if not p52._published_terminal_summary(p43, p47, summary, p43_root, witness_root):
        return None
    if not _terminal_parent_shape(
        terminal_parent, frozenset({p43_root.name, witness_root.name})
    ):
        return None
    try:
        result_hashes = _hashes(
            p43.verify_publication_directory(p43_root),
            frozenset(
                {
                    "receipt_payload_sha256",
                    "receipt_raw_sha256",
                    "result_payload_sha256",
                    "result_raw_sha256",
                }
            ),
            p52,
        )
        witness_hashes = _hashes(
            p47.verify_witness_directory(witness_root),
            frozenset(
                {
                    "receipt_payload_sha256",
                    "receipt_raw_sha256",
                    "witness_payload_sha256",
                    "witness_raw_sha256",
                }
            ),
            p52,
        )
    except (p43.PublicFailure, p47.WitnessFailure, OSError):
        return None
    if result_hashes is None or witness_hashes is None:
        return None
    return _transfer_descriptor(
        "result-and-witness",
        result_hashes=result_hashes,
        witness_hashes=witness_hashes,
    )


def _failure_witness_descriptor(
    p52: object,
    p43: object,
    p47: object,
    summary: object,
    terminal_parent: Path,
    p43_root: Path,
    witness_root: Path,
) -> tuple[dict[str, object], dict[str, object]] | None:
    """Accept only a verified P47 witness of P43's bounded failed posture."""

    posture = p52._p47_failure_posture(p47, summary)
    if posture.get("source") != "pulse-43":
        return None
    if (
        os.path.lexists(p43_root)
        or os.path.lexists(_stage_root(p43_root, "pulse-43"))
        or os.path.lexists(_stage_root(witness_root, "pulse-47"))
        or not _terminal_parent_shape(terminal_parent, frozenset({witness_root.name}))
    ):
        return None
    if type(summary) is not dict:
        return None
    witness = p52._published_witness_posture(summary.get("witness_publication"))
    if witness is None:
        return None
    expected_hashes = _hashes(
        summary.get("witness_publication", {}).get("raw_hashes")
        if type(summary.get("witness_publication")) is dict
        else None,
        frozenset(
            {
                "receipt_payload_sha256",
                "receipt_raw_sha256",
                "witness_payload_sha256",
                "witness_raw_sha256",
            }
        ),
        p52,
    )
    if expected_hashes is None:
        return None
    try:
        witness_hashes = _hashes(
            p47.verify_witness_directory(witness_root),
            frozenset(
                {
                    "receipt_payload_sha256",
                    "receipt_raw_sha256",
                    "witness_payload_sha256",
                    "witness_raw_sha256",
                }
            ),
            p52,
        )
    except (p47.WitnessFailure, OSError):
        return None
    if witness_hashes is None or witness_hashes != expected_hashes:
        return None
    descriptor = _transfer_descriptor(
        "failure-witness-only", result_hashes=None, witness_hashes=witness_hashes
    )
    return posture, descriptor


def _invalid_witness_posture(p52: object, p47: object, summary: object) -> dict[str, object]:
    """Keep P52's bounded P43/P47 vocabulary without treating a witness as success."""

    return p52._p47_failure_posture(p47, summary)

def _run_loaded(
    p52: object,
    p51: object,
    p39: object,
    p41: object,
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    retained_custodies: Mapping[str, object],
    controls: _QualificationControls | None,
) -> WitnessPreservingOrderedResult:
    P50_GATE_IDS = p52.P50_GATE_IDS
    CANONICAL_PLATFORMS = p52.CANONICAL_PLATFORMS
    DESCRIPTOR_DIRECTORY = p52.DESCRIPTOR_DIRECTORY
    _catalog = p52._catalog
    _event = p52._event
    SealedDependencyFailure = p52.SealedDependencyFailure
    load_p35_materializer_and_verifier = p52.load_p35_materializer_and_verifier
    _failure_code = p52._failure_code
    _raise = p52._raise
    _audit_only_public_custody = p52._audit_only_public_custody
    _assert_private_namespace_absent = p52._assert_private_namespace_absent
    _verify_public_prelaunch_custody = p52._verify_public_prelaunch_custody
    _begin_private_launch = p52._begin_private_launch
    _write_seed_atomically = p52._write_seed_atomically
    _validate_materialization_summary = p52._validate_materialization_summary
    _validate_verification_summary = p52._validate_verification_summary
    _remove_seed = p52._remove_seed
    _validate_materialized_descriptor_root = p52._validate_materialized_descriptor_root
    _cleanup_private_launch = p52._cleanup_private_launch
    _prepare_terminal = p52._prepare_terminal
    _cleanup_terminal_publication = p52._cleanup_terminal_publication
    secrets = p52.secrets

    events: list[dict[str, object]] = []
    private_record = _private_record(p52)
    current_gate = P50_GATE_IDS[0]
    p43: object | None = None
    p47: object | None = None
    namespace: Path | None = None
    private_started = False
    failure: str | None = None
    try:
        p43, p45, p47 = p51.load_terminal_dependencies(repo_root)
        runtime_root = p51._safe_runtime_root(private_runtime_root)
        if p45.PLATFORM_GATES != {
            "windows-x86_64": P50_GATE_IDS[1],
            "ubuntu-24.04-x86_64": P50_GATE_IDS[2],
        }:
            _raise(p51, "P52-P45-PLATFORM-BINDING")
        custodies = p51._normalize_custodies(retained_custodies)
        _audit_only_public_custody(p51, runtime_root, custodies, p27_cycle_root)
        expectations = p51.P33_EXPECTATIONS if controls is None else controls.expectations
        if set(expectations) != set(CANONICAL_PLATFORMS):
            _raise(p51, "P52-P33-BINARY-PLATFORM")
        executable_by_platform = {
            platform: p51._verify_custody_binary(
                custodies[platform], expectations[platform], runtime_root
            )
            for platform in CANONICAL_PLATFORMS
        }

        _assert_private_namespace_absent(p51, runtime_root, private_record)
        _verify_public_prelaunch_custody(
            p51,
            p39,
            p41,
            runtime_root,
            p39_checkout_root,
            p41_final_root,
            private_record,
        )
        events.append(p51._validation_event("public-catalog-prevalidation", 5))
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[1]
        _assert_private_namespace_absent(p51, runtime_root, private_record)
        p51._bridge_p44_once(
            p45, repo_root, custodies["windows-x86_64"], "windows-x86_64", runtime_root
        )
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[2]
        _assert_private_namespace_absent(p51, runtime_root, private_record)
        p51._bridge_p44_once(
            p45,
            repo_root,
            custodies["ubuntu-24.04-x86_64"],
            "ubuntu-24.04-x86_64",
            runtime_root,
        )
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[3]
        _assert_private_namespace_absent(p51, runtime_root, private_record)
        p51._run_p27_once(
            runtime_root, p27_cycle_root, p51.load_p27_exact_runner(repo_root)
        )
        private_record["p27_cycle_retention"] = "retained-private-cycle-root"
        private_record["p27_invocations"] = 1
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[4]
        _assert_private_namespace_absent(p51, runtime_root, private_record)
        p31_summary = p51.verify_bound_contract(repo_root)
        if p31_summary != {
            "artifact_count": 9,
            "positive_fixture_count": 6,
            "mutation_control_count": 33,
            "public_input_checks": 39,
        }:
            _raise(p51, "P52-P31-CONTROL-COUNT")
        events.append(p51._validation_event("public-input-contract", 39))
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[5]
        _assert_private_namespace_absent(p51, runtime_root, private_record)
        p35_summary = p51.verify_p35_p37_custody(repo_root)
        if p35_summary != {
            "bound_file_count": 11,
            "p35_release_tree_file_count": 10,
            "machine_schema_count": 1,
            "canonical_lf_file_count": 11,
            "git_clean_checks": 11,
        }:
            _raise(p51, "P52-P35-CUSTODY-COUNT")
        materializer, verifier = load_p35_materializer_and_verifier(repo_root)
        _assert_private_namespace_absent(p51, runtime_root, private_record)
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[6]
        namespace = _begin_private_launch(p51, runtime_root)
        private_started = True
        private_record["private_launch_started"] = True
        seed = secrets.token_bytes(32) if controls is None else controls.seed_bytes
        seed_path = _write_seed_atomically(p51, namespace, seed)
        private_record["seed_byte_count"] = 32
        descriptor_root = namespace / DESCRIPTOR_DIRECTORY
        if controls is not None and controls.force_materializer_destination_conflict:
            try:
                conflict = os.open(
                    descriptor_root,
                    os.O_WRONLY | os.O_CREAT | os.O_EXCL | getattr(os, "O_BINARY", 0),
                    stat.S_IRUSR | stat.S_IWUSR,
                )
                os.fsync(conflict)
                os.close(conflict)
            except OSError as error:
                raise p51.ExecutorFailure("P52-MATERIALIZATION") from error
        private_record["materializer_invocations"] = 1
        try:
            materialization_summary = materializer.materialize(seed_path, descriptor_root)
        except materializer.MaterializationError as error:
            raise p51.ExecutorFailure("P52-MATERIALIZATION") from error
        _validate_materialization_summary(p51, materialization_summary)
        private_record["verifier_invocations"] = 1
        try:
            verification_summary = verifier.verify(descriptor_root, seed_path)
        except verifier.MaterializationError as error:
            raise p51.ExecutorFailure("P52-PRIVATE-VERIFIER") from error
        _validate_verification_summary(p51, verification_summary)
        commitment = materializer.seed_commitment(seed)
        if type(commitment) is not str or not commitment.startswith("sha256:"):
            _raise(p51, "P52-SEED-COMMITMENT")
        private_record["seed_commitment_sha256"] = commitment
        _remove_seed(p51, seed_path, private_record)
        descriptors, projection_variances = _validate_materialized_descriptor_root(
            p51, descriptor_root, runtime_root
        )
        private_record["p35_to_p51_semantic_projection_variance_count"] = projection_variances
        events.append(_event(current_gate, "gate-complete", "passed"))

        current_gate = P50_GATE_IDS[7]
        runner = p51._subprocess_process_runner if controls is None else controls.process_runner
        for descriptor in descriptors:
            if descriptor.execution_mode == "no-launch":
                for platform in CANONICAL_PLATFORMS:
                    private_record["no_launch_records"].append(
                        {
                            "case_id": descriptor.case_id,
                            "ordinal": descriptor.ordinal,
                            "platform": platform,
                            "process_launched": False,
                            "reason": "blocked-no-launch-external-immutable-binary-freeze",
                        }
                    )
                continue
            windows = p51._run_descriptor(
                descriptor,
                "windows-x86_64",
                executable_by_platform["windows-x86_64"],
                runtime_root,
                runner,
            )
            private_record["platform_records"]["windows-x86_64"].append(windows)
            private_record["process_counts"]["windows-x86_64"] += 1
            ubuntu = p51._run_descriptor(
                descriptor,
                "ubuntu-24.04-x86_64",
                executable_by_platform["ubuntu-24.04-x86_64"],
                runtime_root,
                runner,
            )
            private_record["platform_records"]["ubuntu-24.04-x86_64"].append(ubuntu)
            private_record["process_counts"]["ubuntu-24.04-x86_64"] += 1
            if windows["result"]["semantic_projection"] != ubuntu["result"]["semantic_projection"]:
                private_record["first_mismatch_ordinal"] = descriptor.ordinal
                _raise(p51, "P52-FIRST-TARGET-MISMATCH")
        if (
            private_record["process_counts"]
            != {"windows-x86_64": 69, "ubuntu-24.04-x86_64": 69}
            or len(private_record["no_launch_records"]) != 2
        ):
            _raise(p51, "P52-TOPOLOGY-ACCOUNTING")
        _cleanup_private_launch(p51, namespace, private_record)
        terminal, p43_root, witness_root, terminal_parent = _prepare_terminal(
            p51, runtime_root, repo_root
        )
    except (
        p51.ExecutorFailure,
        p51.P31Failure,
        p51.CustodyFailure,
        p51.DependencyFailure,
        SealedDependencyFailure,
        OSError,
        subprocess.SubprocessError,
    ) as error:
        failure = _failure_code(error, "P52-OPERATION")

    if failure is not None:
        if private_started and namespace is not None:
            try:
                _cleanup_private_launch(p51, namespace, private_record)
            except p51.ExecutorFailure as cleanup_error:
                failure = cleanup_error.code
        return _terminal_result(p52, p43, events, current_gate, failure, private_record)

    events.append(_event(P50_GATE_IDS[7], "terminal-stop", "completed"))
    p43.validate_catalog(_catalog())
    p43.validate_events(P50_GATE_IDS, events)
    private_record["execution_outcome"] = "completed"
    publication = _not_attempted_publication()
    result = WitnessPreservingOrderedResult(
        p52._catalog(), events, publication, None, private_record
    )
    private_record["terminal_p47_invocation_count"] = 1
    try:
        terminal_summary = p51.invoke_terminal_pulse47_once(
            terminal, result, p43_root, witness_root
        )
    except (p43.PublicFailure, p47.WitnessFailure):
        terminal_summary = None

    published = _published_result_descriptor(
        p52, p43, p47, terminal_summary, terminal_parent, p43_root, witness_root
    )
    if published is not None:
        publication.update(
            _terminal_publication(
                "published-result",
                {
                    "p43_result": "published-and-verified",
                    "p47_witness": "published-and-verified",
                },
            )
        )
        result = WitnessPreservingOrderedResult(
            result.catalog, result.events, publication, published, private_record
        )
        private_record["outcome"] = "published-result"
        private_record["publication_disposition"] = "published-result"
        private_record["terminal_p47_outcome"] = "published"
        private_record["terminal_publication_cleanup"] = "retained-published-result"
        private_record["terminal_transfer"] = "retained-result-and-witness"
        return result

    witnessed_failure = _failure_witness_descriptor(
        p52, p43, p47, terminal_summary, terminal_parent, p43_root, witness_root
    )
    if witnessed_failure is not None:
        posture, descriptor = witnessed_failure
        publication.update(_terminal_publication("published-failure-witness", posture))
        result = WitnessPreservingOrderedResult(
            result.catalog, result.events, publication, descriptor, private_record
        )
        private_record["outcome"] = "published-failure-witness"
        private_record["publication_disposition"] = "published-failure-witness"
        private_record["terminal_p47_outcome"] = "published"
        private_record["terminal_publication_cleanup"] = "retained-failure-witness"
        private_record["terminal_transfer"] = "retained-failure-witness-only"
        return result

    publication.update(
        _terminal_publication(
            "invalid-witness-publication",
            _invalid_witness_posture(p52, p47, terminal_summary),
        )
    )
    private_record["outcome"] = "invalid-witness-publication"
    private_record["publication_disposition"] = "invalid-witness-publication"
    private_record["terminal_p47_outcome"] = (
        terminal_summary.get("outcome") if type(terminal_summary) is dict else None
    )
    try:
        _cleanup_terminal_publication(
            p51, terminal_parent, p43_root, witness_root, private_record
        )
    except (p51.ExecutorFailure, PermissionError, OSError):
        raise TerminalPublicationCleanupIndeterminate() from None
    return WitnessPreservingOrderedResult(
        result.catalog, result.events, publication, None, private_record
    )


def _run(
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    retained_custodies: Mapping[str, object],
    controls: object | None,
) -> WitnessPreservingOrderedResult:
    try:
        p52, p51 = load_pulse52(repo_root)
    except SealedDependencyFailure as error:
        private_record = {
            "schema": "ferris.pulse-53-private-execution-record/v1",
            "outcome": "in-progress",
            "execution_outcome": "not-started",
            "publication_disposition": "not-attempted",
            "product_conclusion": None,
            "category_conclusion": None,
            "fix_conclusion": None,
            "terminal_transfer": "not-created",
        }
        return _terminal_result(None, None, [], P50_GATE_IDS[0], error.code, private_record)
    try:
        p39, p41 = p52.load_p39_and_p41(repo_root)
    except p52.SealedDependencyFailure as error:
        return _terminal_result(
            p52, None, [], p52.P50_GATE_IDS[0], error.code, _private_record(p52)
        )
    return _run_loaded(
        p52,
        p51,
        p39,
        p41,
        repo_root,
        private_runtime_root,
        p27_cycle_root,
        p39_checkout_root,
        p41_final_root,
        retained_custodies,
        controls,
    )


def run_witness_preserving_ordered_executor(
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    retained_custodies: Mapping[str, object],
) -> WitnessPreservingOrderedResult:
    """Run exact Pulse 52 phases with no production injection surface.

    The caller supplies only the same concrete public custody inputs accepted
    by Pulse 52.  It cannot supply a terminal callback, roots, seed,
    materializer, verifier, launcher, fake binary, expectations, or trust
    control.  Pulse 53 internally binds exact Pulse 51 and Pulse 52 before
    reuse of the sealed ordering engine.
    """

    return _run(
        repo_root,
        private_runtime_root,
        p27_cycle_root,
        p39_checkout_root,
        p41_final_root,
        retained_custodies,
        None,
    )


def _run_qualification_executor(
    repo_root: Path,
    private_runtime_root: Path,
    p27_cycle_root: Path,
    p39_checkout_root: Path,
    p41_final_root: Path,
    retained_custodies: Mapping[str, object],
    *,
    seed_bytes: bytes,
    process_runner: Callable[[object], object],
    expectations: Mapping[str, object],
    force_materializer_destination_conflict: bool = False,
) -> WitnessPreservingOrderedResult:
    """Private fake-only qualification seam; never a production API."""

    p52, _p51 = load_pulse52(repo_root)
    controls = p52._QualificationControls(
        seed_bytes,
        process_runner,
        expectations,
        force_materializer_destination_conflict,
    )
    return _run(
        repo_root,
        private_runtime_root,
        p27_cycle_root,
        p39_checkout_root,
        p41_final_root,
        retained_custodies,
        controls,
    )


__all__ = [
    "TerminalPublicationCleanupIndeterminate",
    "WitnessPreservingOrderedResult",
    "run_witness_preserving_ordered_executor",
]
