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

# ADR-0002: Adopt the Polylith architecture
## Context and problem statement
We are starting a new Rust codebase that will likely grow to several deployable binaries or services sharing a lot of code. We need a structure that makes sharing code easy, keeps clear boundaries between features, and lets us defer the choice of deployment shape (one binary, several services, a library) until we need it.

## Decision drivers
- Share code across multiple binaries or services without copy-paste or a sprawl of published crates.
- Clear separation between a feature's public interface and its implementation.
- Defer deployment decisions; focus on code and features first.
- Best Simple System for Now: avoid speculative infrastructure; one workspace, one build, grow structure only as needed.

## Considered options
- Polylith (components, bases, projects) with each brick a crate in one Cargo workspace
- A single crate with modules
- A Cargo workspace of crates without the brick model
- Multiple repositories, one per service, with shared code published as crates

## Decision outcome
Chosen option: Polylith, with bricks as crates in one Cargo workspace. Code lives as bricks (components for business logic, bases as thin entry points) under `components/` and `bases/`, combined into deployable binaries under `projects/`. A brick's public interface is its `src/lib.rs`; its implementation is private behind that. How the model maps onto Cargo in detail is ADR-0004.

There is no official Polylith tooling for Rust. The checks the Clojure and Python tools provide (`poly check`, `poly deps`, `poly diff`) are a small in-repo tool, `cargo poly` in `tools/poly/`, scoped to checking and never to scaffolding. A brick is created with `cargo new` and two edits (README.md); the tool tells you when the result breaks a rule.

Polylith over the alternatives because it gives code sharing and clear boundaries in one workspace and one build, separates interface from implementation by construction, and lets us defer deployment shape. A single crate with modules has no compiler-enforced boundary between features and is one compilation unit. A workspace without the brick model would have us reinvent the component/base/project distinction and its rules case by case. Multiple repositories reintroduce the duplication and versioning problems we want to avoid.

### Consequences
- Good: shared code with boundaries the compiler enforces (a crate sees only what another crate makes `pub`); interface/implementation separation is structural; deployment shape stays deferrable.
- Good: one workspace, one lockfile, one `target/`, so the whole codebase builds, tests and lints as one unit and an agent has full context.
- Bad: contributors must learn Polylith's model, and there is no `poly create`; bricks are made by hand.
- Neutral: business logic must not live in `projects/`; a project is a composition root only (ADR-0004).

### Confirmation
`cargo poly check` and `cargo poly deps` run in the pre-commit hook and CI and enforce the brick boundaries: in particular, components must not depend on bases, so the dependency direction stays base to components and components remain reusable. A violation fails the build.

## More information
- ADR-0004 (how bricks, projects and the development crate map onto Cargo)
- ADR-0007 (the decision to enforce these boundaries with automated checks)
- Original Polylith documentation (Clojure; the concepts apply, the tooling does not): https://polylith.gitbook.io/polylith/
- Python tools for Polylith (the sibling template's implementation): https://davidvujic.github.io/python-polylith-docs/
- Best Simple System for Now (BSSN): https://dannorth.net/blog/best-simple-system-for-now/
