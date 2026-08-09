# System Effects on Rust Build Latency

Date: 2026-08-09
Question: PERF-Q33
Status: Complete
Decision: adopt a system-environment fingerprint and attribution-confidence
model for every promoted build claim; add read-only warnings for filesystem
crossings, target placement, job-count saturation, memory reserve, concurrent
sessions, security scanning, indexing, power, and virtualization layers;
prototype diagnostics only through supported operating-system and Cargo
interfaces; reject automatic antivirus exclusions, security or indexing
disablement, power-plan changes, forced memory exhaustion, universal job
counts, and environment tuning presented as compiler optimization.

## Executive conclusion

System placement can exceed the compiler effects measured elsewhere in the
FERRIUM program.

Using one Linux toolchain, one METIS-CORE source revision, one lockfile, and
one WSL2 VM, the source and Cargo target were placed independently on:

- WSL's ext4 virtual disk; or
- the Windows `C:` NTFS volume mounted into WSL through the Windows filesystem
  boundary.

With both source and target on ext4, the five-run clean-build median was
4.81 seconds and the warm no-op median was 57.8 milliseconds. With both on the
mounted Windows volume, the medians were 16.52 seconds and 13.68 seconds. The
cross-filesystem row was 3.43x slower for clean builds and 236x slower for warm
no-op checks.

Target placement was the largest observed boundary. With source held on ext4,
moving only the target to the mounted Windows volume changed the clean median
from 4.81 to 15.55 seconds and the warm no-op median from 57.8 milliseconds to
13.89 seconds. Source placement still mattered when the target remained on
ext4: mounted-Windows source increased the clean median to 8.42 seconds and
the warm no-op median to 813.7 milliseconds.

This is not evidence that "virtualization is slow." The fastest row ran inside
WSL2. The measured cause is a layered storage placement:

```text
Linux process
  -> WSL2 VM
  -> ext4 VHD
```

versus:

```text
Linux process
  -> WSL2 VM
  -> Windows filesystem bridge
  -> NTFS volume
  -> Windows filter and security stack
```

The experiment cannot divide the mounted-path penalty among filesystem
translation, NTFS semantics, Defender scanning, indexing, filter drivers, or
host cache behavior. It establishes the combined boundary and matches
Microsoft's guidance to keep Linux-tool projects in the WSL filesystem.

CPU topology also requires a response curve rather than a core-count rule. On
the ext4/ext4 row, Cargo's one-job median was 7.45 seconds. Two jobs reduced it
to 4.63 seconds. Eight, sixteen, and twenty-four jobs were tightly grouped at
3.91, 3.78, and 3.98 seconds. Sixteen was best in this run, but eight was only
3.4% slower and twenty-four was 5.3% slower. The result supports diminishing
returns, not a universal recommendation for sixteen jobs.

Memory and concurrent-session evidence from PERF-Q16 supplies the operational
boundary. Four independent Cargo sessions at eight frontend jobs were 10.8%
slower than one-job sessions, used 21.6% more CPU, and raised peak memory from
1.62 to 2.11 GiB. The current WSL VM exposed 15 GiB and 4 GiB swap on a
31.7 GiB host. Available memory, VM limits, other sessions, editor processes,
and cache pressure are part of benchmark identity.

Security, indexing, power, frequency, and thermal state remain attribution
boundaries:

- Microsoft Defender Antivirus and real-time protection were enabled;
- the Windows Search service was running;
- the `C:` volume was NTFS, not a Dev Drive;
- the active Windows power plan was Balanced; and
- no reliable thermal or frequency trace was collected.

The experiment therefore does not assign a percentage to Defender, indexing,
power, or thermal behavior. Microsoft supplies a Defender Performance Analyzer
for measured scan attribution and a trusted Dev Drive performance mode that
keeps real-time protection enabled. Those supported paths precede exclusions.

FERRIUM should treat the environment as a first-class plan node. Build reports
must distinguish compiler work from execution substrate, source and target
filesystems, cache layer, job limit, memory reserve, session pressure, security
and indexer state, and power or thermal observations. Unknown remains unknown.

## Decision supported

This research determines:

- whether filesystem and execution placement can materially change Rust
  latency;
- whether source and target placement can be measured separately;
- why project-cold does not mean operating-system-cache-cold;
- how virtualization must be decomposed into VM, filesystem, mount, and
  resource-limit layers;
- whether logical processor count is a safe Cargo job recommendation;
- how prior concurrent-session and memory evidence belongs in benchmark
  identity;
- how antivirus, indexing, power, and thermal effects should be attributed;
- which operating-system diagnostics are safe and supported; and
- which environment changes FERRIUM must not automate.

