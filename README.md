# claude-from-zero

Companion repository for the **Claude from Zero** Coursera course —
course 9 of the [Rust for Data Engineering](https://www.coursera.org/specializations/rust-for-data-engineering)
specialization.

Trivial Rust demos that wire together the four pillars of the
production Claude workflow:

- **Claude Code** — the agent in your terminal
- **Skills** — reusable slash commands (`.claude/commands/`)
- **Sub-agents** — parallel delegated work via the Agent tool
- **Provable contracts + pmat** — the reliability layer

## Demos

| Crate | Course module | What it teaches |
|-------|---------------|-----------------|
| `m2-hello` | M2 · Claude Code fundamentals | First Rust function with a named provable contract, bootstrapped from `CLAUDE.md` |
| `m3-lint` | M3 · Skills (slash commands) | A crate with an intentional clippy warning that the `/lint-demo` skill catches |
| `m4-review` | M4 · Sub-agents and reliability | A small binary reviewed by two parallel sub-agents gated by `contracts/review-finding.yaml` |

Module 5 teaches `pmat query`, `pmat comply`, and `pmat hooks` applied
to this whole workspace — no extra crate needed; the repo itself is
the final demo.

## Quick start

```bash
# Compile all demos
cargo build --workspace

# Run the M2 demo's provable contract
cargo run -p m2-hello

# Lint the M3 demo (clippy finds the planted warning)
cargo clippy -p m3-lint

# Review the M4 target against the contract schema
cat contracts/review-finding.yaml

# Validate a sub-agent's YAML review against the runtime contract
cargo run -p m4-review --bin validate-finding -- \
    contracts/review-finding.valid.yaml       # exit 0, contract-OK marker
cargo run -p m4-review --bin validate-finding -- \
    contracts/review-finding.invalid.yaml     # exit 2, invariant violation
```

## Go deeper

For real data-engineering work with this toolkit, continue with the
rest of the
[Rust for Data Engineering](https://www.coursera.org/specializations/rust-for-data-engineering)
specialization.
