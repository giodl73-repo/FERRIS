# REEL Scene-Delivery Value Cohort

Date: 2026-09-15
Status: Complete; invalid before planning

## Frozen case

- REEL head: `5c896f5a9d3fb7bd4a709ace66dfe119c6a568bb`;
- REEL base: `9719aacc4d4554ad8d38502ae33135803836fd3c`;
- Ferris revision: `1baa6a8`;
- selected command: `cargo test --test scene_delivery`;
- full command: `cargo test --all-targets --all-features`; and
- platform: local Windows x86_64 without FFmpeg.

The selected command is documented by REEL and is a target subset of the full
command owned by REEL CI. The separately documented ignored episode test was
not substituted into either lane.

## Findings

### FERRIS-VALUE-033: The focused REEL target was runnable without FFmpeg

The selected command ran eight tests. Six passed and the two explicitly
FFmpeg-dependent tests remained ignored. The command exited `0` with semantic
test evidence.

### FERRIS-VALUE-034: The full owner command had an undeclared local prerequisite

The full command reached real tests but exited `101`. Two non-ignored
`vfx_effect_pass_v0319` tests attempted to launch an external program and
panicked with `program not found`. FFmpeg was unavailable.

The distinction matters: other REEL tests explicitly declare their FFmpeg
dependency through `#[ignore]`, but these two failing tests did not. Ferris did
not reinterpret or bypass that owner result.

### FERRIS-VALUE-035: Failed full preflight blocks value measurement

The strict design required both commands to pass before planning or timing.
Because the full reference failed, no plan, selected/full pair, timing, median,
or value conclusion followed.

## Decision

The real-adopter value gate remains `No`. No FFmpeg installation, command
substitution, REEL change, or narrower reference was attempted. All disposable
roots were removed and the BISECT, ICELINES, and REEL research clones remained
clean. Pulse 21 is exhausted and grants no retry or later-gate authority.
