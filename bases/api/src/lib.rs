//! API base: a thin entry point. Delegates to components and holds no business
//! logic.
//!
//! Public interface: [`run`]. A project (`projects/hello`) wires it into a binary.
//!
//! EXAMPLE brick. Delete it together with `components/greeting` and
//! `projects/hello`.

mod api;

pub use api::run;
