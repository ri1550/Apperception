//! The spec (docs/spec/) and the build queue (docs/queue.yaml): one lint, one status.
//!
//!     cargo poly spec lint     Enforce the structure that docs/spec/README.md describes
//!                              and the shape of docs/queue.yaml. Exit 1 on any problem.
//!                              Runs in the pre-commit hook and CI.
//!     cargo poly spec status   Print what works and what is next, all derived, nothing
//!                              written by hand: per domain, how many rules have a citing
//!                              test and how many questions are open; then the queue in
//!                              order with each step's state. Never fails. A SessionStart
//!                              hook prints it into the agent's context.
//!
//! What `lint` enforces:
//!   - every domain file in docs/spec/ has exactly the five headings, in order:
//!     Purpose, Vocabulary, Data, Behavior rules, Open questions
//!   - behavior rules are numbered from 1 with no gaps and no repeats
//!   - every `<domain> rule N` citation in a brick's tests/ directory and in docs/spec/
//!     names a rule that exists in that domain file
//!   - no term defined in docs/spec/00-glossary.md is defined again in a domain's
//!     Vocabulary section
//!   - every relative markdown link under docs/ resolves to a file
//!   - docs/queue.yaml, if present, is `steps:` holding a list of mappings with the keys
//!     id, rules, and optionally brick and after; ids are unique kebab-case; rules is
//!     `<domain>: "n"` or `"lo-hi"` naming a domain file that exists; `after` names an
//!     earlier step
//!
//! What `status` derives for each queue step:
//!   - waiting on spec: the step cites a rule number the domain does not have yet. The
//!     developer-facing question is that domain's OPEN: lines.
//!   - ready: every cited rule exists and none has a citing test
//!   - in progress: some cited rules have a citing test
//!   - done: every cited rule has a citing test. Delete the step; git is the record.
//!
//! Tests that pin rules are the interface tests under `components/<brick>/tests/` and
//! `bases/<brick>/tests/`; only citations there count as pinning. README.md and
//! TEMPLATE.md in docs/spec/ are conventions, not domains, so their headings and example
//! citations are not checked. Generated index directories under docs/ are skipped for
//! link checking.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use regex_lite::Regex;
use serde_yaml_ng::{Mapping, Value};

use crate::adr::scalar;
use crate::util;

const DOCS: &str = "docs";
const SPEC: &str = "docs/spec";
const QUEUE: &str = "docs/queue.yaml";
const GLOSSARY: &str = "docs/spec/00-glossary.md";
const NOT_DOMAINS: [&str; 3] = ["readme.md", "template.md", "00-glossary.md"];
const GENERATED_DIRS: [&str; 1] = ["docs/adr/index"];
const HEADINGS: [&str; 5] = [
    "Purpose",
    "Vocabulary",
    "Data",
    "Behavior rules",
    "Open questions",
];
const EXTERNAL_PREFIXES: [&str; 4] = ["http://", "https://", "mailto:", "#"];
const STEP_KEYS: [&str; 4] = ["id", "rules", "brick", "after"];

type Rules = BTreeMap<String, BTreeSet<u32>>;

struct Patterns {
    rule: Regex,
    open: Regex,
    citation: Regex,
    term: Regex,
    link: Regex,
    id: Regex,
    range: Regex,
}

fn patterns() -> Patterns {
    let re = |s: &str| Regex::new(s).expect("valid regex");
    Patterns {
        rule: re(r"(?m)^(\d+)\. "),
        open: re(r"(?m)^- OPEN:"),
        citation: re(r"\b([a-z0-9][a-z0-9-]*) rule (\d+)\b"),
        term: re(r"(?m)^\*\*([^*]+)\*\*\s*—"),
        link: re(r"(?m)(^|[^!])\[[^\]]*\]\(([^)\s]+)\)"),
        id: re(r"^[a-z0-9][a-z0-9-]*$"),
        range: re(r"^(\d+)(?:-(\d+))?$"),
    }
}

// --- spec -----------------------------------------------------------------------

fn domain_files() -> Vec<PathBuf> {
    let mut out: Vec<PathBuf> = util::walk(Path::new(SPEC))
        .into_iter()
        .filter(|p| p.parent() == Some(Path::new(SPEC)))
        .filter(|p| p.extension().is_some_and(|e| e == "md"))
        .filter(|p| {
            let name = p
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();
            !NOT_DOMAINS.contains(&name.as_str())
        })
        .collect();
    out.sort();
    out
}

