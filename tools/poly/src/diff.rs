//! `cargo poly diff [--since REF]`: which bricks and projects have changed since the
//! last `stable-*` tag (or since REF), including uncommitted and untracked files, and
//! which projects that reaches. This is the blast radius of what you touched.

use std::collections::{BTreeMap, BTreeSet};

use crate::util;
use crate::workspace::{Kind, Workspace};

pub fn run(args: &[&str]) -> Result<u8, String> {
    let since = match args {
        [] => None,
        ["--since", reference] => Some((*reference).to_string()),
        _ => {
            eprintln!("usage: cargo poly diff [--since REF]");
            return Ok(2);
        }
    };
    let ws = Workspace::load()?;

    let reference = match since {
        Some(r) => r,
        None => match util::git(&["describe", "--tags", "--abbrev=0", "--match", "stable-*"]) {
            Ok(tag) => tag.trim().to_string(),
            Err(_) => {
                println!(
                    "No stable-* tag yet. Tag a known-good commit (`git tag stable-YYYY.MM.DD`) \
                     or run `cargo poly diff --since <ref>`."
                );
                return Ok(0);
            }
        },
    };

    let changed = util::git(&["diff", "--name-only", &reference])?;
    let untracked = util::git(&["ls-files", "--others", "--exclude-standard"])?;
    let files: BTreeSet<&str> = changed
        .lines()
        .chain(untracked.lines())
        .filter(|l| !l.is_empty())
        .collect();

    let mut per_member: BTreeMap<&str, usize> = BTreeMap::new();
    let mut outside = 0;
    for file in &files {
        match ws.owner_of(file) {
            Some(m) => *per_member.entry(&m.rel).or_default() += 1,
            None => outside += 1,
        }
    }

    println!("Changed since {reference}:");
    if per_member.is_empty() {
        println!("  no bricks or projects");
    }
    for (rel, n) in &per_member {
        println!("  {rel} ({n} file(s))");
    }
    if outside > 0 {
        println!("  and {outside} file(s) outside bricks and projects");
    }

    let changed_bricks: BTreeSet<&str> = ws
        .members
        .iter()
        .filter(|m| m.is_brick() && per_member.contains_key(m.rel.as_str()))
        .map(|m| m.name.as_str())
        .collect();
    let affected: Vec<&str> = ws
        .members
        .iter()
        .filter(|m| m.kind == Kind::Project)
        .filter(|p| {
            per_member.contains_key(p.rel.as_str())
                || ws
                    .transitive_bricks(p)
                    .iter()
                    .any(|b| changed_bricks.contains(b.as_str()))
        })
        .map(|p| p.rel.as_str())
        .collect();
    if !affected.is_empty() {
        println!("Projects affected: {}", affected.join(", "));
    }
    Ok(0)
}
