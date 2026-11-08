# AGENTS.md
## What this is
This is a Polylith workspace in Rust. This file governs how context about the codebase (why things are the way they are, what each part does, how to work on it) lives in the codebase itself, and how you, the agent, read it before editing and write it after.

The guiding idea: a brick's full context should always be a few cheap reads, regardless of how large the codebase grows. That holds only because each kind of context has exactly one home and is reached by query or by sitting next to the code, never by reading piles, and because the content in those homes is kept honest by discipline and review. The automated checks enforce structure, not substance. Your job on the write side is distillation, not capture.

Layout:

    Cargo.toml        the workspace: every brick, project and tool is a member crate
    bases/            thin entry points (one library crate per app or service)
    components/       business logic, shared across projects (one library crate each)
    projects/         deployable binaries (one binary crate each; a main.rs and nothing else)
    development/      scratch crate, the stand-in for a REPL (`cargo run -p development`)
    tools/poly/       the `cargo poly` checks that keep the process honest
    docs/spec/        the behavior contract: one file per product domain, plus the glossary
    docs/queue.yaml   the build queue: which spec rules to implement next, in order (agent-maintained)
    docs/adr/         the decision log (and a generated per-brick index)

Each brick is `components/<name>/` or `bases/<name>/` with `Cargo.toml`, `src/lib.rs` (the interface), other modules under `src/` (the implementation) and `tests/` (the interface tests).

## Polylith rules
- Bricks are `components` and `bases`. Components hold business logic and are shared across projects. Bases are thin entry points to an app or service. Both are library crates named `<namespace>-<name>` (the namespace is in `Cargo.toml` under `[workspace.metadata.polylith]`).
- Components must not depend on bases, and bases must not depend on bases. Dependency flows base -> components, never the reverse, so components stay reusable. Enforced by `cargo poly check`.
- A brick's `src/lib.rs` is its public interface: private `mod` lines and an explicit `pub use` list. No `pub mod`, no `pub use x::*`. Enforced by `cargo poly check`.
- Projects (`projects/`) are binary crates that combine one base with components into a deployable artifact. A project's `src/` holds only `main.rs`, the composition root, and no business logic. Enforced by `cargo poly check`.
- Every brick is declared once in `[workspace.dependencies]`; a consumer writes `apperception-<name>.workspace = true`.
- The whole workspace builds, tests and lints as one unit (`cargo build`, `cargo test --workspace`, `cargo clippy --workspace`). `development/` is where you poke at bricks without a project.

## Guiding principle: Best Simple System for Now (BSSN)
Build the simplest thing that meets the need right now, written to an appropriate standard, with no speculative future-proofing. See ADR-0003 for the rationale.

In practice:
- Do not record a decision you have not made. No ADR for a deployment shape, tool, or policy that is still open. Polylith lets you defer these on purpose. An undecided product question is an `OPEN:` line in its spec domain; an undecided architectural choice that no spec domain needs answered yet is written nowhere.
- Do not add a layer where the code is self-evident. A trivial brick needs no prose beyond a one-line crate doc; an obvious change needs no ADR.
- Do not future-proof the context itself: no speculative fields, no per-brick scaffolding "just in case." Add structure when a real need appears, not before.
- When something can be taken away and the system still works for now, take it away. That applies to ADRs and prose as much as to code.

## The context layers
Seven homes, each owning one kind of context:

| Layer                         | Owns                                              | Mutable? | Enforced? |
|-------------------------------|---------------------------------------------------|----------|-----------|
| `docs/spec/<domain>.md`       | What the product intends to do: numbered behavior rules and `OPEN:` questions, before and alongside the code | yes; rule numbers permanent | `cargo poly spec lint`; `cargo test` via rule citations |
| `src/lib.rs`                  | The brick's public interface (its surface)        | yes      | the compiler; `cargo poly check` |
| Tests (`tests/` per brick)    | How the brick behaves and how to call it; which spec rules it pins | yes      | CI |
| Crate docs (`//!` in `lib.rs`, `///` on public items) | Why the brick is shaped this way: intent, invariants; usage as runnable doc examples | yes | doc examples: `cargo test`; prose: no |
| `docs/queue.yaml`             | Which spec rules to build next, in what order. Agent-only; the developer hears it, they do not read it | freely rewritten; finished steps deleted | `cargo poly spec lint` |
| `docs/adr/`                   | Architectural decisions that shape the project's direction, with alternatives and rationale | append-only | `cargo poly adr lint` (front matter) |
| Commit message / PR body      | What a specific change did                         | immutable | no |