fn stem(path: &Path) -> String {
    path.file_stem()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned()
}

/// Split a domain file into its `## ` sections, in file order.
fn sections(text: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = Vec::new();
    for line in text.lines() {
        if let Some(heading) = line.strip_prefix("## ") {
            out.push((heading.trim().to_string(), String::new()));
        } else if let Some((_, body)) = out.last_mut() {
            body.push_str(line);
            body.push('\n');
        }
    }
    out
}

fn section<'a>(secs: &'a [(String, String)], name: &str) -> &'a str {
    secs.iter()
        .find(|(h, _)| h == name)
        .map_or("", |(_, b)| b.as_str())
}

fn check_headings(path: &Path, text: &str, errors: &mut Vec<String>) -> Vec<(String, String)> {
    let secs = sections(text);
    let names: Vec<&str> = secs.iter().map(|(h, _)| h.as_str()).collect();
    if names != HEADINGS {
        errors.push(format!(
            "{}: headings must be exactly {HEADINGS:?}, in that order; found {names:?}",
            util::display(path)
        ));
    }
    secs
}

fn rule_numbers(p: &Patterns, path: &Path, body: &str, errors: &mut Vec<String>) -> BTreeSet<u32> {
    let numbers: Vec<u32> = p
        .rule
        .captures_iter(body)
        .filter_map(|c| c[1].parse().ok())
        .collect();
    let expected: Vec<u32> = (1..=numbers.len() as u32).collect();
    if numbers != expected {
        errors.push(format!(
            "{}: behavior rules must be numbered 1..{} in order with no gaps or repeats; found {numbers:?}",
            util::display(path),
            numbers.len()
        ));
    }
    numbers.into_iter().collect()
}

/// Rule numbers and open-question counts per domain, recording structural errors.
fn read_spec(
    p: &Patterns,
    errors: &mut Vec<String>,
) -> Result<(Rules, BTreeMap<String, usize>), String> {
    let mut rules: Rules = BTreeMap::new();
    let mut open: BTreeMap<String, usize> = BTreeMap::new();
    let shared = glossary_terms(p)?;
    for path in domain_files() {
        let text = util::read(&path)?;
        let secs = check_headings(&path, &text, errors);
        rules.insert(
            stem(&path),
            rule_numbers(p, &path, section(&secs, "Behavior rules"), errors),
        );
        open.insert(
            stem(&path),
            p.open.find_iter(section(&secs, "Open questions")).count(),
        );
        check_vocabulary(p, &path, section(&secs, "Vocabulary"), &shared, errors);
    }
    Ok((rules, open))
}

/// The interface test files of every brick: `components/*/tests/**/*.rs` and `bases/*/tests/**/*.rs`.
fn test_files() -> Vec<PathBuf> {
    let mut out = Vec::new();
    for kind in ["components", "bases"] {
        for brick in util::subdirs(Path::new(kind)).unwrap_or_default() {
            let tests = Path::new(kind).join(&brick).join("tests");
            out.extend(
                util::walk(&tests)
                    .into_iter()
                    .filter(|f| f.extension().is_some_and(|e| e == "rs")),
            );
        }
    }
    out
}

/// `<domain> rule N` citations found in brick tests, grouped by domain.
fn cited_rules(p: &Patterns) -> Result<Rules, String> {
    let mut found: Rules = BTreeMap::new();
    for path in test_files() {
        let text = util::read(&path)?;
        for c in p.citation.captures_iter(&text) {
            if let Ok(n) = c[2].parse() {
                found.entry(c[1].to_string()).or_default().insert(n);
            }
        }
    }
    Ok(found)
}

fn check_citations(p: &Patterns, rules: &Rules, errors: &mut Vec<String>) -> Result<(), String> {
    let mut files = test_files();
    files.extend(domain_files());
    if Path::new(GLOSSARY).exists() {
        files.push(PathBuf::from(GLOSSARY));
    }
    for path in files {
        let text = util::read(&path)?;
        for c in p.citation.captures_iter(&text) {
            let (domain, number) = (&c[1], &c[2]);
            let line = util::line_of(&text, c.get(0).map_or(0, |m| m.start()));
            let where_ = format!("{}:{line}", util::display(&path));
            match rules.get(domain) {
                None => errors.push(format!(
                    "{where_}: cites '{domain} rule {number}' but {SPEC}/{domain}.md does not exist"
                )),
                Some(set) if !number.parse::<u32>().is_ok_and(|n| set.contains(&n)) => {
                    errors.push(format!(
                        "{where_}: cites '{domain} rule {number}' but that rule does not exist"
                    ));
                }
                Some(_) => {}
            }
        }
    }
    Ok(())
}

