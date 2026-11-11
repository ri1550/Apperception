# Atoms
## Purpose
An [atom](00-glossary.md) is the building block of the [knowledge base](00-glossary.md): one unit of the user's own understanding, written only by the user. Nothing a model writes is ever an atom.
This domain covers what an atom is, its identity and [title](00-glossary.md), its lifecycle (create, edit, rename, trash, restore, purge), and what it must preserve for the plain-text round trip (see [plain-text.md](plain-text.md)).

## Vocabulary
Nothing yet.

## Data
An atom has an immutable ID assigned at creation, a title that is unique within its [vault](00-glossary.md), a created time, a modified time, a set of [tags](00-glossary.md), and a Markdown body in the Obsidian flavour (see [markdown.md](markdown.md)). These five properties are the only properties an atom carries.
An atom belongs to exactly one vault. It is either connected or [orphaned](00-glossary.md), a status derived from its links (see [links.md](links.md)), never stored. It may be [trashed](00-glossary.md), in which case it can be restored until its retention period ends and is then purged.
An atom may refer to source material by link but never contains it, and never contains synthetic material.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: What form the ID takes (a timestamp-derived value, a random identifier, or another scheme); only immutability is settled (ledger 24).
- OPEN: What history an atom keeps (every revision, periodic snapshots, or none) and how it is exposed; this must be settled before the first real vault is written, because it cannot be added for past edits (ledger 75).
- OPEN: The exact title constraints: the forbidden characters (filename-unsafe on any OS plus `#`, `^`, `|`) and the length ceiling, enforced at creation and rename (ledger 84).
- OPEN: Whether title uniqueness is case-insensitive.
- OPEN: The default retention period for trashed atoms, and whether purge runs automatically or only when the user asks (ledger 76).
- OPEN: Whether created and modified can be set by the user or by import, or are always set by the app.
- OPEN: Whether an atom with an empty body is allowed.
