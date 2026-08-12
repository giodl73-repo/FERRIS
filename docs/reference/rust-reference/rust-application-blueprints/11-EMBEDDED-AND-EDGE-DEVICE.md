---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-application-blueprints:embedded-and-edge-device
kind: guide
module: rust-application-blueprints
section: rust-application-blueprints
title: Embedded and Edge Device Blueprint
status: source-custody
source_custody: partial
current_path: rust-application-blueprints/11-EMBEDDED-AND-EDGE-DEVICE.md
canonical_path: rust-application-blueprints/11-EMBEDDED-AND-EDGE-DEVICE.md
backsource_ids: [mdloom-backfill:rust-application-blueprints:11-embedded-and-edge-device]
concepts: [embedded rust, edge device, no_std, hal, interrupt, firmware update, hardware in loop]
root_concepts: [rust-application-blueprints]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Embedded and Edge Device Blueprint

## The Big Picture

```
+============================================================================+
| physical world: sensors | actuators | clocks | buses | power               |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| chip support / HAL / drivers                                               |
| registers -> interrupts/DMA -> typed device capabilities                   |
+-----------------------------------+----------------------------------------+
                                    v
+----------------------------------------------------------------------------+
| application state machine                                                  |
| sample -> validate -> decide -> actuate -> persist/report                  |
+----------------------+----------------------+------------------------------+
                       v                      v
                local durable state       edge/network adapter
                       |                      |
                       +-----------+----------+
                                   v
                    watchdog + telemetry + update/rollback
```

An embedded blueprint begins with hardware, timing, power, memory, and recovery
contracts. `no_std` is an available library boundary, not a complete
architecture. The application still needs explicit state ownership, interrupt
handoff, fault containment, and firmware update semantics.

## Workspace Layout

```
meter-firmware/
|-- Cargo.toml
|-- crates/
|   |-- meter-domain/           # `no_std` policy/state machine
|   |-- meter-protocol/         # bounded wire/storage formats
|   |-- meter-drivers/          # device-specific adapters
|   |-- meter-board/            # pins/clocks/peripherals for one board
|   `-- meter-sim/              # host substitutes
|-- apps/
|   `-- meter-firmware/
|-- memory.x
|-- .cargo/
|   `-- config.toml
`-- tests/
    `-- hardware-in-loop/
```

```toml
[workspace]
resolver = "3"
members = ["crates/*", "apps/*", "tests/*"]

[profile.release]
panic = "abort"
lto = true
codegen-units = 1
```

Those release choices are common for constrained firmware, not universal.
Measure size, latency, debug needs, and update policy before adopting them.

## Hardware and Execution Authority

| Layer | Owns |
|-------|------|
| PAC/chip support | register-level representation |
| HAL | portable peripheral capabilities where available |
| Board support | concrete pins, clocks, memory, peripherals |
| Driver | device protocol and timing |
| Application | domain state machine and safety policy |
| Bootloader/update system | image selection, verification, rollback |
| Watchdog | independent progress/fault boundary |

```
interrupt context
   | capture minimal event / clear source
   v
bounded queue or atomic handoff
   |
   v
application task owns state transition and slower I/O
```

Keep interrupt handlers short and make shared-state rules explicit. Whether the
system uses a polling loop, interrupt-driven executor, RTOS, or async embedded
runtime is secondary to who owns each peripheral and when preemption can occur.

## Memory, Time, Power, and Faults

| Resource | Contract |
|----------|----------|
| Stack | bounded worst-case depth; no accidental large locals |
| Heap | absent, fixed, or deliberately bounded allocator |
| Time | monotonic ticks for deadlines; wall time only if synchronized |
| Power | sleep states, wake sources, state retention |
| Flash | write endurance, atomic record/update layout |
| Network | intermittent connectivity, bounded queues, replay identity |
| Fault | watchdog action, safe actuator state, retained diagnostic |

Security is part of the hardware contract: boot trust roots, firmware signing
keys, debug-port policy, device identity, secret storage, physical reset, and
network capability must have owners. Validate lengths before fixed-buffer
copies and DMA setup; an authenticated packet can still violate resource or
state-machine limits.

For edge messaging, store durable outbound identity before transmission if
redelivery or reconnect can repeat a measurement. Device and cloud clocks may
disagree; preserve both observation identity and relevant time basis.

## Testing and Rollback

```
host tests for pure state machine
       -> driver tests with fake bus
       -> emulator/simulator where representative
       -> hardware-in-loop
       -> power-loss/update/watchdog exercise