fn glossary_terms(p: &Patterns) -> Result<BTreeSet<String>, String> {
    if !Path::new(GLOSSARY).exists() {
        return Ok(BTreeSet::new());
    }
    let text = util::read(Path::new(GLOSSARY))?;
    Ok(p.term
        .captures_iter(&text)
        .map(|c| c[1].trim().to_lowercase())
        .collect())
}

fn check_vocabulary(
    p: &Patterns,
    path: &Path,
    body: &str,
    shared: &BTreeSet<String>,
    errors: &mut Vec<String>,
) {
    for c in p.term.captures_iter(body) {
        let term = c[1].trim();
        if shared.contains(&term.to_lowercase()) {
            errors.push(format!(
                "{}: Vocabulary redefines '{term}', which {GLOSSARY} already defines; link to it instead",
                util::display(path)
            ));
        }
    }
}

fn check_links(p: &Patterns, errors: &mut Vec<String>) -> Result<(), String> {
    for path in util::walk(Path::new(DOCS)) {
        if !path.extension().is_some_and(|e| e == "md") {
            continue;
        }
        if GENERATED_DIRS.iter().any(|g| path.starts_with(g)) {
            continue;
        }
        let text = util::read(&path)?;
        for c in p.link.captures_iter(&text) {
            let target = &c[2];
            if EXTERNAL_PREFIXES.iter().any(|pre| target.starts_with(pre)) || target.contains('<') {
                continue; // external, an anchor, or a <placeholder> in convention text
            }
            let target = target.split('#').next().unwrap_or("");
            if !target.is_empty() && !path.parent().unwrap_or(Path::new("")).join(target).exists() {
                let line = util::line_of(&text, c.get(2).map_or(0, |m| m.start()));
                errors.push(format!(
                    "{}:{line}: link to '{target}' does not resolve",
                    util::display(&path)
                ));
            }
        }
    }
    Ok(())
}

// --- queue ----------------------------------------------------------------------

/// The steps in docs/queue.yaml, in order. A missing file is an empty queue.
fn load_queue(errors: &mut Vec<String>) -> Result<Vec<Mapping>, String> {
    if !Path::new(QUEUE).exists() {
        return Ok(Vec::new());
    }
    let text = util::read(Path::new(QUEUE))?;
    let data: Value = match serde_yaml_ng::from_str(&text) {
        Ok(v) => v,
        Err(e) => {
            errors.push(format!("{QUEUE}: not valid YAML: {e}"));
            return Ok(Vec::new());
        }
    };
    let map = match data {
        Value::Null => return Ok(Vec::new()),
        Value::Mapping(m) => m,
        _ => {
            errors.push(format!(
                "{QUEUE}: top level must be a mapping with the single key `steps`"
            ));
            return Ok(Vec::new());
        }
    };
    if map.keys().any(|k| scalar(k) != "steps") {
        errors.push(format!(
            "{QUEUE}: top level must be a mapping with the single key `steps`"
        ));
        return Ok(Vec::new());
    }
    let steps = match map.get("steps") {
        None | Some(Value::Null) => return Ok(Vec::new()),
        Some(Value::Sequence(seq)) => seq,
        Some(_) => {
            errors.push(format!("{QUEUE}: `steps` must be a list"));
            return Ok(Vec::new());
        }
    };
    let mut out = Vec::new();
    for (i, step) in steps.iter().enumerate() {
        match step {
            Value::Mapping(m) => out.push(m.clone()),
            _ => errors.push(format!(
                "{QUEUE}: steps[{}] must be a mapping with id and rules",
                i + 1
            )),
        }
    }
    Ok(out)
}

fn parse_range(p: &Patterns, text: &str) -> Option<(u32, u32)> {
    let caps = p.range.captures(text.trim())?;
    let lo: u32 = caps[1].parse().ok()?;
    let hi: u32 = caps.get(2).map_or(Some(lo), |m| m.as_str().parse().ok())?;
    if lo < 1 || hi < lo {
        return None;
    }
    Some((lo, hi))
}

