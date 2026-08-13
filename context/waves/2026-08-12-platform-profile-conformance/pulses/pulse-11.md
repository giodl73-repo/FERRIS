# Pulse 11: Identity and Provider Profile Family

Status: Authorized; implementation and validation pending
Implementation authority: Bounded to this document

## Goal and authority

Complete one synthetic identity/provider family with bounded credential
parsing, secret-redacted output, and explicit provider selection.

Revision `r1` validates `identity:secret` and exposes only the identity.
Revision `r2` adds one caller-selected synthetic provider and a deterministic
challenge result that is explicitly not authentication or cryptography.

This pulse authorizes exact local consumers, positive and rejection tests,
secret non-disclosure checks, explicit identity/credential/provider/TLS
states, owner Cargo workflows, profiles and digests, immutability,
cross-platform validation, and one nine-role review.

It does not authorize real credentials, authentication, authorization, TLS,
cryptography, key storage, network, external providers, security claims,
deployment, support, another family, production generation, or held-out
access.

## Acceptance

- secrets never appear in successful values or diagnostics;
- malformed, oversized, empty-identity, and empty-secret inputs reject;
- `r2` provider selection is explicit and distinct;
- TLS and real cryptographic assurance remain unsupported;
- all owner and repository gates pass on Windows and Unix;
- source and profile digests are stable and distinct; and
- all nine roles accept measured evidence.

## Evidence

- [Authorization review](../../../../docs/plans/reviews/PULSE-11-IDENTITY-PROVIDER-ROLE-REVIEW.md)
