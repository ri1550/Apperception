//! Implementation of the api base (private).

use apperception_greeting::greet;

/// Run the app: produce the line the binary prints.
pub fn run() -> String {
    greet("world")
}