fn get<'a>(step: &'a Mapping, key: &str) -> Option<&'a Value> {
    step.get(key).filter(|v| !v.is_null())
}

fn check_queue(p: &Patterns, steps: &[Mapping], rules: &Rules, errors: &mut Vec<String>) {
    let mut seen: Vec<String> = Vec::new();
    for (i, step) in steps.iter().enumerate() {
        let where_ = format!("{QUEUE}: steps[{}]", i + 1);
        let extra: Vec<String> = step
            .keys()
            .map(scalar)
            .filter(|k| !STEP_KEYS.contains(&k.as_str()))
            .collect();
        if !extra.is_empty() {
            errors.push(format!(
                "{where_}: unknown keys {extra:?}; allowed: id, rules, brick, after"
            ));
        }
        let id = get(step, "id").map(scalar);
        match &id {
            Some(s) if p.id.is_match(s) => {
                if seen.contains(s) {
                    errors.push(format!("{where_}: duplicate id {s:?}"));
                }
            }
            _ => errors.push(format!(
                "{where_}: id must be kebab-case (got {:?})",
                id.clone().unwrap_or_default()
            )),
        }
        match get(step, "rules").and_then(Value::as_mapping) {
            Some(step_rules) if !step_rules.is_empty() => {
                for (domain, range) in step_rules {
                    let (domain, range) = (scalar(domain), scalar(range));
                    if parse_range(p, &range).is_none() {
                        errors.push(format!("{where_}: rules[{domain}] must look like \"3\" or \"1-4\" (got {range:?})"));
                    }
                    if !rules.contains_key(&domain) {
                        errors.push(format!("{where_}: rules names domain {domain:?}, but {SPEC}/{domain}.md does not exist"));
                    }
                }
            }
            _ => errors.push(format!(
                "{where_}: rules must be a non-empty mapping of <domain>: \"n\" or \"lo-hi\""
            )),
        }
        if let Some(after) = get(step, "after").map(scalar)
            && !seen.contains(&after)
        {
            errors.push(format!(
                "{where_}: after names {after:?}, which is not an earlier step"
            ));
        }
        if let Some(brick) = get(step, "brick")
            && !brick.is_string()
        {
            errors.push(format!("{where_}: brick must be a string"));
        }
        if let Some(s) = id {
            seen.push(s);
        }
    }
}

fn step_rule_set(p: &Patterns, step: &Mapping) -> Rules {
    let mut out: Rules = BTreeMap::new();
    if let Some(step_rules) = get(step, "rules").and_then(Value::as_mapping) {
        for (domain, range) in step_rules {
            if let Some((lo, hi)) = parse_range(p, &scalar(range)) {
                out.insert(scalar(domain), (lo..=hi).collect());
            }
        }
    }
    out
}

fn step_state(p: &Patterns, step: &Mapping, rules: &Rules, cited: &Rules) -> String {
    let wanted = step_rule_set(p, step);
    if wanted.is_empty() {
        return "MALFORMED, run `cargo poly spec lint`".to_string();
    }
    let empty = BTreeSet::new();
    let missing: Vec<String> = wanted
        .iter()
        .filter_map(|(d, ns)| {
            let have = rules.get(d).unwrap_or(&empty);
            let gone: Vec<String> = ns
                .iter()
                .filter(|n| !have.contains(n))
                .map(u32::to_string)
                .collect();
            (!gone.is_empty()).then(|| format!("{d} has no rule {}", gone.join(", ")))
        })
        .collect();
    if !missing.is_empty() {
        return format!(
            "WAITING ON SPEC ({}; see its OPEN: lines)",
            missing.join("; ")
        );
    }
    let total: usize = wanted.values().map(BTreeSet::len).sum();
    let have: usize = wanted
        .iter()
        .map(|(d, ns)| ns.intersection(cited.get(d).unwrap_or(&empty)).count())
        .sum();
    if have == 0 {
        "READY".to_string()
    } else if have < total {
        format!("IN PROGRESS ({have}/{total} rules have a citing test)")
    } else {
        "DONE, delete this step".to_string()
    }
}

