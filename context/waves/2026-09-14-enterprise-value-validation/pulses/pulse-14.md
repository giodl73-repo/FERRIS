# Pulse 14: Native-Cargo-Path Cross-Platform Onboarding

Status: Complete; incomplete after Linux removal
Implementation authority: Exhausted

## Authority

The Linux harness may prepend existing `/root/.cargo/bin` to its process-local
`PATH`. It MUST prove Cargo, Rust, Git, and the staged PowerShell runtime before
planning. Linux MUST complete execution, verification, removal, post-removal
owner commands, and cleanup before Windows materializes.

The pulse otherwise inherits every Pulse 13 input, LF gate, Windows MSVC
environment, command, stop condition, removal invariant, cleanup requirement,
and exclusion. Prior pulse results do not count as Pulse 14 success evidence.

Success requires four successful executions and verified receipts, complete
onboarding removal, four passing post-removal owner commands, exact clean
revisions, and complete disposable cleanup.

## Result

Both Linux consumers planned, executed their owner commands through Ferris,
and produced receipts that passed verification. The harness then removed each
`.ferris/` tree and the root request and proved the tracked trees clean.

Post-removal validation attempted global `pwsh`, which is unavailable. The only
native runtime had correctly been removed with `.ferris/`. Windows therefore
did not materialize. Linux aggregate evidence was retained privately, the
native root and archive were removed, and Pulse 14 is exhausted.
