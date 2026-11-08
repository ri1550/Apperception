//! Interface tests for the greeting component. They see only what `lib.rs`
//! exports, so they cannot pin implementation details. Each test cites the spec
//! rule it holds as `<domain> rule N`.

use apperception_greeting::greet;

/// Asserts greeting rule 1.
#[test]
fn greet_uses_the_name() {
    assert_eq!(greet("Ada"), "Hello, Ada!");
}

/// Asserts greeting rule 2.
#[test]
fn greet_defaults_to_world_when_blank() {
    assert_eq!(greet("   "), "Hello, world!");
}
