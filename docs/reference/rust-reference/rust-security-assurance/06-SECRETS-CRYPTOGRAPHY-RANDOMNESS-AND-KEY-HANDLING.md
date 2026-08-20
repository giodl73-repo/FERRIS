---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-security-assurance:secrets-cryptography-randomness-and-key-handling
kind: guide
module: rust-security-assurance
section: security-engineering
title: Secrets, Cryptography, Randomness, and Key Handling
status: source-custody
source_custody: partial
current_path: rust-security-assurance/06-SECRETS-CRYPTOGRAPHY-RANDOMNESS-AND-KEY-HANDLING.md
canonical_path: rust-security-assurance/06-SECRETS-CRYPTOGRAPHY-RANDOMNESS-AND-KEY-HANDLING.md
backsource_ids: [proof-backfill:rust-security-assurance:06-secrets-cryptography-randomness-and-key-handling]
concepts: [secrets, cryptography, randomness, key management, zeroization]
root_concepts: [key management]
index_roles: [guide, root-concept]
remap_from: []
remap_to: []
updated: null
---
# Secrets, Cryptography, Randomness, and Key Handling

Rust can make secret-handling APIs explicit and memory-safe, but it cannot turn
an ad hoc cryptographic protocol into a secure one. Start with a standard
protocol and a supported implementation, then design the key lifecycle:
generation, storage, use, rotation, revocation, audit, and destruction.

## The Big Picture

```
+============================================================================+
|                         CRYPTOGRAPHIC LIFECYCLE                            |
+============================================================================+
| entropy source -> key generation -> protected storage -> constrained use   |
|       |              |                    |                |                |
|       v              v                    v                v                |
| health/OS trust   algorithm/size      access identity   protocol context    |
+----------------------------------------------------------------------------+
| rotate -> distribute new trust -> revoke old -> retain audit -> destroy    |
+----------------------------------------------------------------------------+
| cross-cutting: logging | memory copies | backups | crash dumps | side channels |
+============================================================================+
```

The algorithm is one box. Most real failures occur in context binding, nonce
management, authorization, storage, rotation, or accidental disclosure.

## Choose Protocols Before Primitives

| Need | Preferred level of abstraction | Avoid |
|------|-------------------------------|-------|
| Transport security | current TLS implementation with peer/name validation | custom handshake or raw encryption |
| Stored-data confidentiality + integrity | authenticated encryption with managed nonce/key policy | encryption without authentication |
| Password storage | purpose-built password hashing with reviewed parameters | fast general hash or reversible encryption |
| Token/signature verification | standard format/protocol with strict algorithm/key policy | "accept any advertised algorithm" |
| Key derivation | standard KDF with domain-separated context | truncating a general hash without design review |
| Random identifiers/security tokens | OS-backed CSPRNG and enough entropy | timestamps, counters, non-crypto PRNG |

Algorithm recommendations evolve. Bind choices to an organizational crypto
standard and named protocol version, and record a migration path.

## Randomness: Distinguish Simulation from Security

```
deterministic PRNG seed ----> reproducible tests/simulations

OS entropy/CSPRNG ----------> keys, nonces where random, reset tokens,
                              session identifiers, salts
```

Rust's standard library intentionally does not expose a general secure RNG.
Use a maintained crate that obtains randomness from supported OS facilities,
and verify its API for the pinned version and targets. Do not silently fall back
to predictable randomness when entropy acquisition fails.

For the `getrandom` 0.3 API, a direct OS-random fill is:

```rust
fn new_token() -> Result<[u8; 32], getrandom::Error> {
    let mut token = [0_u8; 32];
    getrandom::fill(&mut token)?;
    Ok(token)
}
```

The call can fail and may block in platform-specific early-boot conditions.
The crate's target support and backend features are part of the release matrix;
the example does not define token encoding, storage, expiry, or authorization.

| Random value | Uniqueness requirement | Secrecy requirement |
|--------------|------------------------|---------------------|
| Password salt | unique with overwhelming probability | no |
| AEAD nonce | scheme-specific; often unique per key | usually no |
| Reset/session token | unpredictable and collision-resistant | yes |
| Test seed | reproducible | no |
| Long-term private key | generated from approved CSPRNG | yes |

Nonce reuse can catastrophically break some AEAD schemes even though the Rust
types and memory accesses are safe. Model nonce allocation as persistent state,
not as a formatting detail.

Password hashing requires a purpose-built, salted, deliberately expensive
construction with parameters calibrated to the deployment and current
organizational standard. Argon2id, scrypt, and PBKDF2 appear in different
standards and compatibility environments; the algorithm name alone is not a
parameter, side-channel, availability, or migration policy. Rate-limit expensive
verification and design account-existence behavior deliberately so the defense
does not become an unauthenticated CPU-exhaustion or oracle surface.

## Secret Custody and Process Boundaries

```
workload identity
      |
      v
secret/key service -- policy --> permitted operation
      |                               |
      | return key?                   | sign/decrypt inside service/HSM?
      v                               v
process memory                   opaque result
```

Prefer non-exportable keys or remote cryptographic operations for
high-consequence material when latency and availability permit. If a process
must receive secret bytes:

- fetch them late and keep them briefly;
- avoid command-line arguments and broad environment inheritance;
- never include them in logs, panic messages, metrics, traces, or serialized
  debug structures;
