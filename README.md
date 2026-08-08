# FERRIUM

**AI-native systems engineering for safe, efficient Rust tooling.**

FERRIUM is a research and engineering lab for the unfinished parts of modern
native development: compiler-grounded AI assistance, fast builds, trustworthy
language boundaries, supply-chain assurance, concurrency observability, and
portable accelerated compute.

Ferris is the lab's agent identity: an AI collaborator whose work is checked
against compiler, test, lint, benchmark, and evidence surfaces rather than
accepted as plausible text.

## Initial research lanes

| Lane | Question |
|---|---|
| Boundary | How can Rust enter C and C++ systems without weakening safety at the boundary? |
| Hammer | How can build causality, caching, linking, and workspace structure reduce iteration time? |
| Temper | How can generated native code carry auditable safety, provenance, and compliance evidence? |
| Lens | How can async and concurrent Rust become easier to observe, explain, and replay? |
| Furnace | How can ownership-aware native code target CPUs, GPUs, and accelerators portably? |

These are research lanes, not promised products or separate repositories.
FERRIUM promotes a lane into implementation only after a cited research note,
measurable baseline, and bounded validation contract exist.

## Foundation state

FERRIUM begins as a research-and-governance repository. It contains no product
code, crates, packages, or implementation commitments yet. The first research
wave will compare the opportunity lanes, establish measurable baselines, and
recommend what—if anything—should be prototyped.

## Research

- [Rust compiler performance: architecture, bottlenecks, and FERRIUM opportunities](docs/research/2026-08-07-rustc-compiler-performance.md)
- [Rust latency component roadmap](docs/research/2026-08-07-rust-latency-component-roadmap.md)

## Review model

FERRIUM uses the
[ROLES](https://github.com/giodl73-repo/ROLES) `.roles` convention. Rust safety,
compiler performance, interoperability, AI assurance, ecosystem strategy,
scope, validation, and adopter concerns are represented as explicit review
lenses.

The [FERRIUM engineering principles](docs/governance/ENGINEERING_PRINCIPLES.md)
define the lab's decision rules, common failure modes, prototype gate, and
initial review disposition from every repository role.

## Operating rules

1. Research before standardizing a language, protocol, benchmark, or product.
2. Treat compiler success as evidence, not proof of behavioral correctness.
3. Keep shared contracts product-neutral.
4. Measure build, runtime, safety, and migration claims.
5. Do not create implementation packages before the research gate selects a
   bounded prototype.
6. Record non-goals and rejected approaches.

## Non-goals

- Creating a general-purpose Rust replacement before a defensible wedge exists.
- Building another text-only coding assistant without compiler-grounded checks.
- Claiming formal verification, memory safety, or performance without evidence.
- Embedding portfolio-product semantics in shared crates.
- Treating experimental lane names as committed products.

## Validation

```powershell
git grep -n "FERRIUM" -- README.md PRODUCT_PLAN.md context/waves/PHASES.md
git diff --check
```

## License

MIT.
