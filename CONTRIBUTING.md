# Contributing
This repository keeps its own context: why things are the way they are, what each part does, and how to work on it. A few small automated checks keep that context honest so the codebase stays understandable as it grows.

You do not need to memorize how it all works. The checks guide you, and the AI agent does most of the paperwork (tests, spec lines, queue updates, and the ADRs you ask for) for you. Your job is mostly to follow the setup below once, then answer the agent's questions as you work.

If you want the full reasoning, read `AGENTS.md`. This file is just how to get set up and what to do day to day.

## One-time setup (do this once after cloning)
1. **Install Rust** if you have not. Follow https://rustup.rs; it installs `cargo`, `clippy` and `rustfmt` together.

2. **Turn the checks on for this repo:**

       git config core.hooksPath .githooks

   This tells git to run `.githooks/pre-commit` every time you commit. It is a short shell script you can read.

3. **Build everything once:**

       cargo build

   This compiles the bricks and the `cargo poly` checks the hook uses.

That is it. From now on, the checks run automatically every time you commit.
To run every check by hand at any time:

    .githooks/pre-commit

## The everyday loop
1. **Edit your code.** Ask the agent for help; it knows the rules in `AGENTS.md`.
2. **Commit.** When you run `git commit`, the checks run first. If something is wrong, the commit stops and tells you what to fix. Fix it and commit again.
3. **Push** your branch to GitHub.
4. **Open a pull request.** GitHub runs the same checks on its servers, plus the tests and a dependency audit.
5. **Merge** once the checks are green. With branch protection on (see below), the button stays locked until they pass.

The checks on your machine are the fast warning. The checks on GitHub are the real gate. Same checks, two moments. Tests run only on GitHub to keep commits quick; run `cargo test --workspace` yourself before you push.

## When a check stops you, here is what it means
You do not need to understand the internals. Read the message, do the fix, try again.

- **"Format" failed.** Run `cargo fmt --all` and commit again.

- **"Polylith check" failed.** A brick boundary was crossed (usually a component depending on a base), a crate is misnamed, a project has more than a `main.rs`, or a `lib.rs` exposes a whole module. The message names the problem and the file. Ask the agent to help move the code to the right place.

- **"Lint" (clippy) failed.** A warning was found and warnings are errors here. The message names the file, line and what to change; often it suggests the fix. Ask the agent to fix it, or fix it yourself and commit again.

- **Tests failed (on GitHub).** One or more tests, or a doc example, are failing. The output names the file and test. Fix the code or the test, then push again.

- **"ADR front matter lint" failed.** A decision record under `docs/adr/` is missing a field or has an invalid value. The message says which file and which field. Copy `docs/adr/0000-adr-template.md` if you are starting one from scratch, or ask the agent to fix the fields. A warning that a brick named in an old ADR no longer exists is fine; it does not block.

- **"Interface change heads-up" (local).** You changed a brick's public `src/lib.rs`. This is a warning, not a block. If it is a real change to what other code depends on, ask the agent to draft an ADR for it. You will need one before the pull request can merge.

- **"Interface change must be recorded" (on GitHub) failed.** Same situation, but now it blocks the merge. Do one of: add an ADR whose `affects` names the brick (best, the agent can draft it), mention an ADR in a commit message like `ADR-NNNN: ...`, or if the edit was not really a contract change (a comment or formatting), add `[interface-impact: none]` to a commit message. Note: `[interface-impact: none]` is self-reported; code review is the backstop against misuse.

- **"Spec and queue lint" failed.** A product spec file under `docs/spec/` has the wrong headings or rule numbering, a test cites a rule that does not exist, a link under `docs/` is broken, or `docs/queue.yaml` (the agent's build queue) is malformed. The message names the file and line. Ask the agent to fix it; you do not need to read the queue yourself.

- **"Refresh the ADR index" ran.** You staged a decision record, so the hook regenerated the index under `docs/adr/index/` and staged it for you. Nothing to do. (This only runs locally; CI does not enforce index freshness to avoid merge conflicts on parallel branches.)

- **"Build" failed on GitHub with a lockfile message.** `Cargo.lock` is out of date. Run `cargo build` locally, commit the updated `Cargo.lock`, and push again.

- **"Known vulnerabilities" failed on GitHub.** A dependency has a published security advisory. The output names it and the fixed version. Update the dependency (usually `cargo update -p <crate>`) and push again.

## If you are setting up the repository (admin, one time)
Before anything else, replace the placeholders left by the starter:
- **Namespace**: change `apperception` in `Cargo.toml` under `[workspace.metadata.polylith]`, rename the crates in `components/` and `bases/` (their `name` in `Cargo.toml`), and update `[workspace.dependencies]` and the `use` lines to match. `cargo poly check` reports anything left inconsistent.
- **ADR dates**: fill in the `date:` field in each ADR under `docs/adr/` with the date you are formally adopting the decision.
- **ADR decision-makers**: replace `[you]` in each ADR's front matter with the actual names or roles.
- **Example bricks**: delete `components/greeting`, `bases/api` and `projects/hello`, the matching example spec domain `docs/spec/greeting.md`, and the two lines under `[workspace.dependencies]` in `Cargo.toml`, once you have your own bricks. The `.keep` files keep the empty directories in git; Cargo needs them to exist.
- **Inherited tags**: if you cloned or forked this repository instead of using GitHub's **Use this template** button, delete the template's own baseline tags. Otherwise `cargo poly diff` compares your bricks against the template's baseline instead of your own.

      git tag -d $(git tag -l 'stable-*')

Then, make the checks a hard gate by configuring GitHub branch protection:
1. Go to **Settings > Branches**.
2. Under **Branch protection rules**, click **Add rule**.
3. Branch name pattern: `main`.
4. Tick **Require a pull request before merging**.
5. Tick **Require status checks to pass before merging**, then select the **checks** workflow.
6. Click **Create** (or **Save changes**).

Now no change can reach `main` without passing the checks, no matter who or what made it. This is the piece that actually enforces the process; everything else is guidance that makes following it easy.

## Why all of this
The short version: context that lives next to the code and stays accurate is worth far more than docs that drift. The checks stop the context from drifting. The full reasoning, and the map of where every kind of context lives, is in `AGENTS.md`. The decisions behind the setup are in `docs/adr/`.