Anything that doesn't belong to one of these layers does not go in the codebase. What works today is deliberately not a layer: it is the set of spec rules with a passing test citing them, and `cargo poly spec status` derives it.

## Reading a brick before editing
Load the full context in this order before touching any brick:
0. **What the product intends the brick to do:** its spec domain, `docs/spec/<domain>.md`. Code implements numbered rules from there and nothing else; an `OPEN:` line is not buildable. If no domain covers the brick, that is a question for the developer, not a guess.
1. **What the brick exposes (its public surface):** `src/lib.rs`. Everything `pub use`d there is the contract; the compiler hides the rest from other crates, so it cannot drift.
2. **How it behaves and how to call it:** the brick's `tests/`. They are working examples guaranteed current, because CI fails the moment code and test disagree, and they can only see the public surface. Each test cites the rule it pins as `<domain> rule N`.
3. **Why it is shaped this way (intent, invariants, non-obvious constraints):** the crate docs at the top of `src/lib.rs`. The one prose layer; carries only what a test cannot assert. Doc examples there are run by `cargo test`, so they are current too.
4. **Why a decision was made (with alternatives):** `docs/adr/`, filtered to the brick rather than read whole (see below).

Five reads, all current, at any codebase size. For what a specific change did, also check the commit message or PR description.

### Finding the decisions that touch a brick
Do not read the whole ADR log. Query it by the `affects` field:

    grep -rl "<brick_name>" docs/adr/

This returns the handful of decisions touching that brick out of however many total. Or read the prebuilt `docs/adr/index/<kind>-<brick_name>.md`, which `cargo poly adr index` keeps current.

## The spec: docs/spec/
The spec is the behavior contract, one file per product domain, in product language. Rules are numbered sentences a test could assert, and the numbers are permanent. Unsettled questions are `- OPEN:` lines, so `grep -rn "^- OPEN:" docs/spec/` lists everything undecided in the product. A test pins a rule by citing it as `<domain> rule N` in a comment or doc comment.

The one distinction that keeps the spec a contract rather than a draft: rules come from the developer, and anything you derive on your own is an `OPEN:` line. Adding an `OPEN:` line is the one spec edit you make without asking. Every other edit (a new rule, a reworded rule, a glossary entry, a new domain) you draft, show, and land on confirmation. The lifecycles, the five-heading shape, the citation grammar, and the shared-term process live in `docs/spec/README.md`, not here.

## The queue: docs/queue.yaml
The queue holds the one thing no other layer holds: order. Each step is a range of spec rules to implement in one brick, with an optional `after` dependency. Everything else is derived, never written. A step is waiting on spec when it cites rules the domain does not have yet, ready when they exist, in progress when some have a citing test, and done when all do. A done step is deleted. `cargo poly spec status` prints all of this.

You do not read the queue by hand. A SessionStart hook (`.claude/settings.json`) prints `cargo poly spec status` into your context at every session start. If no `[spec]` summary appeared, the hook did not run; run `cargo poly spec status` yourself before doing anything else. Open every session by saying, in one line, what the queue's head is and which domains have open questions it is waiting on. Then read the spec domain of the step you are about to work.

Edit the YAML directly; `cargo poly spec lint` validates it at commit time. A step exists only because the developer asked for the work or a queued step depends on it. Nothing speculative. Chores that cite no rule do not go in the queue: do them now, drop them, or open an issue. Dead ends, sub-steps, and in-flight notes are session material: Claude Code's plan mode holds them while you work, the commit message holds what survives, and nothing else is written down.

## Routing what you learned
How to route what you learned this session into the codebase, and what to discard. Most session material is not durable; if a thing does not clearly match one of these, discard it rather than inventing a home for it.

