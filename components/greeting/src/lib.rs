//! Greeting component.
//!
//! Public interface (the contract other bricks depend on): [`greet`].
//!
//! Why it is shaped this way: blank input is normalised here so every caller gets
//! the same behavior rather than each re-implementing it. The tests in `tests/` pin
//! the exact guarantees and cite the spec rules they hold (`docs/spec/greeting.md`).
//! See ADR-0005 for how these docs fit the context system.
//!
//! EXAMPLE brick demonstrating the interface/implementation split. Delete it once
//! you have your own components.
//!
//! ```
//! assert_eq!(apperception_greeting::greet("Ada"), "Hello, Ada!");
//! ```

mod greeting;

pub use greeting::greet;
