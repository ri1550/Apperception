//! The `hello` project: the composition root that turns the `api` base into a
//! binary.
//!
//! A project holds no logic. Its `src/` has only this file, and this file only
//! wires one base (plus any components it needs) together. See ADR-0004.
//!
//! EXAMPLE project. Delete it together with `bases/api` and `components/greeting`.

fn main() {
    println!("{}", apperception_api::run());
}