fn fmt_rules(step: &Mapping) -> String {
    match get(step, "rules").and_then(Value::as_mapping) {
        Some(m) => m
            .iter()
            .map(|(d, r)| format!("{} {}", scalar(d), scalar(r)))
            .collect::<Vec<_>>()
            .join(", "),
        None => "?".to_string(),
    }
}

// --- verbs ------------------------------------------------------------------------

pub fn lint() -> Result<u8, String> {
    let p = patterns();
    let mut errors: Vec<String> = Vec::new();
    let (rules, _) = read_spec(&p, &mut errors)?;
    check_citations(&p, &rules, &mut errors)?;
    check_links(&p, &mut errors)?;
    let steps = load_queue(&mut errors)?;
    check_queue(&p, &steps, &rules, &mut errors);
    if !errors.is_empty() {
        println!("Spec lint found problems:");
        for e in &errors {
            println!("  - {e}");
        }
        return Ok(1);
    }
    let total: usize = rules.values().map(BTreeSet::len).sum();
    println!(
        "Spec lint passed: {} domain(s), {total} rule(s), {} queue step(s), links resolve.",
        rules.len(),
        steps.len()
    );
    Ok(0)
}

/// Never fails: a session start must not break on a spec problem.
pub fn status() -> u8 {
    match status_lines() {
        Ok(lines) => println!("{}", lines.join("\n")),
        Err(e) => println!("[spec] status could not run: {e}. Run `cargo poly spec lint`."),
    }
    0
}

fn status_lines() -> Result<Vec<String>, String> {
    let p = patterns();
    let mut errors: Vec<String> = Vec::new();
    let (rules, open) = read_spec(&p, &mut errors)?;
    let cited = cited_rules(&p)?;
    let steps = load_queue(&mut errors)?;
    let empty = BTreeSet::new();
    let mut lines: Vec<String> = Vec::new();

    if rules.is_empty() {
        lines.push(format!("[spec] no domains in {SPEC}/ yet. Nothing can be built until the developer asks for one."));
    } else {
        lines.push(format!(
            "[spec] {} domain(s). Rules with a citing test are what works today.",
            rules.len()
        ));
        for (domain, numbers) in &rules {
            let pinned = cited.get(domain).unwrap_or(&empty);
            let have = numbers.iter().filter(|n| pinned.contains(n)).count();
            let missing: Vec<String> = numbers
                .iter()
                .filter(|n| !pinned.contains(n))
                .map(u32::to_string)
                .collect();
            let mut line = format!(
                "  {domain}: {} rule(s), {have} cited, {} open question(s)",
                numbers.len(),
                open.get(domain).copied().unwrap_or(0)
            );
            if !missing.is_empty() {
                line.push_str(&format!("; uncited: {}", missing.join(", ")));
            }
            let stray: Vec<String> = pinned
                .iter()
                .filter(|n| !numbers.contains(n))
                .map(u32::to_string)
                .collect();
            if !stray.is_empty() {
                line.push_str(&format!("; cited but not in spec: {}", stray.join(", ")));
            }
            lines.push(line);
        }
        for domain in cited.keys().filter(|d| !rules.contains_key(*d)) {
            lines.push(format!(
                "  {domain}: cited by tests but {SPEC}/{domain}.md does not exist"
            ));
        }
    }

    if steps.is_empty() {
        lines.push(format!(
            "[queue] {QUEUE}: empty. Add a step when a domain has rules to build and the developer wants them built."
        ));
    } else {
        lines.push(format!(
            "[queue] {QUEUE}: {} step(s), in order.",
            steps.len()
        ));
        for (i, step) in steps.iter().enumerate() {
            let mut head = format!(
                "  {}. {}",
                i + 1,
                get(step, "id").map(scalar).unwrap_or_default()
            );
            if let Some(brick) = get(step, "brick") {
                head.push_str(&format!(" ({})", scalar(brick)));
            }
            head.push_str(&format!(" rules: {}", fmt_rules(step)));
            if let Some(after) = get(step, "after") {
                head.push_str(&format!(" [after {}]", scalar(after)));
            }
            lines.push(format!("{head}: {}", step_state(&p, step, &rules, &cited)));
        }
    }

    if !errors.is_empty() {
        lines.push(format!(
            "[spec] {} lint problem(s); run `cargo poly spec lint`.",
            errors.len()
        ));
    }
    Ok(lines)
}
