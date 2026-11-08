# Apperception
A local-first knowledge base and note-taking app, built as a Polylith monorepo in Rust. Code lives as small bricks (components and bases, each a crate) that combine into deployable projects, and the repo keeps its own context so anyone, human or AI agent, can understand why things are the way they are.

## Getting started
You need a Rust toolchain with `clippy` and `rustfmt` (https://rustup.rs installs all three). Then, from this directory:

    git init                                  # if this is not already a git repo
    git config core.hooksPath .githooks       # turn the checks on for this repo
    cargo build                               # build everything, including the `cargo poly` checks

That is the whole setup. Nothing else to install.

## Run it
The example project wires the `api` base, which delegates to the `greeting` component, into a binary:

    cargo run -p hello
    # -> Hello, world!

## Common commands

    cargo test --workspace       # run all tests: unit, interface and doc examples
    cargo clippy --workspace --all-targets -- -D warnings   # lint, warnings are errors
    cargo fmt --all              # format
    cargo poly check             # validate the workspace (brick boundaries, interfaces)
    cargo poly deps              # dependencies between bricks and what each project includes
    cargo poly diff              # which bricks and projects changed since the last stable-* tag
    cargo poly spec status       # what works (spec rules with a citing test) and what is queued
    cargo run -p development     # run your scratch code against the bricks

## Create new bricks
There is no scaffolding command; a brick is a crate and `cargo poly check` tells you when it breaks a rule.

    cargo new --lib components/<name> --name apperception-<name>   # business logic, shared
    cargo new --lib bases/<name> --name apperception-<name>        # a thin entry point
    cargo new --bin projects/<name>                         # a deployable binary

Then, in the new crate's `Cargo.toml`, replace `version` and `edition` with `version.workspace = true`, `edition.workspace = true`, `license.workspace = true`, `publish.workspace = true`, and add `[lints] workspace = true`. For a component or base, add one line to `[workspace.dependencies]` in the root `Cargo.toml`: `apperception-<name> = { path = "components/<name>" }`. Copying `components/greeting` and renaming is just as good.

A brick's public interface is its `src/lib.rs` (an explicit `pub use` list); its implementation is the other modules under `src/`. Keep business logic in components, keep bases thin, and keep a project to a single `main.rs`.

## Build and deploy
A `project` under `projects/` is the deployable unit: a binary crate. Build one with `cargo build --release -p <name>`; the binary lands in `target/release/<name>`. Each project's `Cargo.toml` lists exactly the base and components it needs, so Cargo compiles only those into it.

## Where to read more
- New contributor: `CONTRIBUTING.md` (setup and the day-to-day loop in detail).
- AI agent, or the full process and where every kind of context lives: `AGENTS.md`.
- The decisions behind this setup: `docs/adr/`.

## Layout

    Cargo.toml      the workspace; every crate below is a member
    bases/          thin entry points (one library crate per app or service)
    components/     business logic, shared across projects (one library crate each)
    projects/       deployable binaries (one binary crate each, main.rs only)
    development/    scratch crate for poking at bricks
    tools/poly/     the `cargo poly` checks that keep the process honest
    docs/spec/      the behavior contract: one file per product domain, numbered rules, OPEN: questions
    docs/queue.yaml the build queue: which rules to implement next, in order (agent-maintained)
    docs/adr/       the decision log (and a generated per-brick index)

## Example bricks
`components/greeting`, `bases/api` and `projects/hello` are examples that demonstrate the interface/implementation split and the composition root, and `docs/spec/greeting.md` is the matching example spec domain whose two rules the greeting tests cite. Delete all four once you have your own (see CONTRIBUTING.md).

## Namespace
Every component and base is a crate named `apperception-<brick>`; the namespace is set in `Cargo.toml` under `[workspace.metadata.polylith]`. To use your own, change it there and rename the crates in `components/`, `bases/` and `[workspace.dependencies]` consistently. `cargo poly check` reports any brick whose name does not match.
