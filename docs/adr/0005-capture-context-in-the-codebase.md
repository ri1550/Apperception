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

# ADR-0005: Capture context in the codebase
## Context and problem statement
We want the why, what, and how of the codebase to live in the codebase itself, so that a developer or an AI agent can reconstruct any part's context without relying on memory, chat logs, or tribal knowledge. We also want useful context from each development session, whether by a human or an agent, to end up in the right place. The risk is the opposite failure: dumping everything and drowning the signal.

## Decision drivers
- Any brick's full context should be a few cheap reads at any codebase size.
- Each kind of context needs exactly one home, reached by query or by sitting next to the code, never by reading piles.
- Prose that describes how to call code drifts; an example that is compiled and run does not.
- BSSN: capture only what is durable, discard the rest. The value is in the distillation, not the capture.

## Considered options
- One central log plus colocated, query-tagged layers (the system below)
- Per-brick decision records and notes only
- An external wiki or doc tool
- Dumping raw session transcripts into the repo
- No formal capture; rely on commit history and memory

## Decision outcome
Chosen option: a small set of layers, each owning one kind of context, plus a routing rule applied at the end of a session and a reading protocol applied before editing. The full, living specification is `AGENTS.md` at the workspace root; this ADR records the decision and its rationale, `AGENTS.md` records the mechanics.

The layers: `src/lib.rs` for the public interface, the interface tests in `tests/` for behavior and how to call a brick, the crate docs (`//!` in `lib.rs` and `///` on public items, with runnable examples) for intent, invariants and usage, `docs/adr/` for decisions with alternatives, and commit or PR messages for what a change did. Decisions are found per brick by querying the `affects` field (`grep -rl <brick> docs/adr/`), not by reading the whole log.

Doc examples are the usage layer. `cargo test` compiles and runs every example in a doc comment, so a usage note that stops matching the code fails the build instead of misleading the next reader. Prose in the crate docs is reserved for what neither a test nor an example can express: why the brick is shaped this way.

Central-plus-tagged over the alternatives because per-brick-only records fragment the log and have no home for cross-cutting decisions; an external wiki breaks the "context lives with the code" goal; and dumping raw transcripts recreates the signal-to-noise problem we are trying to avoid. The discipline that makes this work is routing each thing learned to exactly one layer and discarding what has no durable home.

### Consequences
- Good: context is co-located with code, queryable, and stays cheap to load as the codebase grows.
- Good: tests and doc examples carry behavioral and usage context as enforced layers; prose is reserved for what an assertion cannot express.
- Bad: depends on discipline (good commit messages, the discard default, crate-doc accuracy) that cannot be fully enforced by tooling.
- Neutral: tests are treated as a context layer and are written at the interface, which the `tests/` directory guarantees by construction (ADR-0004). This is recorded here rather than in a separate ADR for now; it will be split out only if a contested testing decision arises.

### Confirmation
The context layers are kept accurate by the automated checks described in ADR-0006 and ADR-0007; in particular `cargo test` runs the doc examples. Everything else is convention that `AGENTS.md` makes the path of least resistance.

## More information
- `AGENTS.md` (the living specification this ADR points at)
- ADR-0006 (the decision to enforce this process with automated checks)
- Documentation tests: https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html
- Best Simple System for Now (BSSN): https://dannorth.net/blog/best-simple-system-for-now/
