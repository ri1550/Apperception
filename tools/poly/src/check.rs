//! `cargo poly check`: the Polylith rules that Cargo alone does not enforce.
//! `cargo poly deps`: the brick dependency graph, for reading blast radius.
//!
//! What `check` enforces:
//!   - every workspace member lives under components/, bases/, projects/, development/
//!     or tools/
//!   - a component or base is a library crate named `<namespace>-<directory>`, with
//!     no binary target
//!   - a brick's `src/lib.rs` is an explicit interface: no `pub mod`, no `pub use ...::*`
//!   - components never depend on bases; bases never depend on bases
//!   - a project is a binary crate whose `src/` holds only `main.rs`, depending on
//!     exactly one base (and any components); it is the composition root (ADR-0004)
//!   - nothing depends on a project, on the development crate, or on a tool

use std::collections::BTreeMap;
use std::fs;

use crate::util;
use crate::workspace::{Kind, Member, Workspace};

pub fn check() -> Result<u8, String> {
    let ws = Workspace::load()?;
    let mut errors: Vec<String> = Vec::new();

    for m in &ws.members {
        match m.kind {
            Kind::Component | Kind::Base => check_brick(&ws, m, &mut errors),
            Kind::Project => check_project(&ws, m, &mut errors),
            Kind::Development | Kind::Tool => {}
            Kind::Other => errors.push(format!(
                "{}: workspace member outside components/, bases/, projects/, development/ and tools/; \
                 every crate is a brick, a project, scratch space or a tool",
                m.rel
            )),
        }
        for dep in &m.deps {
            let Some(d) = ws.find(dep) else { continue };
            let reason = match d.kind {
                Kind::Project => "projects are composition roots; nothing depends on them",
                Kind::Development => {
                    "the development crate is scratch space; nothing depends on it"
                }
                Kind::Tool => "tools are not part of the product; nothing depends on them",
                _ => continue,
            };
            errors.push(format!(
                "{}: depends on `{}` ({}); {}",
                m.rel, dep, d.rel, reason
            ));
        }
    }

    if errors.is_empty() {
        let bricks = ws.members.iter().filter(|m| m.is_brick()).count();
        let projects = ws
            .members
            .iter()
            .filter(|m| m.kind == Kind::Project)
            .count();
        println!(
            "Polylith check passed: {bricks} brick(s), {projects} project(s), boundaries hold."
        );
        return Ok(0);
    }
    println!("Polylith check found problems:\n");
    for e in &errors {
        println!("  - {e}");
    }
    println!("\nSee AGENTS.md, \"Polylith rules\", and ADR-0004 for the layout these enforce.");
    Ok(1)
}

fn check_brick(ws: &Workspace, m: &Member, errors: &mut Vec<String>) {
    let word = m.kind.word();
    let expected = format!("{}-{}", ws.namespace, m.brick);
    if m.name != expected {
        errors.push(format!(
            "{}: crate is named `{}` but a {word} is `<namespace>-<directory>`, here `{expected}` \
             (namespace from [workspace.metadata.polylith] in Cargo.toml)",
            m.rel, m.name
        ));
    }
    if !m.has_lib {
        errors.push(format!(
            "{}: a {word} is a library crate, but this one has no lib target (src/lib.rs)",
            m.rel
        ));
    }
    if m.has_bin {
        errors.push(format!(
            "{}: a {word} must not have a binary target; binaries are projects under projects/",
            m.rel
        ));
    }
    if let Ok(text) = fs::read_to_string(m.dir.join("src/lib.rs")) {
        for problem in interface_problems(&text) {
            errors.push(format!("{}/src/lib.rs: {problem}", m.rel));
        }
    }
    for dep in &m.deps {
        let Some(d) = ws.find(dep) else { continue };
        match (m.kind, d.kind) {
            (Kind::Component, Kind::Base) => errors.push(format!(
                "{}: component depends on base `{}` ({}); dependencies flow base -> component, never the reverse, \
                 so components stay reusable",
                m.rel, dep, d.rel
            )),
            (Kind::Base, Kind::Base) => errors.push(format!(
                "{}: base depends on base `{}` ({}); bases are entry points and share code through components",
                m.rel, dep, d.rel
            )),
            _ => {}
        }
    }
}

