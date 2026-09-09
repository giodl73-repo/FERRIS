# Pulse 01: Exact Local Revision Binding

Status: authorized

## Authority

Implement only the bounded V1 in
[`FERRIS_REVISION_BOUND_VALIDATION_EVIDENCE_PLAN.md`](../../../../docs/plans/FERRIS_REVISION_BOUND_VALIDATION_EVIDENCE_PLAN.md).

## Allowed production behavior

- accept one atomic base/head/tested revision triple on `validation-plan`;
- resolve local Git commit identities without network access;
- require the Git root to equal the Cargo workspace root;
- require current `HEAD` to equal the tested revision;
- require the tested revision to equal or contain the head;
- derive merge-base-to-head changed and deleted paths with bounded Git output;
- bind the resolved revisions, normalized change set, and plan identity;
- emit deterministic structured failures.

## Forbidden behavior

- fetching, checkout mutation, or remote API access;
- parsing workflow files;
- declaring or running owner commands;
- recording owner execution outcomes;
- signing or claiming an attestation;
- changing required checks or narrowing existing CI;
- extending the slice to federated workspaces or non-Git source control.

## Proof

The pulse is complete only after focused Rust/schema/CLI tests, BISECT local and
hosted adoption, and clean required role review. The budget is then consumed;
only corrective fixes and closeout remain authorized.
