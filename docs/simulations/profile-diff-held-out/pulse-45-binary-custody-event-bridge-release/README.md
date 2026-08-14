# Pulse 45 binary-custody event bridge release

This public, standard-library-only composition adapter makes a completed
Pulse 44 retained-binary result usable as one intermediate gate in a larger
Pulse 43 ordered ledger. It does not alter, reinterpret, or reopen the sealed
Pulse 44 release: Pulse 44's `terminal-stop/completed` remains its valid
standalone result. Pulse 45 validates that entire closed result and emits its
own platform-specific Pulse 43 `gate-complete/passed` event only after every
required Pulse 44 completion field is present.

This is public release-composition infrastructure only. It creates no
diagnostic, custody, private-data, executable-byte, product, category, fix,
support, certification, or PLATFORM-001 authority.

## Fixed predecessor and mapping

Before importing the predecessor, the CLI verifies the exact Pulse 44
manifest raw/aggregate identities
`sha256:eae4db6c4add7f20a919cd301dc307cc7845f808f458219b5627c135ed5f0c94` /
`sha256:a22efbbb233ee53550c8ac9771a83af3829c16ce8f7f7a2ff15638adf2f58f94`,
qualification receipt raw/payload identities
`sha256:d17ac162d7e8d5afb9f41fa789afe43c2512f2ee1dd30b4afaae4bde16491f1b` /
`sha256:a5a5be3d0832476ba0addb4edda2790d3e02acda49a1266601e6065bc0f9cf29`,
release seal raw/payload identities
`sha256:97598062129317e89862407cc00971aa11ac179420088f4d508678b535cab2a8` /
`sha256:4b90c678255fe3567760ce2ef253192a5489ee684ae57a4eb15446f038c189b5`,
and the manifest-bound adapter source digest
`sha256:101951fed6006b390499ba6400c828a0c0e902f018ec75bdb30bde9eb23f0942`.

Only these platform records are accepted:

| Platform | Pulse 43 catalog gate |
|---|---|
| `windows-x86_64` | `windows-retained-binary-custody` |
| `ubuntu-24.04-x86_64` | `ubuntu-retained-binary-custody` |

`bridge_pulse_44(repo, cutoff, platform, work_root, final_root,
invoker=...)` calls its injected Pulse 44 invoker once. The CLI loads the
verified public Pulse 44 adapter and calls `retain_binary_custody` once with
those exact five arguments. There is no retry, fallback, alternate adapter,
or implicit platform mapping.

## Closed translation boundary

`schemas/ferris.pulse-45-binary-custody-event-bridge.v1.schema.json` documents
the complete closed Pulse 44 source summary and the bounded bridge output.
The implementation rejects unknown members, duplicate JSON members in sealed
predecessor records, and every malformed or success-shaped partial summary.
Published Pulse 44 output must have outcome `published`, state `published`,
`final_files_present:true`, `files`, work, stage, and final verification all
`2/2`, one rename, zero retries, and the exact
`retained-binary-custody` `terminal-stop/completed` event. Only then does
Pulse 45 return a platform gate `gate-complete/passed`.

A closed Pulse 44 failed record with `absent`, `rolled-back`, or
`indeterminate` custody state remains failed: Pulse 45 returns that exact
public `P44-*` failure code, preserves the state, and emits a
platform-specific `terminal-stop/failed`. An invocation exception or malformed
record instead produces a bounded `P45-*` terminal failure. Results contain
only fixed release identities, public platform/gate metadata, bounded
counts, and a Pulse 43 event. They contain no local path, filesystem detail,
private data, executable bytes, or source Pulse 44 terminal-completion event.

Run from any working directory:

```console
python binary_custody_event_bridge.py --repo C:\public\cutoff-checkout --cutoff 29517d732db13cc2ffa304684b344f3538ab587d --platform windows-x86_64 --work-root C:\public\work --final-root C:\public\custody
```

The work and final roots are passed to Pulse 44 and remain controlled runtime
state; they are never emitted by Pulse 45 or committed.

## Qualification

The 14 deterministic Python test methods cover both platform mappings; every
required completion field; malformed and success-shaped partial records; an
incorrect Pulse 44 terminal event; incoherent success sync postures; all three
preserved failure postures;
exactly-once and thrown invocation behavior; path-free output; exact CLI
argument forwarding; predecessor identity loading; and Pulse 43 composition.
The composition control proves that both platform `gate-complete` events may
precede a later gate, while either platform `terminal-stop/failed` prevents a
later ordered event. The Rust integration validator recomputes this release's
sealed identities and repeats both composition outcomes.
