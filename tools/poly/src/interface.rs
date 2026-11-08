//! `cargo poly interface --mode pre-commit|ci`: a public interface change must be
//! recorded.
//!
//! In this workspace a brick's public interface is its `src/lib.rs`. Changing it can
//! affect every project that uses the brick, so the change should come with an ADR
//! (or a clear note that it is not really a contract change).
//!
//!   --mode pre-commit   Runs on your machine before a commit. It only WARNS, so it
//!                       never blocks you. Think of it as a friendly heads-up.
//!
//!   --mode ci           Runs on GitHub for a pull request. It BLOCKS the merge if a
//!                       brick's lib.rs changed and nothing records the decision.
//!
//! What records a decision, per changed brick:
//!   1. Best: an ADR under docs/adr/ in the same pull request whose `affects` names
//!      the brick (or `"*"`). The agent can draft it.
//!   2. A commit message that mentions an ADR, e.g. "ADR-0007: rename charge()".
//!   3. If the edit does NOT change the public surface (a comment, formatting), a
//!      commit message containing "[interface-impact: none]".
//!
//! Options 2 and 3 are self-reported; code review is the backstop.

use crate::adr;
use crate::workspace::{Member, Workspace};

pub fn run(args: &[&str]) -> Result<u8, String> {
    let ws = Workspace::load()?;
    match args {
        ["--mode", "pre-commit"] => pre_commit(&ws),
        ["--mode", "ci"] => ci(&ws),
        _ => {
            eprintln!("usage: cargo poly interface --mode <pre-commit|ci>");
            Ok(2)
        }
    }
}

/// The bricks whose `src/lib.rs` is among `files`.
fn changed_interfaces<'a>(ws: &'a Workspace, files: &[&str]) -> Vec<&'a Member> {
    ws.members
        .iter()
        .filter(|m| m.is_brick())
        .filter(|m| files.contains(&format!("{}/src/lib.rs", m.rel).as_str()))
        .collect()
}

/// Does one of the ADR files name this brick in `affects` (or every brick, `"*"`)?
fn recorded_in(adr_files: &[&str], brick: &Member) -> bool {
    let Some(key) = brick.kind.affects_key() else {
        return false;
    };
    adr_files.iter().any(|path| {
        let Ok(text) = std::fs::read_to_string(path) else {
            return false;
        };
        let Ok(front) = adr::front_matter(&text) else {
            return false;
        };
        adr::names(&front, key)
            .iter()
            .any(|n| n == "*" || *n == brick.brick)
    })
}

fn pre_commit(ws: &Workspace) -> Result<u8, String> {
    let staged = crate::util::git(&["diff", "--cached", "--name-only"])?;
    let files: Vec<&str> = staged.lines().collect();
    let adrs: Vec<&str> = files
        .iter()
        .copied()
        .filter(|f| adr::is_record_path(f))
        .collect();
    let unrecorded: Vec<&Member> = changed_interfaces(ws, &files)
        .into_iter()
        .filter(|b| !recorded_in(&adrs, b))
        .collect();
    if unrecorded.is_empty() {
        return Ok(0);
    }
    println!("Heads-up: you changed a public interface (src/lib.rs):");
    for b in &unrecorded {
        println!("  - {}/src/lib.rs", b.rel);
    }
    println!(
        "This changes what other code depends on. If it is a real interface change, add an ADR \
         under docs/adr/ whose `affects` names the brick, in this commit (the agent can draft it). \
         CI will ask for this later. Not blocking you now."
    );
    Ok(0) // never block locally
}

fn ci(ws: &Workspace) -> Result<u8, String> {
    let base = std::env::var("BASE_REF")
        .or_else(|_| std::env::var("GITHUB_BASE_REF"))
        .ok()
        .filter(|b| !b.is_empty())
        .unwrap_or_else(|| "main".to_string());
    let range = format!("origin/{base}...HEAD");
    let diff = crate::util::git(&["diff", "--name-only", &range])
        .map_err(|e| format!("{e}\n(fetch the base branch first: git fetch origin {base})"))?;
    let files: Vec<&str> = diff.lines().collect();

    let changed = changed_interfaces(ws, &files);
    if changed.is_empty() {
        println!("No public interface changes. OK.");
        return Ok(0);
    }

    let messages = crate::util::git(&["log", "--format=%B", &range])?.to_lowercase();
    if messages.contains("adr-") || messages.contains("[interface-impact:") {
        println!("Public interface changed and a commit message records it. OK.");
        return Ok(0);
    }

    let adrs: Vec<&str> = files
        .iter()
        .copied()
        .filter(|f| adr::is_record_path(f))
        .collect();
    let unrecorded: Vec<&Member> = changed
        .into_iter()
        .filter(|b| !recorded_in(&adrs, b))
        .collect();
    if unrecorded.is_empty() {
        println!(
            "Public interface changed and an ADR in this pull request names each changed brick. OK."
        );
        return Ok(0);
    }

    println!("This pull request changes a public interface but records no decision for:\n");
    for b in &unrecorded {
        println!(
            "  - {}/src/lib.rs  ({} `{}`)",
            b.rel,
            b.kind.word(),
            b.brick
        );
    }
    println!(
        "\nDo one of these, then push again:\n\
         \x20 1. Add or update an ADR under docs/adr/ whose `affects` names the brick (best option; \
         the agent can draft it).\n\
         \x20 2. Reference an ADR in a commit message, e.g. 'ADR-NNNN: ...'.\n\
         \x20 3. If this is NOT a contract change (a comment or formatting), add \
         '[interface-impact: none]' to a commit message.\n\
         See CONTRIBUTING.md for details."
    );
    Ok(1)
}
