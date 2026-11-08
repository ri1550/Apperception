//! Small helpers shared by the commands: running programs, reading files, walking
//! directories.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Run a program and return its stdout. A non-zero exit is an error carrying stderr.
pub fn run(program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("cannot run `{program}`: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "`{program} {}` failed: {}",
            args.join(" "),
            stderr.trim()
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn git(args: &[&str]) -> Result<String, String> {
    run("git", args)
}

/// The cargo binary that invoked us, or plain `cargo`.
pub fn cargo() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

pub fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))
}

/// Every file under `dir`, recursively, in sorted order. A missing directory is empty.
pub fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    collect(dir, &mut out);
    out.sort();
    out
}

fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else {
            out.push(path);
        }
    }
}

/// Immediate subdirectories of `dir`, by name, sorted. A missing directory is `None`.
pub fn subdirs(dir: &Path) -> Option<Vec<String>> {
    let entries = fs::read_dir(dir).ok()?;
    let mut names: Vec<String> = entries
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    Some(names)
}

/// 1-based line number of a byte offset in `text`.
pub fn line_of(text: &str, offset: usize) -> usize {
    text[..offset].matches('\n').count() + 1
}

/// A path relative to the workspace root, with forward slashes.
pub fn display(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
