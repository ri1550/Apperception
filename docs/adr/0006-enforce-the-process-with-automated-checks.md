---
status: accepted
date: 2026-11-08
decision-makers: Owen Robert McGregor
consulted: []
informed: []
affects:
  components: ["*"]
  bases: ["*"]
  projects: ["*"]
interface-impact: none
---

# ADR-0006: Enforce the context system with automated checks
## Context and problem statement
ADR-0005 records how context lives in the codebase, and `AGENTS.md` describes the process for keeping it accurate. But a written process that relies on people, and on AI agents, remembering to follow it will drift, especially with junior or fast-moving contributors. We need the context system to be enforced, not just described, and we need the local half of that enforcement to work with nothing installed beyond the Rust toolchain and git.

## Decision drivers
- The process must hold even when the contributor does not know or recall it.
- Feedback should arrive fast, ideally before a bad change is even committed.
- There must be a gate that cannot be skipped, to actually protect `main`.
- No extra tool to install: a Rust contributor has `cargo` and `git` and nothing else should be assumed.
- Best Simple System for Now: the least machinery that makes the correct path the easy one and blocks the wrong one.

## Considered options
- Rely on code review alone
- Rely on `AGENTS.md` and the agent's good behavior alone
- Local hooks only; CI only; both layers (local hooks plus CI gated by branch protection)
- For the local layer: the pre-commit framework (as in the Python template), lefthook, cargo-husky, or a plain tracked git hook
- For the checks themselves: Python scripts as in the Python template, shell scripts, or a Rust tool crate in the workspace

## Decision outcome
Chosen option: both layers, with a plain git hook locally and the checks written as a Rust tool in the workspace.

The local layer is `.githooks/pre-commit`, a tracked shell script turned on once with `git config core.hooksPath .githooks`. It gives fast feedback at the keyboard: a blocked commit or a warning. CI (`.github/workflows/checks.yml`) re-runs the same checks on GitHub, and branch protection makes passing them a requirement to merge.

The checks are subcommands of `cargo poly`, a small crate in `tools/poly/` reached through a cargo alias, so they need nothing but `cargo`: `adr lint` (decision log front matter and numbering), `adr index` (the per-brick index), `interface` (a changed public `lib.rs` must come with a recorded decision), `spec lint` and `spec status` (ADR-0008), and `remind` (prints a pointer to the pre-commit checklist in `AGENTS.md` at the end of every hook run so the agent sees it in context before the commit lands). Code quality checks follow the same two-layer approach and are covered in ADR-0007.

Two refinements over the Python template, learned from applying it:
- The interface gate is per brick. For each brick whose `src/lib.rs` changed, the pull request must contain an ADR whose `affects` names that brick (or `"*"`), or a commit message must say `ADR-NNNN` or `[interface-impact: none]`. Any ADR in the pull request no longer satisfies the check for every brick.
- `adr lint` warns rather than fails when a name in `affects` no longer exists. Records are immutable and outlive the bricks they name; failing forever on a deleted brick would push people to edit history.

Both layers over the alternatives because review and good intentions do not scale and are exactly what fails with juniors; local hooks alone can be bypassed (`--no-verify`) and live only on each machine; CI alone gives slow feedback and lets a broken change get committed and pushed before catching it. A plain hook over the pre-commit framework, lefthook and cargo-husky because each of those is an install or a build-time side effect, and the hook is thirty lines of shell calling `cargo`. A Rust tool over Python scripts because a Rust repository should not require Python and PyYAML on every machine and runner; over shell because parsing Cargo metadata, YAML front matter and the queue in shell is fragile.

### Consequences
- Good: the context system is enforced regardless of who or what makes a change; the hard gate (branch protection) cannot be skipped.
- Good: fast local feedback reduces failed CI runs, and nothing beyond `cargo` and `git` is needed to get it.
- Bad: one command per clone (`git config core.hooksPath .githooks`) and one repository setting (branch protection) that an admin must enable. If branch protection is not enabled, CI advises but does not block.
- Bad: a plain hook runs against the working tree, not the staged snapshot, so unstaged edits can affect what it checks. Acceptable; CI checks the real thing.
- Neutral: the interface-change check is heuristic. The `[interface-impact: none]` and `ADR-NNNN` escapes are self-reported; code review is the backstop against misuse.

### Confirmation
The `checks` workflow runs in CI on every pull request and is required by branch protection on `main`. The checks enforce themselves: a misconfiguration shows up as a failing run. `cargo poly adr lint` validates every ADR's filename, front matter and field values and rejects duplicate numbers. `cargo poly adr index` keeps `docs/adr/index/` current locally; it runs in the hook whenever an ADR is staged but is not enforced in CI, to avoid merge conflicts on parallel branches. `cargo poly interface --mode ci` blocks a merge that changes a public `src/lib.rs` without a recorded decision for that brick. `cargo poly remind` runs last in every hook run. `CONTRIBUTING.md` documents the per-contributor and admin setup.

## More information
- ADR-0005 (the context system this enforces)
- ADR-0007 (code quality checks that use the same two-layer approach)
- `CONTRIBUTING.md` (setup and day-to-day workflow)
- `tools/poly/README.md` (what each check does)
- `AGENTS.md` (the process these checks enforce)
- git hooks: https://git-scm.com/docs/githooks
- Best Simple System for Now (BSSN): https://dannorth.net/blog/best-simple-system-for-now/