It does not authorize security exclusions, disabling protection or indexing,
changing power plans, creating storage volumes, editing `.wslconfig`, forcing
memory pressure, migrating repositories, or implementing a scheduler.

## Evidence reviewed

### Local evidence

- [EXP-01 environment response matrix](perf-q33-system-effects/results/EXP-01-environment-response-matrix.md)
- [Rust latency telemetry](2026-08-07-rust-latency-telemetry.md)
- [Cargo graph scheduling](2026-08-08-cargo-graph-scheduling.md)
- [Frontend parallelism](2026-08-08-frontend-parallelism.md)
- [Debug information and object emission](2026-08-09-debug-information-object-emission.md)
- [Linking and incremental linking](2026-08-09-linking-incremental-linking.md)
- [Build latency measurement contract](../specs/BUILD_LATENCY_MEASUREMENT_CONTRACT.md)

### External sources

- [Microsoft WSL filesystem placement guidance](https://learn.microsoft.com/en-us/windows/wsl/filesystems)
- [Microsoft WSL configuration and VM resource controls](https://learn.microsoft.com/en-us/windows/wsl/wsl-config)
- [Microsoft Dev Drive guidance](https://learn.microsoft.com/en-us/windows/dev-drive/)
- [Microsoft Defender Dev Drive performance mode](https://learn.microsoft.com/en-us/defender-endpoint/microsoft-defender-endpoint-antivirus-performance-mode)
- [Microsoft Defender Antivirus Performance Analyzer](https://learn.microsoft.com/en-us/defender-endpoint/tune-performance-defender-antivirus)
- [Cargo `build.jobs` configuration](https://doc.rust-lang.org/cargo/reference/config.html#buildjobs)
- [rustc performance testing](https://rustc-dev-guide.rust-lang.org/tests/perf.html)

## Findings

### FERRIUM-446: source and target placement can dominate compiler deltas

**Sources:** EXP-01 storage matrix; Microsoft WSL filesystem guidance.

**Observed behavior:** the mounted-Windows source/target row took 16.52 seconds
clean and 13.68 seconds warm, versus 4.81 seconds and 57.8 milliseconds on
WSL ext4.

**Implication:** benchmark reports must name source, target, Cargo home, and
temporary filesystem placement. A compiler or profile comparison across
different placements is invalid.

**Confidence:** High for the pinned WSL fixture.

### FERRIUM-447: target placement was the largest measured storage boundary

**Sources:** EXP-01 ext4-source rows.

**Observed behavior:** moving only the target from ext4 to the mounted Windows
volume increased clean latency 223.2% and warm no-op latency 23,909.9%.

**Implication:** `CARGO_TARGET_DIR` placement is a first-class environment
dimension. Target-heavy object, metadata, archive, incremental, and fingerprint
traffic can dominate an otherwise small rebuild.

**Confidence:** High for this Linux-on-WSL path.

### FERRIUM-448: source placement remains visible after target isolation

**Sources:** EXP-01 ext4-target rows.

**Observed behavior:** with the target held on ext4, mounted-Windows source
increased the clean median 75.0% and warm no-op median 1,306.6%.

**Implication:** moving only build outputs is not always sufficient. Source,
manifest, lockfile, and freshness scanning can retain a filesystem-crossing
cost.

**Confidence:** High for the fixture.

### FERRIUM-449: project-cold is not operating-system-cache-cold

**Sources:** EXP-01 method and `/usr/bin/time` evidence; measurement contract.

**Observed behavior:** every clean sample used a new empty Cargo target, but
toolchain, registry, source, executable, VHD, and host page caches remained
available. Ext4 rows often reported no block inputs despite complete
compilation.

**Implication:** reports must separate project target state, Cargo registry
state, compiler process state, operating-system page cache, VM cache, and
physical storage state. FERRIUM should not claim a hardware-cold build unless
those layers are explicitly controlled.

**Confidence:** High.

### FERRIUM-450: virtualization is a stack, not a binary label

**Sources:** EXP-01; Microsoft WSL filesystem and configuration guidance.

**Observed behavior:** the fastest and slowest rows both ran inside the same
WSL2 VM. Their decisive difference was whether source and target remained on
ext4 or crossed to mounted Windows storage.

**Implication:** "native versus virtualized" is too coarse. Reports need guest,
kernel, virtual disk, mount, filesystem, host filter, and resource-limit
identity before attributing a virtualization effect.

**Confidence:** High on the decomposition; no bare-metal Linux comparison was
performed.

### FERRIUM-451: logical processor count is not a universal Cargo job count

**Sources:** EXP-01 Cargo jobs matrix; Cargo `build.jobs` documentation.

**Observed behavior:** on a 24-logical-processor host, one job took
7.45 seconds. Eight, sixteen, and twenty-four jobs took 3.91, 3.78, and
3.98 seconds. Maximum logical concurrency did not produce the minimum median.

**Implication:** FERRIUM should report a workload response curve and
diminishing-return region. It must not recommend jobs from logical processors
alone.

**Confidence:** High for the fixture; low for a universal optimum.

### FERRIUM-452: job-count response is environment and graph dependent

**Sources:** EXP-01; PERF-Q03; PERF-Q16.

**Observed behavior:** dependency readiness, available parallel units, CPU
classes, VM scheduling, page cache, and other host work all influence the
response. The four-job row exceeded 10% MAD/median even after a declared
warm-up, while eight, sixteen, and twenty-four were close.

**Implication:** unstable rows remain visible. The fastest point from one run
is not a portable setting, and small differences inside the response plateau
should not trigger configuration churn.

**Confidence:** High.

### FERRIUM-453: memory reserve and session pressure belong above one Cargo tree

**Sources:** PERF-Q16 multi-session control; EXP-01 environment inventory;
Microsoft WSL configuration guidance.

**Observed behavior:** four independent sessions increased CPU and memory while
slowing the batch. The current WSL VM exposed about half of host RAM and all
logical processors.

**Implication:** reports need host memory, guest limit, swap, current available
memory, peak process-tree RSS, session count, and jobserver domains. A later
cooperative budget may reserve interactive capacity, but forced memory pressure
and hidden process control are rejected.

**Confidence:** High for the prior session control; medium for broader
operational prevalence.

### FERRIUM-454: enabled security software is a confounder, not measured blame

**Sources:** local Defender state; Microsoft Defender Performance Analyzer.

**Observed behavior:** Defender Antivirus, real-time protection, behavior
monitoring, and IOAV protection were enabled. The mounted-path experiment did
not record Defender scan events.

**Implication:** FERRIUM may record protection state and recommend a supported
performance trace. It must not infer Defender's share from wall time or disable
protection to make a benchmark look faster.

**Confidence:** High on state; low on contribution to the measured delta.

### FERRIUM-455: Dev Drive performance mode is safer than broad exclusions

**Sources:** Microsoft Dev Drive and Defender performance-mode documentation.

**Observed behavior:** trusted Dev Drives use asynchronous Defender scanning
while retaining real-time protection. Standard NTFS volumes do not receive
that mode. Microsoft states that performance mode provides more protection
than folder exclusions.

**Implication:** a Windows diagnostic may identify an eligible measured Dev
Drive experiment or direct users to platform guidance. It must not create
volumes, alter trust, or add exclusions automatically.

**Confidence:** High on the supported platform boundary; unmeasured locally.

### FERRIUM-456: indexing state requires trace evidence before attribution

**Sources:** local Windows Search service state; EXP-01.

**Observed behavior:** Windows Search was running, but no per-path indexing
trace was collected.

**Implication:** indexer state belongs in the environment record. FERRIUM must
not assign a slowdown or disable indexing without an operating-system trace and
owner approval.

**Confidence:** High on state; unknown on measured impact.

### FERRIUM-457: power and thermal state must not be inferred from timing drift

**Sources:** local Balanced power-plan state; EXP-01 sample distributions.

**Observed behavior:** the active plan was Balanced. No reliable frequency,
temperature, fan, throttling, or battery trace accompanied the builds.

**Implication:** power source, plan, frequency policy, and thermal observations
are recorded when available. Outliers remain unexplained rather than being
retroactively labeled thermal throttling.

**Confidence:** High on the attribution rule; unknown on local thermal impact.

### FERRIUM-458: wall time and stable compiler-work metrics answer different questions

**Sources:** rustc performance-testing guidance; PERF-Q01.

**Observed behavior:** wall time captured the large storage and scheduling
effects users experience. Rust's upstream performance process uses controlled
hardware metrics such as instructions to reduce environmental noise when
evaluating compiler changes.

**Implication:** FERRIUM keeps wall time primary for workflow impact and
requires rustc-perf-compatible stable-work evidence before attributing small
changes to compiler algorithms.

**Confidence:** High.

### FERRIUM-459: environment diagnostics should remain read-only and reversible

**Sources:** all PERF-Q33 evidence and role review.

**Observed behavior:** supported diagnostics can expose placement, job limits,
memory, services, protection state, power plans, and VM configuration without
changing them.

**Implication:** the immediate product boundary is an environment fingerprint,
comparison guard, and warning surface. Configuration changes remain explicit
experiments owned by the user, operating-system administrator, Cargo, or
upstream Rust.

**Confidence:** High.

## System-environment ledger

The compiler query plan and Build Forest should be able to record:

| Field | Meaning |
|---|---|
| Host identity | OS build, machine class, CPU model/topology, physical memory, and storage device class |
| Execution substrate | Native host, VM, container, WSL version, guest kernel, and resource limits |
| Path placement | Source, target, Cargo home, temp, linker state, and cache filesystems and mounts |
| Cache state | Cargo target, registry, process, page, VM, and physical-storage cache assumptions |
| CPU policy | Cargo jobs, rustc jobs, jobserver domains, affinity if explicit, power plan, and observed frequency |
| Memory policy | Host available memory, guest cap, swap, reserve, process-tree peak, and concurrent sessions |
| Security state | Antivirus product, real-time status, supported performance mode, and trace reference |
| Indexing state | Indexer service and any measured path activity |
| Thermal state | Sensor or throttling evidence, or unknown |
| Background pressure | Competing build, editor, agent, CI, and system process activity |
| Attribution | Measured cause, correlated state, uncontrolled confounder, or unknown |

The ledger protects comparisons; it does not tune the host.

## Recommendations

### Adopt now

- Require source, target, Cargo home, temp, cache, and linker-state placement in
  promoted latency records.
- Separate project-cold, process-cold, page-cache, VM-cache, and physical-cold
  claims.
- Record execution substrate, VM limits, CPU topology, job settings, memory,
  protection, indexing, power, and known background activity.
- Interleave comparisons, declare warm-ups, retain every sample, and preserve
  unstable rows.
- Use wall time for user impact and rustc-perf-compatible stable metrics for
  compiler-change attribution.
- Warn when Linux tools build through mounted Windows paths, following
  Microsoft guidance.

### Prototype behind a compatibility boundary

- A read-only environment fingerprint and comparison guard.
- Source/target placement diagnostics for native, WSL, container, VM, network,
  and remote filesystems.
- A job-count response experiment that reports the plateau rather than one
  universal optimum.
- A cooperative machine/session census using supported process and jobserver
  evidence.
- Optional owner-approved Defender Performance Analyzer integration.
- Owner-approved Dev Drive, VM-resource, power, and indexing experiments with
  rollback and separate security review.

### Reject or defer

- Automatic antivirus exclusions or disabling protection.
- Automatic indexing, service, power-plan, affinity, priority, or thermal-policy
  changes.
- Creating, formatting, trusting, or migrating to a Dev Drive.
- Editing `.wslconfig`, VM limits, swap, or repository placement automatically.
- Forced memory exhaustion, cache dropping, broad process termination, or
  destructive hardware-cold procedures on a shared workstation.
- Universal Cargo or rustc job counts.
- Comparing different operating systems or toolchains and attributing the
  result solely to virtualization.
- Calling an environment placement gain a compiler optimization.

## Role review

| Role | Disposition |
|---|---|
| Rust Safety Steward | Accepted because security and correctness controls are not disabled to improve a benchmark. |
| Compiler Performance Engineer | Accepted because wall, CPU, RSS, placement, cache layers, sample distributions, and attribution confidence remain separate. |
| Interop Boundary Auditor | Accepted because Windows, Linux, WSL, mount, filesystem, target, and native-tool boundaries remain explicit. |
| AI Assurance Skeptic | Accepted because Defender, indexing, power, thermal, and virtualization shares remain unknown without traces. |
| Ecosystem Strategist | Accepted because the result uses Cargo, Microsoft, and rustc-supported diagnostics rather than replacement infrastructure. |
| Rust Maintainer | Accepted because the immediate output is a readable warning and comparison guard, not unexplained host mutation. |
| Native Platform Adopter | Accepted because enterprise security, policy, VM limits, rollback, and administrator ownership remain first-class. |
| Scope Keeper | Accepted because the result authorizes environment evidence and bounded experiments while rejecting host automation. |
| Validation Checker | Accepted because the matrix uses identical source, lockfile, toolchain, warm-ups, interleaving, five samples, negative boundaries, and visible variance. |

## Limitations

- The storage matrix used one public Rust library, one WSL2 VM, one Windows
  host, one Linux toolchain, and one physical storage device.
- Mounted-path cost combines filesystem translation, NTFS, host filters,
  Defender, indexing, and cache behavior; their shares were not isolated.
- No bare-metal Linux, Dev Drive, ReFS, network filesystem, container,
  Hyper-V VM, macOS, Linux antivirus, or cloud runner was measured.
- No reliable temperature, frequency, energy, instruction-count, or storage
  device-counter trace accompanied the runs.
- No deliberate memory exhaustion, cache dropping, security disablement,
  indexing disablement, power-plan change, or process-priority change was
  performed.
- Cargo job-count results describe this graph and environment, not a universal
  optimum.
