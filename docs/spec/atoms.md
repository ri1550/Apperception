# Atoms
## Purpose
An [atom](00-glossary.md) is the building block of the [knowledge base](00-glossary.md): one unit of the user's own understanding, written only by the user. Nothing a model writes is ever an atom.
An atom is one Markdown file in the vault's [Notes folder](00-glossary.md). The file is the truth; the app reads it, writes it, and never holds a copy that outranks it. This domain covers what the file contains, the atom's identity and [title](00-glossary.md), its lifecycle (create, edit, rename, trash, restore, purge), and the protections that keep a save from destroying work.

## Vocabulary
**Frontmatter** — the YAML block at the top of an atom's file that carries its properties.

## Data
An atom is a file named `<title>.md`. Its title is its filename without the extension and is unique within its [vault](00-glossary.md). Its frontmatter carries `id` (immutable, assigned at creation), `created`, `modified`, and `tags`; these four and the title are the only properties an atom has. Everything after the frontmatter is the body, Markdown in the Obsidian flavour (see [markdown.md](markdown.md)), kept exactly as written.
An atom belongs to exactly one vault. It is either connected or [orphaned](00-glossary.md), a status derived from its links (see [links.md](links.md)), never stored. It may be [trashed](00-glossary.md), in which case it can be restored until its retention period ends and is then purged.
An atom may refer to source material by link but never contains it, and never contains synthetic material.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: Which protections against silent overwrite are rules: a save writes only to the atom the edit was made in and switching atoms flushes or discards pending saves first; on a mismatch with the file on disk (vaults rule 10) the user sees both versions; every save keeps the previous content recoverable. Raised 2026-11-19 after Atomic overwrote one note with another's content during a switch.
- OPEN: How a file is written so that a crash or power loss never leaves a half-written atom (write to a temporary file and rename into place, or another method).
- OPEN: What form the ID takes (a timestamp-derived value, a random identifier, or another scheme); only immutability is settled (ledger 24).
- OPEN: What history an atom keeps and how it is exposed: whether the database records every save, every external change, or both, and for how long (ledger 75).
- OPEN: The exact title constraints: the forbidden characters (filename-unsafe on any OS plus `#`, `^`, `|`) and the length ceiling, enforced at creation and rename (ledger 84).
- OPEN: Whether title uniqueness is case-insensitive, which the filesystems of macOS and Windows force.
- OPEN: Whether `created` and `modified` live in frontmatter, come from the file's own timestamps, or both, given that an external editor updates the file's timestamp but not its frontmatter.
- OPEN: Where trashed atoms go (a `.trash` folder in the vault as Obsidian offers, the system trash, or inside `.apperception`) and the default retention period (ledger 76).
- OPEN: Whether an atom with an empty body is allowed.
