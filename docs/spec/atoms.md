# Atoms
## Purpose
An [atom](00-glossary.md) is the building block of the [knowledge base](00-glossary.md): one unit of the user's own understanding, written only by the user. Nothing a model writes is ever an atom.
An atom is one Markdown file in the vault's [Notes folder](00-glossary.md). The file is the truth; the app reads it, writes it, and never holds a copy that outranks it. This domain covers what the file contains, the atom's identity and [title](00-glossary.md), its lifecycle (create, edit, rename, trash, restore, purge), and the protections that keep a save from destroying work.

## Vocabulary
**Frontmatter** — the YAML block at the top of an atom's file that carries its properties.

## Data
An atom is a file named `<title>.md`. Its title is its filename without the extension and is unique within its [vault](00-glossary.md). Its frontmatter carries `id` (immutable, assigned at creation), `created`, `modified`, and `tags`; these four and the title are the only properties an atom has. Everything after the frontmatter is the body, Markdown in the Obsidian flavour (see [markdown.md](markdown.md)), kept exactly as written.
An atom belongs to exactly one vault. It is either connected or [orphaned](00-glossary.md), a status derived from its links (see [links.md](links.md)), never stored. It may be [trashed](00-glossary.md), in which case it sits in `.apperception/trash` and can be restored until its retention period ends and is then purged.
An atom may refer to source material by link but never contains it, and never contains synthetic material.

## Behavior rules
1. A title may not contain `/`, `\`, `:`, `*`, `?`, `"`, `<`, `>`, `|`, `#`, `^`, `[`, or `]`, may not begin or end with a space or a dot, and is at most 200 characters long; this holds at creation and at rename.
2. Titles are unique within a vault regardless of letter case.
3. A trashed atom is moved to `.apperception/trash`, kept there for a retention period that is 30 days by default, and purged automatically when the period ends.
4. An atom may have an empty body.
5. An atom's history is a sequence of snapshots of the atom over time, and the user sets in settings how long history is kept.

## Open questions
- OPEN: Which protections against silent overwrite are rules: a save writes only to the atom whose editor produced it, with pending saves flushed or discarded before an editor is reused for another atom; every save keeps the previous content recoverable. Raised 2026-11-19 after Atomic overwrote one note with another's content during a switch; deferred for testing and depends on how saves and history are handled.
- OPEN: How a file is written so that a crash or power loss never leaves a half-written atom: direct write as Obsidian does, which the developer leans toward, or write to a temporary file and rename into place. Prototype, test and ADR.
- OPEN: What form the ID takes (a timestamp-derived value, a random identifier, a time-ordered identifier such as UUID version 7); only immutability is settled. Prototype, test and ADR (ledger 24).
- OPEN: What produces a history snapshot (atoms rule 5): every save, every external change the watcher sees, a timer, or a combination; deferred until saving is settled (ledger 75).
- OPEN: Whether `created` and `modified` live in frontmatter, come from the file's own timestamps, or both, given that an external editor updates the file's timestamp but not its frontmatter.
