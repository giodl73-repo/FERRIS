# EXP-01: Environment Response Matrix

Date: 2026-08-09

Question: how much can filesystem placement and CPU job policy change Rust
build latency without changing source, dependencies, or compiler algorithms?

## Environment

Host:

- Windows 11 Enterprise Insider Preview, build 26310
- Intel Core i7-12800HX
- 16 physical cores and 24 logical processors
- 31.7 GiB physical memory
- `C:` fixed NTFS volume
- Balanced power plan
- Microsoft Defender Antivirus and real-time protection enabled
- Windows Search running

Guest:

- WSL2 Ubuntu 24.04
- Linux `6.6.87.2-microsoft-standard-WSL2`
- 24 logical processors visible
- 15 GiB memory and 4 GiB swap visible
- ext4 root filesystem in the WSL virtual disk
- Windows `C:` mounted through the WSL Windows-filesystem boundary

Toolchain and fixture:

- `rustc 1.97.1 (8bab26f4f 2026-07-14)`
- LLVM 22.1.6
- `cargo 1.97.1 (c980f4866 2026-06-30)`
- target `x86_64-unknown-linux-gnu`
- METIS-CORE revision `78ae34090e043e79a206f2daffaa3889389b4790`
- one generated and shared `Cargo.lock`
- registry acquisition completed before measurements
- `cargo build --lib --locked --offline --quiet`

Each row had one excluded warm-up followed by five interleaved measured
repetitions. `/usr/bin/time -v` captured process-tree user time, system time,
peak RSS, page faults, and block I/O. Every project-cold sample used a new empty
target directory; global toolchain, registry, operating-system, VM, and storage
caches were not dropped.

## Storage placement matrix

The source and target were copied or created independently on:

- `ext4`: WSL's Linux filesystem; or
- `mounted Windows`: the NTFS `C:` volume accessed from Linux through WSL.

### Wall-time samples

| Source | Target | Project-cold samples, ms | Warm no-op samples, ms |
|---|---|---|---|
| Mounted Windows | Mounted Windows | 14,740.5; 16,946.0; 17,035.4; 15,894.0; 16,516.4 | 9,428.8; 14,141.4; 13,678.6; 15,331.3; 12,427.8 |
| Mounted Windows | ext4 | 8,235.9; 8,323.4; 8,420.8; 8,754.2; 9,207.6 | 813.7; 836.6; 721.8; 823.9; 744.3 |
| ext4 | Mounted Windows | 15,552.9; 15,769.5; 15,868.6; 12,988.8; 13,893.9 | 8,372.8; 15,212.3; 19,866.8; 13,889.7; 13,414.3 |
| ext4 | ext4 | 4,694.7; 4,492.1; 6,851.0; 5,503.7; 4,812.8 | 57.8; 56.8; 62.3; 55.9; 80.9 |

### Median outcomes

| Source | Target | Cold median | Cold MAD | Warm median | Warm MAD | Cold vs ext4/ext4 | Warm vs ext4/ext4 |
|---|---|---:|---:|---:|---:|---:|---:|
| Mounted Windows | Mounted Windows | 16,516.4 ms | 519.0 ms | 13,678.6 ms | 1,250.8 ms | +243.2% | +23,545.1% |
| Mounted Windows | ext4 | 8,420.8 ms | 184.9 ms | 813.7 ms | 22.8 ms | +75.0% | +1,306.6% |
| ext4 | Mounted Windows | 15,552.9 ms | 315.7 ms | 13,889.7 ms | 1,322.6 ms | +223.2% | +23,909.9% |
| ext4 | ext4 | 4,812.8 ms | 320.7 ms | 57.8 ms | 1.9 ms | baseline | baseline |

All promoted storage rows remained below 10% MAD/median. The mounted-target
warm rows were close to the threshold at 9.1% and 9.5%.

### Resource evidence

Project-cold medians:

| Source | Target | User | System | Peak RSS | Major faults |
|---|---|---:|---:|---:|---:|
| Mounted Windows | Mounted Windows | 5.28 s | 1.79 s | 246.6 MiB | 1,827 |
| Mounted Windows | ext4 | 5.37 s | 1.25 s | 241.5 MiB | 259 |
| ext4 | Mounted Windows | 5.03 s | 1.66 s | 247.8 MiB | 1,736 |
| ext4 | ext4 | 4.78 s | 1.22 s | 241.2 MiB | 240 |