- **Hit a product question the spec does not answer, or a choice with real alternatives that is not yet decided?** An `OPEN:` line in the spec domain, naming the alternatives when it is a choice. Then stop building what it covers. Do not draft or offer an ADR; see "Decisions" below.
- **The developer settled what the product should do?** A numbered rule in the spec domain, appended with the next number, confirmed by the developer, with the `OPEN:` line it resolves deleted in the same change.
- **The developer asked for an ADR?** Draft it from the template and confirm it. If the choice changes behavior, the test that pins the new behavior is part of recording it. Only the developer starts an ADR.
- **Changed a brick's public surface?** The `src/lib.rs` change itself, a test asserting the new contract behaves as intended, and a crate-doc update if the prose needs it. The interface-change check will also want an ADR naming the brick or an `[interface-impact: none]` justification; tell the developer and let them choose.
- **Established or changed how a brick is supposed to behave** (an edge case, an input/output guarantee, a regression you just fixed)? A test in the brick's `tests/`, citing the spec rule it pins. This is the executable half of "how it works," and it is preferred over prose whenever the behavior can be asserted.
- **Learned how a brick is meant to be called?** A doc example on the public item, so `cargo test` keeps it true.
- **Scoped build work the developer asked for, with rules to cite?** A step in `docs/queue.yaml`, placed by dependency.
- **Learned a durable gotcha or invariant that no assertion can capture**, something about intent or rationale rather than behavior? The crate docs in `src/lib.rs`.
- **Just describes what this diff does?** Commit or PR body.
- **Exploration that concluded nothing durable, in-flight notes, dead ends?** Discard. Do not write it anywhere.

When something could live as either a test or a line of prose, the test wins. It is enforced; the prose is not. Prose only catches what an assertion structurally cannot, which is why intent and rationale are the crate docs' job.

### Decisions
ADRs record architectural decisions that shape the direction of the project. Only the developer starts one. Do not draft one, and do not offer to, because a question looks like a decision, because a choice has alternatives, or because a change feels significant. Write the `OPEN:` line if a spec domain needs the answer, otherwise write nothing, and carry on. When the developer wants an ADR they will ask, and then you draft it from `docs/adr/0000-adr-template.md` and confirm it with them. This is deliberate: agents offer ADRs far more often than they are wanted, and an unwanted ADR is noise in an append-only log (ADR-0008).

## Working with the developer
This file is guidance you follow while working; it catches most process gaps in conversation. It cannot by itself force anyone to do anything. The hard guarantee comes from the automated gates under "What is enforced vs trusted" (CI and the pre-commit hook). Use both: you guide while the work happens, the gates block what slips through.

Assume the developer may be junior or moving fast and may not know Polylith vocabulary. The system works only if you carry the process, not them.

How to behave:
- **Do the bookkeeping yourself.** When a change needs a test, a crate-doc update, a spec `OPEN:` line, a queue update, or an ADR the developer has asked for, draft it and ask the developer to confirm. Do not tell them to go write it. Make the correct path the easy one.
- **Explain the term as you use it.** When you say base, brick interface, spec domain, or `interface-impact`, add a one-line plain explanation. Do not assume the vocabulary is known.
- **One question at a time.** Do not interrogate. Ask the single most important confirmation, act on the answer, move on.
- **Prefer the simplest action (BSSN).** If a brick is trivial, say a longer crate doc is not needed rather than writing one "to be safe." Ask before adding structure.

