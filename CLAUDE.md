# claude-from-zero

Companion repo for the "Claude from Zero" course. Trivial Rust demos
showing skills + sub-agents + contracts + pmat working together.

## Hard rules

- **Provable contracts.** Every demo `main` ends with an `assert!`
  that proves a named invariant from the top-level docstring, then
  a `println!("contract: ... OK")` so a screencast shows the proof.
- **No hand-written JavaScript.** Ever. This is a Rust teaching repo.
- **pmat query replaces grep.** See global `~/.claude/CLAUDE.md` for
  the full decision tree.

## Layout

| Path | Purpose |
|------|---------|
| `m2-hello/` | M2 demo: first Rust fn with a provable contract |
| `m3-lint/` | M3 demo: crate with a planted clippy warning for `/lint-demo` |
| `m4-review/` | M4 demo: target binary for parallel sub-agent review |
| `.claude/commands/` | Skills — one per module that has a skill demo |
| `contracts/` | aprender-format YAML kernel contracts validated by `pv` |

## Commands

```bash
cargo build --workspace        # compile everything
cargo test --workspace         # run tests
cargo clippy --workspace       # lint
cargo run -p m2-hello          # prove the M2 contract
pv validate contracts/square-kernel-v1.yaml   # M4 gate
```

## Quality gates

- `cargo fmt --all -- --check` — formatting
- `cargo clippy --workspace --all-targets -- -D warnings` — lint
  (the M3 demo is excluded — its planted warning is the lesson)
- `cargo test --workspace` — tests
- `pmat comply` — compliance suite (run before commit)

## Conventions

- **Provable contract discipline.** Every demo binary carries a
  `Provable contract:` line in its top-level docstring and closes
  `main` with a matching `assert!`. If a fix removes the assertion,
  the demo is no longer finished.
- **Skill body wraps a CLI.** Skills in `.claude/commands/` shell
  out to a Rust binary (here: `cargo clippy`) and shape the output;
  they never reimplement logic in the skill prompt.
- **Contract-gated sub-agents.** When delegating to a sub-agent,
  pass the relevant `contracts/*.yaml` in the prompt and require
  the sub-agent to run `pv validate` on the contract before
  returning a verdict. A verdict that contradicts a documented
  invariant is rejected by the parent.
- **`pv` is the provable-contracts CLI.** Contracts in `contracts/`
  are authored in the aprender kernel-contract YAML format and
  validated with `pv validate`. Never hand-roll a parallel YAML
  validator — extend the contract and re-run `pv` instead.
