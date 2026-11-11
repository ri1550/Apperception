# Vaults
## Purpose
A [vault](00-glossary.md) is the unit of ownership: one directory the user can copy, move, back up, and delete as a whole, holding one [knowledge base](00-glossary.md). The database inside it is the source of truth; plain-text files are import and export, never storage (see [plain-text.md](plain-text.md)).
This domain covers the life of a vault (create, open, close), the [registry](00-glossary.md) that knows which vaults exist, independence between vaults, and the rule that the [cache](00-glossary.md) is disposable.

## Vocabulary
Nothing yet.

## Data
A vault knows its location on disk, its name, and when it was last opened. It holds exactly one knowledge base, one cache, and its own settings (see [settings.md](settings.md)). Nothing in one vault refers to anything in another vault.
The registry lives outside every vault and lists each known vault with its location and last-opened time; it also holds global settings. Removing a vault from the registry does not touch the vault's directory.
The cache is derived from the knowledge base and can be deleted at any time; the vault rebuilds it.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: The exact on-disk layout of a vault directory (truth database, cache, internal source files, settings) is a prototype-and-ADR question (ledger 57).
- OPEN: Whether the cache lives in its own file beside the truth database or in tables inside it (ledger 59).
- OPEN: Where internal source files are stored once source material exists: blobs in the database, content-addressed files beside it, or elsewhere (ledger 58).
- OPEN: Whether one process may open a vault at a time, or several readers and one writer, and what a lock looks like (ledger 77).
- OPEN: How a vault is backed up and moved safely while the database is the truth: close-and-copy, a backup command, the plain-text mirror, or all three (ledger 78).
- OPEN: What happens at open when a registered vault's directory is missing or its database is damaged.
- OPEN: Whether the registry is a SQLite file in the XDG data directory, as expected, or another format (ledger 60).
- OPEN: The vault-open time budget on a multi-million-word vault; the developer said about three seconds at most (ledger 88).
