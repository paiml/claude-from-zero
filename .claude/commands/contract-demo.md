---
name: contract-demo
description: Run pv validate + status on a provable-contracts YAML and report the quality gate
argument-hint: <contract-path>
---

# /contract-demo — pv wrapped as a skill

This skill demonstrates the "provable contracts as the sub-agent reliability
gate" pattern from Module 4 of the Claude from Zero course. It wraps
`pv` (provable-contracts) — the Rust CLI from
[aprender-contracts](https://github.com/paiml/aprender) — so a learner
can validate and inspect a kernel contract without memorising
subcommand flags.

## Arguments

`$ARGUMENTS` should be one token: the path to a YAML contract,
e.g. `contracts/square-kernel-v1.yaml`. If empty, default to
`contracts/square-kernel-v1.yaml`.

## Instructions

1. Resolve the contract path. If `$ARGUMENTS` is empty, use
   `contracts/square-kernel-v1.yaml`.

2. Run validation:

   ```bash
   pv validate <contract-path> 2>&1 | tee /tmp/pv-validate.txt
   ```

3. Run the status summary:

   ```bash
   pv status <contract-path> 2>&1 | tee /tmp/pv-status.txt
   ```

4. Read both `/tmp/pv-validate.txt` and `/tmp/pv-status.txt`.

5. Report to the user:
   - The contract description (first sentence from `pv status`).
   - Validation verdict: `valid` or the specific error(s) `pv` emits.
   - Counts of equations, proof obligations, falsification tests, and
     Kani harnesses.
   - The QA gate id (e.g. `F-SQ-001`).
   - If there are **more** proof obligations than falsification tests,
     flag it — `pv` enforces `tests >= obligations`.

6. Do **not** run `pv scaffold`, `pv codegen`, or `pv kani` from this
   skill. Those write files and are a separate skill by design. This
   skill is read-only so the learner sees the gate without side
   effects.

## Why this pattern

- The formal gate (YAML parse, obligation/test count check, QA gate
  lookup) is pure computation — `pv` owns it. Fast, deterministic,
  reusable from the shell or CI.
- Sub-agents invoke this skill before approving a diff that touches
  any function named in the contract. A sub-agent whose verdict
  contradicts `pv validate` is rejected by the parent.
- Contract YAML is the single source of truth; the Rust
  implementation and its unit tests restate invariants, but `pv` is
  what proves the restated set is closed and complete.
