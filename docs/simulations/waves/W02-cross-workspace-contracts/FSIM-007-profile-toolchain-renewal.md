# FSIM-007: Profile Renewal after Toolchain Change

Wave: W02
Revision: 1
State: Retraced
Claim state: simulated

## Question

What support state applies when a previously approved exact profile is used
with a newer compiler that has not completed renewal?

## Locked fixture

- Profile `windows-msvc-service` revision 7 is approved for rustc 1.89.0,
  Cargo 1.89.0, `x86_64-pc-windows-msvc`, and a named Windows SDK.
- Its evidence is 30 days old and otherwise unexpired.
- The active environment changes to rustc 1.90.0.
- No revision 8 profile or exact 1.90.0 validation exists.
- The application requests `test`.

Matched control: run the same request under the exact 1.89.0 environment.

## Governing specifications

- PLATFORM-001 exact environment, support, renewal, and stage states;
- APPLICATION-001 renewal and stale application state;
- RESOLUTION-001 eligibility and alternatives; and
- VIEW-001 plan output.

## Expected trace

| Stage | Predicted result |
|---|---|
| Change | Environment Change Record identifies exact compiler identity change |
| Profile | Revision 7 no longer covers the selected environment |
| Support | Existing support statement remains historical; it is not extended to 1.90.0 |
| Application Contract | Stale or incomplete for the new environment |
| Planning | May plan renewal validation but cannot claim supported execution |
| Resolution | Alternatives are select exact 1.89.0, renew into revision 8, use an explicitly unsupported experiment, or defer |
| Control | Exact 1.89.0 environment remains eligible subject to other evidence |

## Assertions

- [x] Evidence age alone does not preserve support after identity change.
- [x] Compiler-floor declarations do not replace exact execution.
- [x] Local success cannot infer support.
- [x] Renewal produces a reviewed new profile revision.
- [x] Historical revision 7 remains queryable.

## Simulation issues

None. PLATFORM-001 and APPLICATION-001 define an unambiguous stale and renewal
path.

## Claim boundary

No compiler was run. Eligibility and support are simulated from exact profile
and environment identities.