The wall-time gap was much larger than CPU-work differences. Mounted targets
also showed substantially more major page faults and block input. These
counters establish a system boundary, not a unique attribution to NTFS,
virtualization, Defender, indexing, or one filter driver.

## Cargo job-count response

The source and target both remained on ext4. Each job count had one excluded
warm-up and five project-cold repetitions.

| Jobs | Wall samples, ms | Median | MAD | User + system | Peak RSS |
|---:|---|---:|---:|---:|---:|
| 1 | 8,753.0; 7,448.5; 7,304.0; 7,828.7; 6,682.0 | 7,448.5 ms | 380.2 ms | 4.54 s | 230.4 MiB |
| 2 | 5,299.1; 4,728.1; 4,555.5; 4,630.7; 4,196.1 | 4,630.7 ms | 97.3 ms | 4.68 s | 233.6 MiB |
| 4 | 5,071.5; 4,442.4; 4,412.1; 3,891.6; 3,721.8 | 4,412.1 ms | 520.5 ms | 5.11 s | 238.8 MiB |
| 8 | 4,762.2; 3,949.4; 3,620.3; 3,838.3; 3,911.5 | 3,911.5 ms | 73.2 ms | 4.91 s | 241.8 MiB |
| 16 | 4,643.3; 4,110.5; 3,774.8; 3,782.2; 3,767.8 | 3,782.2 ms | 14.4 ms | 4.82 s | 241.3 MiB |
| 24 | 3,982.4; 4,066.1; 4,131.1; 3,908.1; 3,635.4 | 3,982.4 ms | 83.7 ms | 4.88 s | 241.4 MiB |

The four-job row had 11.8% MAD/median and is unstable. The stable eight,
sixteen, and twenty-four rows differed by only 5.3% from best to worst.
Sixteen happened to produce the lowest median, but the evidence establishes a
plateau and diminishing returns rather than a universal setting.

One job was 96.9% slower than sixteen. Two jobs were 22.4% slower. Twenty-four
logical jobs were 5.3% slower than sixteen and 1.8% slower than eight.

## Existing concurrent-session control

PERF-Q16 measured four independent Cargo sessions with isolated targets. At
eight frontend jobs, the complete batch was 10.8% slower than one-job
sessions, used 21.6% more CPU, and raised peak memory from 1.62 to 2.11 GiB.

This evidence is not combined numerically with the Linux Cargo-jobs matrix. It
supplies the separate machine-level session and memory-pressure boundary.

## Security, indexing, power, and thermal state

Read-only host inventory found:

- Defender Antivirus enabled;
- real-time, behavior, and IOAV protection enabled;
- Windows Search running;
- `C:` formatted as NTFS;
- Balanced power plan active; and
- no reliable thermal or frequency trace.

No security or indexing service was disabled. No exclusions, Dev Drive,
power-plan changes, cache dropping, forced memory pressure, affinity, priority,
or process termination were used.

The storage result therefore represents the complete observed mounted-path
stack. It does not isolate Defender, indexing, NTFS, WSL translation, host
filters, or storage hardware.

## Interpretation

- Source and target filesystem placement can exceed many compiler-profile
  deltas.
- Cargo target placement was the largest measured storage boundary.
- Warm no-op Cargo work is highly sensitive to metadata and fingerprint path
  operations.
- WSL ext4 performed well; the result cannot be summarized as virtualization
  overhead.
- Project-cold means an empty target, not empty operating-system or VM caches.
- Cargo job count needs a response curve; logical processors are not an
  automatic optimum.
- Memory reserve and independent sessions belong in the environment record.
- Security, indexing, power, and thermal shares require dedicated traces.

## Limitations

- One public fixture, host, VM, toolchain, storage device, and guest OS.
- Source copies had identical content and lockfile but different absolute
  paths; debug information can retain paths.
- No bare-metal Linux or cross-platform output comparison was attempted.
- `/usr/bin/time` block counters are operating-system counters, not logical
  filesystem-operation counts.
- No instruction, cycle, energy, temperature, frequency, Defender-event,
  indexer-event, or storage-device trace was collected.
- No deliberate adverse-condition experiment was run on the shared
  workstation.
