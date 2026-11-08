//! Implementation of the greeting component (private).
//!
//! Only the names re-exported from `lib.rs` are the public interface. Everything
//! here is free to change as long as that interface keeps behaving as the tests
//! say. See ADR-0002 (Polylith) and ADR-0005 (context system).

/// Return a friendly greeting for `name`.
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", clean(name))
}

// Not re-exported from lib.rs: private, not part of the public interface.
fn clean(name: &str) -> &str {
    let trimmed = name.trim();
    if trimmed.is_empty() { "world" } else { trimmed }
}