fn check_project(ws: &Workspace, m: &Member, errors: &mut Vec<String>) {
    if !m.has_bin {
        errors.push(format!(
            "{}: a project is a binary crate, but this one has no bin target (src/main.rs)",
            m.rel
        ));
    }
    if m.has_lib {
        errors.push(format!(
            "{}: a project must not have a lib target; shared code is a component, an entry point is a base",
            m.rel
        ));
    }
    let src = m.dir.join("src");
    for file in util::walk(&src) {
        if file != src.join("main.rs") {
            let shown = file.strip_prefix(&m.dir).unwrap_or(&file);
            errors.push(format!(
                "{}: `{}` is not allowed; a project's src/ holds only main.rs, the composition root \
                 (logic lives in components, entry points in bases)",
                m.rel,
                util::display(shown)
            ));
        }
    }
    let bases: Vec<&str> = m
        .deps
        .iter()
        .filter(|d| ws.find(d).is_some_and(|dep| dep.kind == Kind::Base))
        .map(String::as_str)
        .collect();
    match bases.len() {
        1 => {}
        0 => errors.push(format!(
            "{}: project depends on no base; a project wires exactly one base into a binary",
            m.rel
        )),
        n => errors.push(format!(
            "{}: project depends on {n} bases ({}); a project wires exactly one base into a binary",
            m.rel,
            bases.join(", ")
        )),
    }
}

/// Ways a brick's `lib.rs` can stop being an explicit interface.
pub fn interface_problems(text: &str) -> Vec<String> {
    let code = strip_comments(text);
    let mut problems = Vec::new();
    for line in code.lines() {
        let line = line.trim_start();
        if let Some(rest) = line.strip_prefix("pub mod ") {
            let name = rest
                .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                .next()
                .unwrap_or("");
            problems.push(format!(
                "`pub mod {name}` exposes a whole module; the interface is the explicit `pub use` list \
                 (keep the module private and re-export what other bricks may call)"
            ));
        }
    }
    for statement in code.split(';') {
        let statement = statement.trim_start();
        if statement.starts_with("pub use ") && statement.contains('*') {
            problems.push(
                "`pub use ...::*` re-exports everything in a module; name each item so the interface stays explicit"
                    .to_string(),
            );
        }
    }
    problems
}

fn strip_comments(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("/*") {
        out.push_str(&rest[..start]);
        match rest[start..].find("*/") {
            Some(end) => rest = &rest[start + end + 2..],
            None => rest = "",
        }
    }
    out.push_str(rest);
    out.lines()
        .map(|line| line.find("//").map_or(line, |i| &line[..i]))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn deps() -> Result<u8, String> {
    let ws = Workspace::load()?;
    let width = ws.members.iter().map(|m| m.rel.len()).max().unwrap_or(0);
    let mut used_by: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for m in &ws.members {
        for d in &m.deps {
            used_by.entry(d.as_str()).or_default().push(&m.rel);
        }
    }

    println!("Bricks (direct dependencies on other bricks):");
    for m in ws.members.iter().filter(|m| m.is_brick()) {
        let on: Vec<&str> = m.deps.iter().map(String::as_str).collect();
        let on = if on.is_empty() {
            "-".to_string()
        } else {
            on.join(", ")
        };
        println!("  {:<width$}  {:<10}  -> {}", m.rel, m.kind.word(), on);
    }

    println!("\nProjects (what each one includes, transitively):");
    let projects: Vec<&Member> = ws
        .members
        .iter()
        .filter(|m| m.kind == Kind::Project)
        .collect();
    if projects.is_empty() {
        println!("  none yet");
    }
    for p in projects {
        let includes: Vec<String> = ws
            .transitive_bricks(p)
            .iter()
            .filter_map(|name| ws.find(name))
            .map(|m| m.rel.clone())
            .collect();
        println!(
            "  {:<width$}  -> {}",
            p.rel,
            if includes.is_empty() {
                "-".to_string()
            } else {
                includes.join(", ")
            }
        );
    }

    println!("\nUsed by (who depends on each brick):");
    for m in ws.members.iter().filter(|m| m.is_brick()) {
        let users = used_by
            .get(m.name.as_str())
            .map_or("-".to_string(), |u| u.join(", "));
        println!("  {:<width$}  <- {users}", m.rel);
    }
    Ok(0)
}
