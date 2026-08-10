# Blueprint Research Questions

Blueprint research turns the completed performance and Crates Series evidence
into bounded Cargo Application Model decisions.

| Question | Status | Decision |
|---|---|---|
| [BLUE-Q01](BLUE-Q01-forest-root-references.md) | Complete | Use typed refs over immutable roots; reserve labels for metadata and keep artifact restoration separately gated |
| [BLUE-Q02](BLUE-Q02-federated-execution-plan.md) | Complete | Add a federated Blueprint Plan: the plan is global, the work remains local to owner-specific closures |
| [BLUE-Q03](BLUE-Q03-cross-command-scope-mapping.md) | Complete | Define multi-dimensional scope coordinates and typed mappings across source, Cargo, compiler, validation, runtime, contract, native, and evidence systems |
| [BLUE-Q04](BLUE-Q04-competitive-positioning-cli.md) | Complete | Position FERRIS Blueprint as Cargo-native cross-workspace orchestration with `ferris blueprint` and `cargo blueprint` over one engine |

No Blueprint research question authorizes implementation. Each question must
produce cited findings, positive and negative controls, role review, explicit
non-goals, and a separately approved implementation gate.
