# Search
## Purpose
One search box finds atoms by what the user types, ranking lexical, semantic, and fuzzy matches together, with operators to narrow when the user wants precision. The reference points are Obsidian's query language, the Omnisearch plugin's ranking, and Atomic's hybrid search.
Atoms are what search returns by default. Source and synthetic material are reachable only through an explicit filter and never mixed into the default ranking.

## Vocabulary
**Operator** — a prefix in a query such as `tag:` or `path:` that restricts matches to one field or kind.

## Data
A query is text with optional operators, quoted phrases, exclusions, and grouping. A result names an atom, a score, and the excerpt that matched. In slice 1 only lexical matching exists; semantic ranking arrives with model providers.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: Which slice brings fuzzy (typo-tolerant) ranking and where its index lives; the developer does not know yet.
- OPEN: Which operators exist; Obsidian's set is `file: path: content: match-case: ignore-case: tag: line: block: section: task: task-todo: task-done:` plus `[property:value]`, `OR`, `-`, parentheses, and `/regex/`.
- OPEN: Which sort orders exist beyond relevance (title, modified, created, each direction).
- OPEN: The acceptable latency for results; the developer accepts a small delay (ledger 88).
- OPEN: Whether search covers trashed atoms.
- OPEN: How lexical and semantic scores are blended once both exist.
