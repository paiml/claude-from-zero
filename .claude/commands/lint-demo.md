---
name: lint-demo
description: Run cargo clippy on a workspace crate and report the top three warnings by severity
argument-hint: <crate-name>
---

# /lint-demo — cargo clippy wrapped as a skill

This skill demonstrates the "skills that wrap a CLI" pattern from
Module 3 of the Claude from Zero course.

## Arguments

`$ARGUMENTS` should be one token: the crate name to lint, e.g. `m3-lint`.
If empty, lint the whole workspace.

## Instructions

1. Run clippy:

   ```bash
   cargo clippy -p <crate> --all-targets --message-format=short 2>&1 | tee /tmp/clippy-out.txt
   ```

   If `$ARGUMENTS` is empty, run `cargo clippy --workspace --all-targets
   --message-format=short` instead.

2. Read `/tmp/clippy-out.txt`.

3. Report to the user:
   - How many warnings clippy found, split by severity (`warning`,
     `error`) if both appear.
   - The top three warnings in the form
     `path:line:col — lint-name — brief rephrase`.
   - The exit status of the cargo process (`success` or `non-zero`).

4. Do **not** attempt to fix the warnings — that's a separate skill.
   This skill is read-only by design so the learner sees the raw
   clippy output shaped into a report.

## Why this pattern

- Pure computation (lexing, type analysis, lint rule execution) lives
  in `cargo clippy` — the Rust binary. Fast, deterministic, reusable
  from the shell.
- Orchestration and presentation live in this skill body — the agent
  chooses how to rank, how to rephrase, how to surface only the three
  most important findings.
- Errors propagate as structured text, not silently swallowed.
