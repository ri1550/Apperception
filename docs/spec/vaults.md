# Vaults
## Purpose
A [vault](00-glossary.md) is the unit of ownership: one folder the user can copy, move, back up, put under git, sync, and open with any other Markdown tool, holding one [knowledge base](00-glossary.md) as plain `.md` files. The files are the source of truth. Beside them, an `.apperception` folder holds the vault's settings and its [database](00-glossary.md): the [cache](00-glossary.md), history, and synthetic material, none of which is ever the truth about an atom.
This domain covers the life of a vault (create, open, adopt, close), the [registry](00-glossary.md) that knows which vaults exist, independence between vaults, what the app may write into the folder, and what the user loses if the `.apperception` folder is deleted.

## Vocabulary
**Adopt** — open an existing folder of Markdown files as a vault for the first time.

## Data
A vault knows its location on disk, its name, and when it was last opened. Its root holds the atoms as Markdown files, one file per atom, with no subfolders for atoms. An `.apperception` folder at the root holds the vault's settings and the database. Nothing in one vault refers to anything in another vault.
The registry lives outside every vault and lists each known vault with its location and last-opened time; it also holds global settings. Removing a vault from the registry does not touch the vault's folder.
The database holds three kinds of thing. The cache is rebuilt from the files whenever it is missing or stale. History is every revision of every atom the app has observed; it is durable but never the current truth. Synthetic material is durable, regenerable at model cost, and never the truth.
Deleting the `.apperception` folder loses no atom. It loses history and synthetic material; the cache is rebuilt on the next open.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: The layout inside `.apperception`: one database file or several (cache, history, synthetic, each separately deletable), and whether settings are plain files, as Obsidian's `.obsidian/*.json` are, or rows in the database (ledger 59).
- OPEN: Where source material files live: a `sources` folder beside the atoms, inside `.apperception`, or elsewhere (ledger 58).
- OPEN: What adopting a folder that contains subfolders does: refuse, move every file to the root and record folder paths as tags (renaming on title collision), or tolerate subfolders read-only.
- OPEN: What adopting does with files that lack an `id` in frontmatter: assign one and write it into the file, or leave the file untouched until the user edits it.
- OPEN: Two writers: how the app behaves when a file changes on disk while it is open (a sync tool, another editor), and the rule that it never writes a file it has not read since the last change (ledger 77).
- OPEN: Whether one app process may open a vault at a time, and how a second process is told no (ledger 77).
- OPEN: What happens at open when a registered vault's folder is missing, or `.apperception` is missing or damaged: rebuild the cache silently, and warn that history and synthetic material are gone.
- OPEN: Whether the registry is a SQLite file in the XDG data directory, as expected, or another format (ledger 60).
- OPEN: The vault-open time budget on a multi-million-word vault; the developer said about three seconds at most, which means the cache persists between opens and reconciles by file timestamps rather than rebuilding (ledger 88).
- OPEN: Which files in the vault root the app ignores (dot-files, non-Markdown files) and whether it shows them.
