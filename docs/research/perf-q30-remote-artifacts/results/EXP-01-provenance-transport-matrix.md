# EXP-01: Remote provenance and transport matrix

Date: 2026-08-09

Question: can an immutable signed root safely transport one complete rustc
incremental generation, reject incompatible consumers before installation, and
save more compilation time than verification and transfer consume?

## Environment

- Windows 11 build 26310 on local NTFS
- Intel Core i7-12800HX, 16 cores and 24 logical processors
- 31.7 GiB RAM
- Producer: `rustc 1.99.0-nightly (1a98b1e13 2026-08-07)`, LLVM 23.1.0
- Mismatch control: `rustc 1.95.0 (59807616e 2026-04-14)`, LLVM 22.1.2
- Target: `x86_64-pc-windows-msvc`
- Fixture: one 374,120-byte metadata-only crate with 5,000 trait-backed owners
- Repetitions: five interleaved cold and restored compilations

The retained harness creates an Ed25519-signed canonical manifest, binds the
producer, source, compiler, sysroot libraries, command identity, output, file
set, and compressed transport digest, restores the payload into an isolated
directory, and then invokes rustc against the copy.

Run:

```powershell
python measure_remote_provenance.py
```

## Published root

| Property | Result |
|---|---:|
| Incremental files | 5 |
| Logical payload | 17,918,329 bytes |
| ZIP payload | 6,335,704 bytes |
| ZIP / logical ratio | 35.4% |
| Root digest verification | Passed |
| Manifest signature verification | Passed |
| Packaging time | 303.2 ms |

The root signed the compressed transport digest. This permits one
cryptographic blob verification before extraction. Rehashing every extracted
file remains useful as an audit or corruption diagnostic, but it duplicates
work when the signed transport blob has already been verified.

## Timing results

| Stage | Median | MAD |
|---|---:|---:|
| Cold incremental compile | 436.6 ms | 4.9 ms |
| Restored unchanged compile | 204.6 ms | 7.8 ms |
| Archive SHA-256 verification | 38.7 ms | 0.6 ms |
| Ed25519 signature verification | 0.174 ms | 0.003 ms |
| Extraction | 48.6 ms | 1.2 ms |
| Full extracted-tree audit | 97.1 ms | 5.0 ms |
| Local transport-digest restore and compile | 297.4 ms | 9.3 ms |
| Local full-tree-audit restore and compile | 402.5 ms | 7.9 ms |

The restored generation saved 232.0 ms of compilation before transport and
verification. Signature cost was negligible; hashing, extraction, and optional
second hashing dominated the consumer overhead.

## Transfer economics

The following calculations add measured verification, extraction, and restored
compile medians to idealized payload transfer time. They exclude network
latency, service queues, authentication, and contention, so they are favorable
to remote reuse.

| Verification mode | Break-even bandwidth | 100 Mbps result | 1 Gbps result | 10 Gbps result |
|---|---:|---:|---:|---:|
| Signed transport digest | 351 Mbps | 362.4 ms slower | 93.7 ms faster | 139.3 ms faster |
| Full extracted-tree audit | 1,071 Mbps | 459.5 ms slower | 3.3 ms slower | 42.3 ms faster |

This fixture does not justify universal remote restoration. It shows that a
small but meaningful generation can win on a fast local network when one
signed transport digest is sufficient, while duplicate verification or a
100 Mbps path erases the gain. Larger avoided compilation may improve the
ratio; many small roots, latency, and cold CAS misses may worsen it.

## Compatibility matrix

The compatibility policy evaluated producer evidence before installation.
Rejected cases were still compiled in disposable directories to observe rustc's
fallback behavior.

| Consumer | Policy | Fallback compile | Result |
|---|---|---:|---|
| Exact source URI, digest, arguments, rustc, and sysroot | Accept | 204.6 ms | Reused state; output matched |
| Same bytes at a relocated source URI | Reject: source URI | 400.0 ms | Near-cold work; output identity changed |
| Added `-Cdebuginfo=1` | Reject: compiler arguments | 478.9 ms | Full-scale work; output changed |
| Stable 1.95 compiler and sysroot | Reject: compiler identity | 515.8 ms | Full-scale work; output changed |
| One local source-body edit | Reject: source digest | 276.6 ms | Partial compiler reuse was possible, but not an exact remote hit |

Rustc safely missed or recomputed in these controls, but downloading the wrong
root first would waste transport. The store therefore needs a complete
consumer expectation and action identity before CAS lookup or installation.
Path relocation is not free: a portable design needs an explicit supported
path-remapping contract rather than assuming equal source bytes are sufficient.

After an exact restored compile, rustc changed the installed directory. The
immutable remote generation must therefore be copied or materialized into an
isolated mutable consumer directory; consumers must never compile in the CAS
root itself.

## Negative cases

| Case | Expected | Observed |
|---|---|---|
| Manifest builder identity changed without resigning | Reject | Signature rejected |
| Compressed archive byte changed | Reject | Transport digest rejected |
| `query-cache.bin` byte changed after extraction | Reject | File digest rejected |
| Generation advanced by an edit, then compared with the prior root | Reject | File-set mismatch |
| Signed label sequence 1 replayed after sequence 2 | Reject | Rollback rejected |
| Expired signed label | Reject | Expiration rejected |
| Valid label targeting a policy-revoked root | Reject | Revocation rejected |

These controls establish transport integrity and policy enforcement, not build
correctness. A trusted or compromised producer can still sign incorrect bytes.
The consumer must separately trust the builder and validate the build type,
external parameters, resolved dependencies, and required behavioral evidence.

## Limitations

- The fixture is synthetic, metadata-only, single-crate, and Windows-only.
- Transfer was modeled from byte size and bandwidth rather than a remote
  service with latency, authorization, retries, and contention.
- The experiment transported a complete compiler-private generation but did
  not claim format stability across compiler versions.
- The simple fixture had no build script, proc macro, native library, SDK,
  network access, clock dependency, or hidden environment input.
- The Ed25519 key modeled signature mechanics, not production keyless identity,
  transparency-log inclusion, threshold trust, or key rotation.
- Cargo artifact sets remain governed by Cargo fingerprints, dep-info, sidecars,
  layout, and upstream cache design; this experiment did not replace them with
  raw compiler outputs.