- restrict core/crash dumps and diagnostic access;
- avoid unnecessary clones and conversions;
- define rotation behavior without process-wide outage.

## Zeroization Is a Narrow Control

Crates can overwrite a buffer on drop, but zeroization is not a universal
erasure proof. Copies may exist in allocator arenas, registers, swap, crash
dumps, serialization buffers, logs, or compiler-generated temporaries. The
optimizer and platform affect what can be guaranteed.

| Control | Helps with | Does not establish |
|---------|------------|--------------------|
| Zeroizing owned buffer | post-use memory remanence in that buffer | absence of all copies |
| Locked/non-pageable memory | swap exposure on supported OS/config | protection from process compromise |
| Secret wrapper without `Debug` | accidental formatting | logging by custom code |
| HSM/managed key operation | non-exportability and policy enforcement | correct application authorization |

## Cryptographic API Review

```
input -> parse strict format -> select fixed algorithm/policy
      -> validate key identity + purpose + time/audience/context
      -> perform operation
      -> handle failure without oracle or secret leakage
```

Check:

1. algorithm and mode are fixed by trusted policy, not attacker input;
2. key usage is separated by purpose and environment;
3. signatures/MACs bind every security-relevant field;
4. comparisons and error behavior do not expose avoidable side channels;
5. nonce/counter state survives restart where required;
6. certificate/key rotation supports overlap and rollback;
7. dependency features select the intended backend on every target.

Do not hand-roll "constant-time" comparison or masking code from source-level
intuition. Compiler optimization, microarchitecture, caches, and the selected
backend affect leakage. Prefer reviewed protocol/library APIs, retain their
target assumptions, and use specialized side-channel evaluation when the threat
model requires more than best-effort avoidance.

## Compliance Boundary

FIPS or other regulatory requirements generally concern a validated
cryptographic module, approved configuration, operational environment, and key
management process. Using an algorithm with an approved name does not by itself
make an application compliant. Confirm requirements with qualified legal,
compliance, and cryptographic specialists for the jurisdiction and deployment.

## Old World -> New World Bridge

| Established practice | Rust expression |
|----------------------|-----------------|
| .NET `RandomNumberGenerator` vs `Random` | OS-backed crypto RNG crate vs deterministic PRNG |
| `SecureString` debate | minimize copies and lifetime; no type can erase every external copy |
| DPAPI/Keychain/libsecret | platform credential protection behind a narrow Rust interface |
| HSM/Key Vault signing | workload identity requests non-exportable operation |
| Approved crypto library list | pinned crates/backends/features plus protocol and target review |

For Microsoft environments, Managed Identity and Azure Key Vault/Managed HSM
can provide credential-free access and auditable key operations; platform crypto
providers may help satisfy enterprise requirements. These are supplemental
implementations of universal custody principles.

## Common Confusion Points

- **"Memory safety protects secrets."** It reduces accidental corruption and
  exploitation paths; logs, authorization, dumps, side channels, and compromise
  remain.
- **"Encryption implies integrity."** Use an authenticated construction or a
  protocol that provides both.
- **"Nonce means random."** Some schemes require uniqueness, some derive
  nonces; follow the exact construction.
- **"Zeroize means erased."** It covers a specific buffer under specific
  implementation conditions.
- **"A crypto crate is automatically audited."** Review project status,
  backend, features, target support, advisories, and protocol use.
- **"FIPS-approved algorithm means compliant product."** Validation and
  operational scope are broader.

## Decision Cheat Sheet

| Situation | Do |
|-----------|----|
| Need secure transport | Use current TLS stack and strict peer/name policy |
| Need password storage | Use reviewed password-hashing library and current organizational parameters |
| Need random token/key | Use supported OS-backed CSPRNG; fail closed on entropy failure |
| Need AEAD nonce | Follow the selected scheme's exact uniqueness/generation contract |
| High-value signing key | Prefer non-exportable HSM/service operation with workload identity |
| Secret must enter memory | Minimize lifetime/copies and control logs/dumps; consider zeroization as defense in depth |
| Regulated crypto | Verify validated module/config/environment with specialists |

## Primary Sources

- RustCrypto organization: https://github.com/RustCrypto
- `getrandom` documentation: https://docs.rs/getrandom/
- IETF TLS 1.3, RFC 8446: https://www.rfc-editor.org/rfc/rfc8446
- NIST Cryptographic Standards and Guidelines:
  https://csrc.nist.gov/projects/cryptographic-standards-and-guidelines
- OWASP Cryptographic Storage Cheat Sheet:
  https://cheatsheetseries.owasp.org/cheatsheets/Cryptographic_Storage_Cheat_Sheet.html
- Azure Key Vault documentation: https://learn.microsoft.com/azure/key-vault/

## Related Guides

- Previous: [05-BUILD-SCRIPTS-PROC-MACROS-COMPILERS-AND-BUILD-TRUST.md](05-BUILD-SCRIPTS-PROC-MACROS-COMPILERS-AND-BUILD-TRUST.md)
- Next: [07-PARSING-DESERIALIZATION-INPUT-VALIDATION-AND-PROTOCOL-ABUSE.md](07-PARSING-DESERIALIZATION-INPUT-VALIDATION-AND-PROTOCOL-ABUSE.md)
- General cryptography: [../cryptography/00-OVERVIEW.md](../cryptography/00-OVERVIEW.md)
