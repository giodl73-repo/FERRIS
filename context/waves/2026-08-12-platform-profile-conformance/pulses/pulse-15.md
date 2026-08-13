# Pulse 15: Substitution and Emergency Response

Status: Authorized; implementation and validation pending
Implementation authority: Bounded to this document

## Goal and authority

Execute a typed synthetic-provider substitution from Alpha to Beta, enter an
emergency contained state, reject use while contained, and roll back exactly
to Alpha.

This pulse authorizes only test-local state transitions and invalid-transition
controls. It does not authorize real providers, credentials, cryptography,
network, production emergency control, deployment, or support.

## Acceptance

- substitution retains prior and active identities;
- emergency containment rejects provider use;
- rollback restores the exact prior identity;
- rollback outside emergency and repeated emergency entry reject; and
- Windows and Unix repository gates pass.

## Evidence

- [Nine-role review](../../../../docs/plans/reviews/PULSE-15-SUBSTITUTION-ROLE-REVIEW.md)
