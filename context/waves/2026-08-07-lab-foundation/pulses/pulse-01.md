# Pulse 01: Research Foundation

## Goal

Create the FERRIUM research, governance, and review foundation without product
code.

## Changes

- Add README, product plan, wave, and pulse records.
- Add repo-specific wave, pulse, and research skills.
- Add repository-local review roles.
- Record the research gate that must pass before implementation begins.

## Validation

- `git grep -n "FERRIUM" -- README.md PRODUCT_PLAN.md context/waves/PHASES.md`
- `git grep -n "Writing product code\\|implementation package\\|research gate" -- README.md PRODUCT_PLAN.md context/waves`
- `git diff --check`

## Status

Complete.
