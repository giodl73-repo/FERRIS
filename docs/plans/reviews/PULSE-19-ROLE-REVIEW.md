# Pulse 19 Nine-Role Cargo Preservation Review

Date: 2026-08-12
Pulse: Ordinary Cargo Preservation Control
Disposition: Accepted as a representative development control
Implementation authority: Consumer fixture, conformance test, and evidence only

## Review question

Does the new owner-native before-and-after control establish a precise
ordinary-Cargo preservation result for one representative consumer without
creating a Cargo adapter, universal removal claim, support claim, or
PLATFORM-001 advancement?

## Role dispositions

### Rust Safety Steward

**Disposition:** Accept.

The consumer uses safe Rust and one deterministic unit test. Compiler and test
success are not promoted into broader behavioral or safety proof.

### Compiler Performance Engineer

**Disposition:** Accept.

Separate external target directories prevent the after result from depending
on the before build cache. No timing or performance comparison is claimed.

### Interop Boundary Auditor

**Disposition:** Accept with scope limitation.

The zero-dependency library has no ABI, WIT, wire, native, provider, or
deployment boundary. Those cases remain explicitly outside this control.

### AI Assurance Skeptic

**Disposition:** Accept.

Exact cutoff, commands, environments, owner results, snapshot assertions, and
omitted domains are visible. One passing control is not presented as universal
Cargo or lifecycle evidence.

### Ecosystem Strategist

**Disposition:** Accept.

Cargo remains the only owner of metadata, lock, compilation, and unit-test
behavior. Ferris neither invokes nor replaces Cargo and introduces no hidden
resolver or registry.

### Rust Maintainer

**Disposition:** Accept.

The test uses ordinary locked offline Cargo commands, keeps build output
outside the consumer workspace, verifies exact metadata equality, and leaves
source and lock state unchanged.

### Native Platform Adopter

**Disposition:** Accept with limitation.

Windows and Ubuntu 24.04.4 WSL2 both pass, but the fixture has no native tools,
cross targets, packaging, deployment, operations, or recovery behavior.

### Scope Keeper

**Disposition:** Accept.

The pulse adds a fixture, test, and evidence only. It does not add product
behavior, a Cargo adapter, profile generation, mutation, hidden evaluation,
or specification advancement.

### Validation Checker

**Disposition:** Accept.

The integration path records owner metadata and unit behavior before and
after Ferris, uses separate cold target directories, compares the full
consumer snapshot after every step, and passes the 65-test suite on both
recorded platforms.

## Remaining gates

- This is one zero-dependency single-package control.
- Registry, dependency, workspace, feature, build-script, macro, generated,
  unsafe, native, target, provider, failure, and deployment cases are absent.
- Actual profile adoption, automation removal, rollback, and retained
  historical evidence are not exercised.
- No PRODUCT-001 Removal Record or PLATFORM-001 lifecycle gate is complete.
- The independent Pulse 16 held-out package and score remain outstanding.
- No support, compatibility, security, freshness, approval, certification, or
  production claim is authorized.

## Decision and authority

All nine roles accept Pulse 19 as representative ordinary-Cargo preservation
evidence only. The review grants no Cargo-adapter, lifecycle, support, or
held-out authority.
