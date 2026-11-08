//! The workspace model: every crate Cargo knows about, classified by where it lives,
//! with its dependencies on other workspace crates. Read from `cargo metadata`, so it
//! is always what Cargo itself sees.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::util;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Component,
    Base,
    Project,
    Development,
    Tool,
    Other,
}

impl Kind {
    pub fn word(self) -> &'static str {
        match self {
            Kind::Component => "component",
            Kind::Base => "base",
            Kind::Project => "project",
            Kind::Development => "development crate",
            Kind::Tool => "tool",
            Kind::Other => "member",
        }
    }

    /// The `affects` key in an ADR's front matter for this kind, if it has one.
    pub fn affects_key(self) -> Option<&'static str> {
        match self {
            Kind::Component => Some("components"),
            Kind::Base => Some("bases"),
            Kind::Project => Some("projects"),
            _ => None,
        }
    }
}

pub struct Member {
    /// The crate name, e.g. `apperception-greeting`.
    pub name: String,
    pub kind: Kind,
    /// The short name: the directory under components/, bases/ or projects/, e.g. `greeting`.
    pub brick: String,
    /// Absolute directory of the crate.
    pub dir: PathBuf,
    /// Directory relative to the workspace root, e.g. `components/greeting`.
    pub rel: String,
    /// Crate names of the workspace members this one depends on (any dependency kind).
    pub deps: Vec<String>,
    pub has_lib: bool,
    pub has_bin: bool,
}

impl Member {
    pub fn is_brick(&self) -> bool {
        matches!(self.kind, Kind::Component | Kind::Base)
    }
}

pub struct Workspace {
    pub namespace: String,
    pub members: Vec<Member>,
}

impl Workspace {
    pub fn load() -> Result<Self, String> {
        let out = util::run(
            &util::cargo(),
            &["metadata", "--no-deps", "--format-version", "1"],
        )?;
        let meta: Value =
            serde_json::from_str(&out).map_err(|e| format!("cannot parse cargo metadata: {e}"))?;
        let root = PathBuf::from(
            meta["workspace_root"]
                .as_str()
                .ok_or("cargo metadata has no workspace_root")?,
        );
        let namespace = meta["metadata"]["polylith"]["namespace"]
            .as_str()
            .ok_or(
                "Cargo.toml needs `[workspace.metadata.polylith]` with `namespace = \"...\"`, \
                 the prefix of every brick's crate name",
            )?
            .to_string();
        let packages = meta["packages"]
            .as_array()
            .ok_or("cargo metadata has no packages")?;

        let dirs: Vec<(String, PathBuf)> = packages
            .iter()
            .filter_map(|p| Some((p["name"].as_str()?.to_string(), crate_dir(p)?)))
            .collect();

        let mut members = Vec::new();
        for p in packages {
            let name = p["name"]
                .as_str()
                .ok_or("a package has no name")?
                .to_string();
            let dir = crate_dir(p).ok_or_else(|| format!("{name}: no manifest_path"))?;
            let rel = dir
                .strip_prefix(&root)
                .map(util::display)
                .unwrap_or_else(|_| util::display(&dir));
            let segments: Vec<&str> = rel.split('/').collect();
            let kind = match segments.as_slice() {
                ["components", _] => Kind::Component,
                ["bases", _] => Kind::Base,
                ["projects", _] => Kind::Project,
                ["development"] => Kind::Development,
                ["tools", _] => Kind::Tool,
                _ => Kind::Other,
            };
            let brick = segments.last().unwrap_or(&"").to_string();

            let mut deps: BTreeSet<String> = BTreeSet::new();
            for d in p["dependencies"].as_array().into_iter().flatten() {
                let Some(path) = d["path"].as_str() else {
                    continue;
                };
                let path = PathBuf::from(path);
                if let Some((dep_name, _)) = dirs.iter().find(|(_, dir)| *dir == path) {
                    deps.insert(dep_name.clone());
                }
            }

            let kinds: Vec<&str> = p["targets"]
                .as_array()
                .into_iter()
                .flatten()
                .flat_map(|t| t["kind"].as_array().into_iter().flatten())
                .filter_map(Value::as_str)
                .collect();
            let has_lib = kinds.iter().any(|k| {
                matches!(
                    *k,
                    "lib" | "rlib" | "dylib" | "cdylib" | "staticlib" | "proc-macro"
                )
            });
            let has_bin = kinds.contains(&"bin");

            members.push(Member {
                name,
                kind,
                brick,
                dir,
                rel,
                deps: deps.into_iter().collect(),
                has_lib,
                has_bin,
            });
        }
        members.sort_by(|a, b| a.rel.cmp(&b.rel));
        Ok(Workspace { namespace, members })
    }

    pub fn find(&self, name: &str) -> Option<&Member> {
        self.members.iter().find(|m| m.name == name)
    }

    /// The member whose directory contains `file` (a path relative to the root).
    pub fn owner_of(&self, file: &str) -> Option<&Member> {
        self.members
            .iter()
            .filter(|m| file.starts_with(&format!("{}/", m.rel)))
            .max_by_key(|m| m.rel.len())
    }

    /// Every brick `member` depends on, directly or through other bricks, by crate name.
    pub fn transitive_bricks(&self, member: &Member) -> BTreeSet<String> {
        let mut seen = BTreeSet::new();
        let mut stack: Vec<&str> = member.deps.iter().map(String::as_str).collect();
        while let Some(name) = stack.pop() {
            let Some(dep) = self.find(name) else { continue };
            if dep.is_brick() && seen.insert(dep.name.clone()) {
                stack.extend(dep.deps.iter().map(String::as_str));
            }
        }
        seen
    }
}

fn crate_dir(package: &Value) -> Option<PathBuf> {
    Path::new(package["manifest_path"].as_str()?)
        .parent()
        .map(Path::to_path_buf)
}

/// Make the workspace root the working directory, so every command can use paths
/// like `docs/adr` no matter where it was invoked from.
pub fn chdir_to_root() -> Result<(), String> {
    let manifest = util::run(
        &util::cargo(),
        &["locate-project", "--workspace", "--message-format", "plain"],
    )
    .or_else(|_| {
        util::git(&["rev-parse", "--show-toplevel"]).map(|top| format!("{}/Cargo.toml", top.trim()))
    })?;
    let root = Path::new(manifest.trim())
        .parent()
        .ok_or("cannot find the workspace root")?
        .to_path_buf();
    std::env::set_current_dir(&root).map_err(|e| format!("cannot enter {}: {e}", root.display()))
}