```

```text
cargo test -p meter-domain
rustup target add thumbv7em-none-eabihf
cargo build -p meter-firmware --target thumbv7em-none-eabihf
# flash with repository-owned tooling, run HIL assertions, capture artifact hash
```

`thumbv7em-none-eabihf` is an executable example for a Cortex-M4F-class target,
not a universal device choice. Replace it with the repository's pinned target
and probe tools. Toolchains, linker scripts, runners, and hardware revisions are
part of reproducibility.

Safe firmware rollback typically requires:

| Mechanism | Purpose |
|-----------|---------|
| Signed/verified image | reject unauthorized or corrupt code |
| A/B or fallback slot | retain bootable prior image |
| Boot-attempt counter | detect crash loop |
| Health confirmation | mark new image good only after bounded evidence |
| State schema strategy | old firmware can read state or migration is forward-only |

An old image is useless if new firmware irreversibly rewrites persistent state
or peripheral configuration without a compatibility plan.

Security rollback and operational rollback can conflict: a vulnerable signed
image may need to be below an enforced minimum version. Define who may raise the
anti-rollback floor, when fallback remains allowed, and how a failed update is
recovered without reopening revoked firmware.

Device retirement must reach a safe physical state, stop telemetry/commands,
revoke device identity and update credentials, wipe protected state where
required, preserve audit evidence, and remove the device from fleet authority.

## Universal Bridge First

The universal bridge is real-time state-machine engineering: external events
arrive under resource bounds, effects interact with physical authority, and
recovery must reach a safe state. Rust's ownership model can encode peripheral
exclusivity, but it does not replace timing or hardware evidence.

Supplementally, the layering resembles driver/HAL/application separation in
other embedded ecosystems. Managed-device frameworks may provide richer
runtimes; bare-metal Rust commonly requires the repository to own more of the
link, memory, panic, and update contract.

## Decision Cheat Sheet

| Need | Choose |
|------|--------|
| Pure portable control logic | `no_std` core crate with host tests |
| One board/product | board crate plus firmware entry binary |
| Multiple boards | shared application/driver crates, separate board crates |
| Tight interrupt latency | minimal ISR and explicit handoff |
| Intermittent cloud link | durable outbound queue and message identity |
| Safe remote update | verified A/B/fallback flow with health confirmation |
| Rich OS edge gateway | ordinary service/worker blueprint may fit better |

## Common Confusion Points

- **`no_std` does not mean no dependencies.** Dependencies must themselves
  support the target and chosen allocation model.
- **Compilation is not hardware validation.** Clocks, interrupts, DMA, power,
  and electrical behavior need device evidence.
- **Async does not remove interrupt races.** It changes scheduling and wakeup
  structure; peripheral ownership still matters.
- **A watchdog is not a retry loop.** It is an independent recovery boundary and
  needs a safe-state policy.
- **Wall clock is often weak authority.** Use monotonic time for deadlines and
  record synchronization uncertainty.
- **Firmware rollback includes persistent state.** Binary slots alone are
  insufficient.
- **A valid signature does not mean an image remains authorized.** Version
  revocation and anti-rollback policy need an explicit recovery path.

## Primary Sources

- Embedded Rust Book: https://docs.rust-embedded.org/book/
- Embedded Rust Discovery: https://docs.rust-embedded.org/discovery/
- Rust `no_std`: https://docs.rust-embedded.org/book/intro/no-std.html
- Rust platform support: https://doc.rust-lang.org/rustc/platform-support.html
- Rustonomicon: https://doc.rust-lang.org/nomicon/

## Related Guides

- Wasm portability: [10-WEBASSEMBLY-AND-COMPONENT-APPLICATION.md](10-WEBASSEMBLY-AND-COMPONENT-APPLICATION.md)
- Event transport: [07-EVENT-DRIVEN-AND-MESSAGING-APPLICATION.md](07-EVENT-DRIVEN-AND-MESSAGING-APPLICATION.md)
