# Pulse 12: Assurance, Packaging, and Deployment Profile Family

Status: Authorized; implementation and validation pending
Implementation authority: Bounded to this document

## Goal and authority

Complete the ninth family with deterministic release records plus actual Cargo
package construction. Revision `r1` inventories one package artifact.
Revision `r2` adds an explicit deployment channel, prior identity, and exact
rollback identity while retaining signing and deployment as unavailable.

This pulse authorizes exact consumers, positive/rejection tests, `cargo
package`, package-content inspection, assurance and lifecycle records,
profiles/digests, immutability, Windows/Unix validation, and role review.

It does not authorize signing, attestation, installation, deployment,
credentials, remote systems, approval, support, production generation,
another family, or held-out access.

## Acceptance

- package construction and expected contents are directly observed;
- `r2` requires distinct current, prior, and rollback identities;
- invalid channels and equal current/prior identities reject;
- signing, deployment, and operations remain unavailable;
- all owner and repository gates pass on Windows and Unix;
- source and profile digests are stable and distinct; and
- all nine roles accept measured evidence.

## Evidence

- [Authorization review](../../../../docs/plans/reviews/PULSE-12-ASSURANCE-DEPLOYMENT-ROLE-REVIEW.md)
