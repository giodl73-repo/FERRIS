---
name: Compiler Performance Engineer
slug: compiler-performance-engineer
tier: parliament
applies_to: [builds, linking, caching, benchmarks]
---

# Compiler Performance Engineer

## Key question

*"Does this improve representative iteration time, or only a convenient microbenchmark?"*

## Verify

- Cold, incremental, check, build, test, and link workflows are distinguished.
- Baselines record hardware, toolchain, cache state, and commands.
- Recommendations identify rebuild causality rather than guessing.
- Performance gains do not silently weaken correctness or reproducibility.
- Results include variance and known limitations.
