# Workspace checks: `cargo poly`
A small crate that enforces the development process described in `AGENTS.md`. The alias in `.cargo/config.toml` makes it `cargo poly <command>`, so it needs nothing installed beyond `cargo` and `git`. The pre-commit hook (`.githooks/pre-commit`) and CI (`.github/workflows/checks.yml`) run it automatically, and you can run any command by hand from anywhere inside the workspace.

| Command | What it does | Blocks? |
|---------|--------------|---------|
| `cargo poly check` | Validates the Polylith layout Cargo cannot check by itself: every member is under `components/`, `bases/`, `projects/`, `development/` or `tools/`; a brick is a library crate named `<namespace>-<directory>` with no binary target; a brick's `src/lib.rs` has no `pub mod` and no glob re-export; components never depend on bases and bases never on bases; a project is a binary crate with only `src/main.rs`, depending on exactly one base; nothing depends on a project, the development crate or a tool. | Yes, locally and in CI |
| `cargo poly deps` | Prints each brick's dependencies on other bricks, what each project includes transitively, and who depends on each brick. | Never |
| `cargo poly diff [--since REF]` | Lists the bricks and projects with changes since the latest `stable-*` tag (or `REF`), including uncommitted and untracked files, and the projects that reach a changed brick. | Never |
| `cargo poly interface --mode pre-commit` | Warns when a staged change touches a brick's `src/lib.rs` without a staged ADR whose `affects` names that brick. | No |
| `cargo poly interface --mode ci` | Same check against the pull request's base branch. Fails unless each changed brick is named by an ADR in the pull request, or a commit message says `ADR-NNNN` or `[interface-impact: none]`. | CI: yes |
| `cargo poly adr lint` | Validates every ADR's filename, front matter, status and `interface-impact`, and rejects duplicate numbers. Warns, without failing, when a name in `affects` does not exist now. | Yes, locally and in CI |
| `cargo poly adr index` | Regenerates `docs/adr/index/`, a per-brick view of which ADRs affect which brick. | Refreshes files locally when an ADR is staged; not enforced in CI (avoids merge conflicts on parallel branches) |
| `cargo poly spec lint` | Validates `docs/spec/`: five headings per domain in order, contiguous rule numbers, every `<domain> rule N` citation in a brick's `tests/` and in `docs/spec/` resolves, no glossary term redefined, links under `docs/` resolve. Validates `docs/queue.yaml`: shape, unique ids, rule ranges naming a real domain, `after` naming an earlier step. | Yes, locally and in CI |
| `cargo poly spec status` | Derives what works and what is next: per domain, rules with a citing test and open questions; the queue with each step's state (waiting on spec, ready, in progress, done). A SessionStart hook prints it into the agent's context. | Never |
| `cargo poly remind` | Prints a pointer to the pre-commit checklist in `AGENTS.md`. Runs last in every hook run. | Never |

External tools the hook and CI run alongside it: `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`, `cargo build --locked` (CI) and `cargo audit` (CI, via the RustSec action).

## How it reads the workspace
`check`, `deps`, `diff` and `interface` run `cargo metadata --no-deps` and classify each package by its directory. A brick's short name is its directory name; that is the name ADRs use in `affects` and the lint checks against. The namespace comes from `[workspace.metadata.polylith]` in the root `Cargo.toml`.

## Editing a check
These are intentionally simple (Best Simple System for Now). Each command is one module under `src/`. If you change what a check enforces, update `AGENTS.md` and `CONTRIBUTING.md` to match, and consider whether the change deserves an ADR.
