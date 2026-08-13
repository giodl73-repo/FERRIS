# Experimental Profile Evidence Fixture Matrix

Status: Development conformance fixtures
Schema: `ferris.profile-evidence/v0`

These synthetic pairs exercise the Pulse 14 `profile-diff` boundary across
the nine independent families required by Draft PLATFORM-001. They are not
canonical profiles, owner observations, compatibility results, support
statements, approvals, or evidence that PLATFORM-001 may advance.

| Family | Fixture directory | Material section | Representative changed path |
|---|---|---|---|
| Hosted service | `hosted-service/` | `stages` | `/sections/stages/deploy/state` |
| CLI and configuration | `cli-configuration/` | `lifecycle` | `/sections/lifecycle/removal/state` |
| Pure data processing | `pure-data/` | `closure` | `/sections/closure/active/digest` |
| Embedded and `no_std` | `embedded-no-std/` | `targets` | `/sections/targets/thumbv7em-none-eabihf/state` |
| Browser WASM | `browser-wasm/` | `targets` | `/sections/targets/wasm32-unknown-unknown/runtime` |
| WebAssembly component | `wasm-component/` | `identity` | `/sections/identity/component_contract` |
| Bundled or system-native dependency | `native-dependency/` | `native` | `/sections/native/openssl/source_mode` |
| Identity, credential, TLS, and cryptographic provider | `identity-crypto-provider/` | `providers` | `/sections/providers/tls/provider` |
| Testing, assurance, packaging, and deployment | `assurance-packaging-deployment/` | `assurance` | `/sections/assurance/package_attestation/state` |

Every pair:

- keeps one profile ID and consumer across revisions;
- changes revision `r1` to `r2`;
- changes at least one family-specific section value;
- includes distinctive raw values that tests require to remain absent from
  human and JSON output; and
- remains below all Pulse 14 input and change-count bounds.

No fixture invokes Cargo, an owner tool, a network, a platform, a provider, a
native library, a build, a deployment, or an approval workflow.
