//! `cargo poly`: the checks that keep this Polylith workspace honest.
//!
//! Run from anywhere inside the workspace (the alias lives in `.cargo/config.toml`):
//!
//!     cargo poly check                 brick boundaries: layout, crate names, dependency
//!                                      direction, projects as composition roots, lib.rs as
//!                                      an explicit interface
//!     cargo poly deps                  which bricks depend on which; what each project includes
//!     cargo poly diff [--since REF]    bricks and projects changed since the last stable-* tag
//!     cargo poly interface --mode pre-commit|ci
//!                                      a changed public lib.rs must come with a recorded decision
//!     cargo poly adr lint              the decision log: filenames and front matter
//!     cargo poly adr index             regenerate docs/adr/index/
//!     cargo poly spec lint             docs/spec/ structure, rule citations, links, docs/queue.yaml
//!     cargo poly spec status           what works (rules with a citing test) and what is queued
//!     cargo poly remind                the pointer to the pre-commit checklist in AGENTS.md
//!
//! Exit code 0 means all good. 1 means there is something to fix and the output says
//! what and where. 2 means the command line was not understood.
//!
//! The checks are deliberately small (Best Simple System for Now). If you change what
//! one enforces, update AGENTS.md and CONTRIBUTING.md to match.

mod adr;
mod check;
mod diff;
mod interface;
mod spec;
mod util;
mod workspace;

use std::process::ExitCode;

const USAGE: &str = "usage: cargo poly <command>

  check                          brick boundaries and interfaces
  deps                           brick dependencies and what each project includes
  diff [--since REF]             bricks and projects changed since the last stable-* tag
  interface --mode pre-commit|ci a changed public lib.rs needs a recorded decision
  adr lint | adr index           the decision log
  spec lint | spec status        the spec and the build queue
  remind                         the pre-commit checklist pointer";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    match dispatch(&args) {
        Ok(code) => ExitCode::from(code),
        Err(message) => {
            eprintln!("poly: {message}");
            ExitCode::from(1)
        }
    }
}

fn dispatch(args: &[&str]) -> Result<u8, String> {
    workspace::chdir_to_root()?;
    match args {
        ["check"] => check::check(),
        ["deps"] => check::deps(),
        ["diff", rest @ ..] => diff::run(rest),
        ["interface", rest @ ..] => interface::run(rest),
        ["adr", "lint"] => adr::lint(),
        ["adr", "index"] => adr::index(),
        ["spec", "lint"] => spec::lint(),
        ["spec", "status"] => Ok(spec::status()),
        ["remind"] => {
            println!("Before committing: walk the \"Pre-commit checklist\" section in AGENTS.md.");
            Ok(0)
        }
        _ => {
            eprintln!("{USAGE}");
            Ok(2)
        }
    }
}