### Stop and confirm before
- **Changing a public interface** (a brick's `src/lib.rs` surface). Say plainly: "this changes what other code depends on; every project using `<brick>` is affected. Intended?" If yes, make the change, add or adjust the test, and say that the interface-change check will need either an ADR naming the brick or an `[interface-impact: none]` justification in a commit message. The developer chooses which; draft the ADR only if they ask.
- **Hitting a choice with real alternatives.** Say: "this is a choice with other options (A, B). It is an `OPEN:` line in `<domain>.md` until you decide." Write the `OPEN:` line. Do not offer an ADR.
- **Adding, rewording, or withdrawing a numbered spec rule, creating a domain, or editing `docs/spec/00-glossary.md`.** Show the sentence, then land it on confirmation. Adding an `OPEN:` line needs no confirmation.
- **Anything destructive or hard to reverse:** deleting or renaming a brick, a public name, or a spec domain, merging or splitting components, moving a dependency boundary. Confirm intent and name the blast radius (`cargo poly diff` shows the affected bricks and projects; `cargo poly deps` shows who depends on a brick; `grep -rn "<domain> rule" components/ bases/` shows the tests pinned to a domain).
- **Putting business logic in a base or a project.** Remind: logic lives in components, bases stay thin, a project is one `main.rs`.
- **Adding a dependency.** Ask whether it is needed now or whether a few lines do the job for now.

### Remind, but do not block, when
- A behavior changed and no test was added for it.
- A brick implements a spec rule and no test cites it as `<domain> rule N`.
- A public interface changed and no ADR is linked.
- A queue step is done (every rule cited) and still in the file.
- A brick's crate docs now contradict the code.
- Scratch is left in `development/` that should be reset.

### Pre-commit checklist
The automated checks enforce structure. This covers what they cannot:
1. **Behavior added or changed → a test asserts it and cites its rule.** If you changed what a brick does or fixed a bug, there should be a test in the brick's `tests/` that would fail without that change, citing the spec rule it pins. A test that only passes incidentally teaches nothing and breaks on refactor.

2. **A question or choice came up → it is an `OPEN:` line, or the developer settled it.** If it is still open, the `OPEN:` line naming the alternatives is in the spec domain. If the developer settled it, the rule is in the spec and the `OPEN:` line is gone. An ADR exists only if the developer asked for one.

3. **Crate docs still match the code.** If the public surface or the brick's intent changed, check that the `//!` prose still accurately describes it. Stale prose is worse than no prose: it actively misleads. Doc examples take care of themselves; `cargo test` fails if they lie.

4. **Scratch reset in `development/`.** Throwaway code in `development/src/main.rs` and dependencies added to `development/Cargo.toml` for an exploration that concluded nothing durable should be removed. If something durable was found, route it to the right layer instead of leaving it there.

5. **The queue matches the code.** A step whose rules all have a citing test is deleted in this commit. A step the developer asked for is added. Nothing in between is written down.

## Conventions
### ADRs
- Location: `docs/adr/`, a single global numbered sequence. Not per brick.
- Started by the developer. The agent drafts one only when asked and never offers one unprompted.
- Template: copy `docs/adr/0000-adr-template.md`. Do not edit it in place. It is MADR 4.0 (see References) extended with the `affects` and `interface-impact` fields.
- Filenames: `NNNN-short-kebab-title.md`, zero padded.
- Numbers are sequential and never reused.
- Body prose: single line per paragraph (no hard-wrapping). Markdown renders it the same; keeps grep -rl matches to a single line per hit.
- ADRs are immutable. To reverse a decision, write a new ADR and set the old one's status to `superseded by ADR-NNNN`.
- Status lifecycle: `proposed` -> `accepted` -> (`deprecated` | `superseded`).
- Front matter carries `affects` (the components, bases, and projects the decision touches, by short name) and `interface-impact` (`none` | `new` | `breaking`).
- A change that breaks a public `src/lib.rs` contract requires an ADR whose `affects` names the brick, or an explicit `[interface-impact: none]` justification; the interface-change check enforces that something was said.
- ADRs are for decisions. Do not write one to describe current state, and do not write one for a brick whose shape involved no contested choice.

### Spec
- Conventions, the rule lifecycle, the citation grammar, and the shared-term process live in `docs/spec/README.md`. Not repeated here.

### Queue
- Shape and the derived states are in the header of `docs/queue.yaml` and the module docs of `tools/poly/src/spec.rs`. Not repeated here.

### Tests
- Live per brick in `tests/`, as integration tests that can only see `src/lib.rs`. Run them with `cargo test --workspace`, or one brick's with `cargo test -p apperception-<name>`. `cargo poly diff` shows which bricks and projects a change affects; that signal also tells you the blast radius of what you touched.
- Assert the contract and documented behavior, not implementation internals. A test pinning an incidental detail breaks on every refactor and teaches the next reader nothing. Test at the interface; leave the implementation free to change. Unit tests inside `src/` are allowed for internals and cite no spec rule.
- Cite the spec rule a test pins as `<domain> rule N` in a comment or doc comment on the test. That exact form is what the lint and `cargo poly spec status` grep for.

### Crate docs
- Use the `//!` crate doc at the top of `src/lib.rs` for the short "what this exposes and how to call it" note (it shows up in `cargo doc` and IDE hovers), and `///` with a runnable example on a public item when calling it is not obvious. Promote to a `README.md` in the brick only when the description runs longer than a screen.
- Do not write both saying the same thing; they will drift.
- Not mandatory per brick. A trivial one-function component is self-explanatory; a long crate doc there is noise. Write one when the brick has enough surface or non-obvious behavior to warrant it.

### Bricks
- Create a component or base with `cargo new --lib components/<name> --name apperception-<name>`, then add `version.workspace = true`, `edition.workspace = true`, `license.workspace = true`, `publish.workspace = true` and `[lints] workspace = true` to its `Cargo.toml`, and one line `apperception-<name> = { path = "components/<name>" }` under `[workspace.dependencies]` in the root `Cargo.toml`. Copy an existing brick if that is quicker. `cargo poly check` tells you what is missing.
- Create a project with `cargo new --bin projects/<name>` and one base under its `[dependencies]`.

## Guardrails
- The interface contract lives only in `src/lib.rs`. Never copy it into prose; the copy drifts and the prose is not enforced.
- Behavioral guarantees live in tests, not in crate-doc prose. If it can be asserted, assert it. If it is how to call something, make it a doc example.
- Rules come from the developer. Never invent a spec rule to unblock yourself; write the `OPEN:` line and stop.
- Never draft or offer an ADR unasked.
- Raw session narrative does not go in the codebase. Only the distilled, routed artifacts above.
- Keep the prose layer to slow-changing things: purpose, invariants, non-obvious constraints. Volatile specifics belong in the contract and the tests, where they are enforced. The more you push into prose, the more drift you buy.

## What is enforced vs trusted
Two layers hold the process together. The agent behavior above is the soft layer: it catches gaps in conversation, while the work happens. The gates below are the hard layer: they block non-compliant changes regardless of who or what made them. For junior or fast-moving developers, the gates are what actually force the process; the agent guidance only makes following it the easy path. Wire the gates up early.

Hard gates (block the change):
- `cargo poly check`: brick naming and location, `lib.rs` as an explicit interface, the base-to-component dependency direction, projects as composition roots.
- The compiler and `cargo clippy --workspace --all-targets -- -D warnings`: types are valid and no warning stands, in tests and examples too.
- `cargo fmt --check`: one formatting.
- Tests (`cargo test --workspace`) in CI: behavioral guarantees in tests and doc examples.
- `cargo build --locked` and `cargo audit` in CI: the lockfile is current and no dependency has a known advisory.
- Decision log lint (`cargo poly adr lint`): front matter parses, statuses are valid, numbering is unique. A name in `affects` that no longer exists only warns.
- Spec and queue lint (`cargo poly spec lint`): spec headings in order, rule numbering contiguous, `<domain> rule N` citations resolve, links under `docs/` resolve, no glossary term redefined, `docs/queue.yaml` well-formed and naming real domains.
- Interface-change check (`cargo poly interface --mode ci`): a PR changing a brick's `src/lib.rs` must include an ADR naming that brick or justify `interface-impact` in a commit message. This keeps the why-layer honest.
- The pre-commit hook (`.githooks/pre-commit`): runs formatting, `cargo poly check`, the linters, the ADR index refresh and clippy locally before a commit lands, so feedback comes in seconds rather than from a failed CI run. Highest-leverage gate for juniors.
- CI (`.github/workflows/checks.yml`) plus branch protection on `main`: the unskippable server-side gate. See CONTRIBUTING.md for the one-time setup.

The per-brick ADR index is generated by `cargo poly adr index` into `docs/adr/index/`. Setup and the day-to-day loop for developers live in CONTRIBUTING.md; the decisions behind this enforcement are in `docs/adr/`.

Trusted (convention, made easy but not blocked):
- Good commit and PR messages, the discard discipline, crate-doc accuracy. Distillation quality cannot be fully enforced; this file and the agent behavior make the correct path the easy one.
- That a spec rule is true and worth having, and that a queue step was asked for. The lint checks that they are well-formed; the developer's confirmation is the backstop.
- The `[interface-impact: none]` escape in the interface-change check is self-reported. The check enforces that something was said; code review is the backstop against an agent or developer using it to avoid recording a real interface change.

## References
- Polylith (concepts): https://polylith.gitbook.io/polylith/
- MADR 4.0: https://adr.github.io/madr/
- Best Simple System for Now: https://dannorth.net/blog/best-simple-system-for-now/
- Cargo workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
