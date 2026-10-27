# terraform-dia

Rust CLI that parses Terraform files and generates opinionated infrastructure diagrams. See README.md for the idea.

## Scope (current)
- Provider: AWS only.
- Output: JSON only (raw model dump). Diagram formats are not decided yet — don't add them.
- Parsing: static HCL parsing of `.tf` files (no `terraform init`/plan).

## Structure
Cargo workspace:
- `crates/tfdia-core` — parser, model, rules. No filesystem IO (keeps a future WASM build possible).
- `crates/tfdia-cli` — `clap` CLI, file reading/writing.
- `tests/` — integration test crate (`tfdia-tests`). `tests/fixtures/<name>/` holds sample Terraform projects; each one is snapshot-tested. Add a fixture for every new Terraform feature supported.

## Commands
- Build: `cargo build`
- Test: `cargo test` (snapshots: `cargo insta review`)
- Lint: `cargo clippy --all-targets -- -D warnings`
- Format: `cargo fmt`

Run fmt, clippy and tests before considering a task done.

## How to work with me
- I'm learning Rust (intermediate level, strong in Go).
- Mixed mode: you write boilerplate/infra (CLI wiring, CI, tests setup, serde structs). For core logic (parser, model, rules) guide me with hints and review my code instead of writing it, unless I ask.
- Explain non-obvious Rust idioms briefly when you use them; Go comparisons are welcome.
- Keep answers short.
