# Vaults
## Purpose
A [vault](00-glossary.md) is the unit of ownership: one folder the user can copy, move, back up, put under git, sync, and open with any other Markdown tool, holding one [knowledge base](00-glossary.md) as plain `.md` files. The files are the source of truth. Beside them, an `.apperception` folder holds the vault's settings, its [database](00-glossary.md) (the [cache](00-glossary.md), rebuilt from the files), and its history and synthetic material, none of which is ever the truth about an atom.
This domain covers the life of a vault (create, open, adopt, close), the [registry](00-glossary.md) that knows which vaults exist, independence between vaults, what the app may write into the folder, and what the user loses if the `.apperception` folder is deleted.

## Vocabulary
**Adopt** — open an existing folder of Markdown files as a vault for the first time.

## Data
A vault knows its location on disk, its name, and when it was last opened. Its root holds the atoms as Markdown files, one file per atom, with no subfolders for atoms. An `.apperception` folder at the root, like Obsidian's `.obsidian`, holds the vault's settings as plain JSON files that describe how the vault is set up and used, and the database. Nothing in one vault refers to anything in another vault.
The registry lives outside every vault and lists each known vault with its location and last-opened time; it also holds global settings. Removing a vault from the registry does not touch the vault's folder.
The database holds only the cache, which is rebuilt from the files whenever it is missing or stale. History, every revision of every atom the app has observed, and synthetic material, which is regenerable at model cost, are kept inside `.apperception` outside the database; both are durable and neither is ever the truth about an atom.
Deleting the database loses nothing but rebuild time. Deleting the whole `.apperception` folder loses settings, history, and synthetic material, and never an atom.

## Behavior rules
1. A vault is a folder of Markdown files, one per atom, with an `.apperception` folder inside it holding the vault's settings as plain files and its database.
2. The database holds only what can be rebuilt from the vault's files; deleting it loses nothing the user wrote and nothing the app cannot rebuild.
3. History and synthetic material are kept inside `.apperception` outside the database, and deleting them loses only history and synthetic material, never an atom.
4. A user may have any number of vaults, and nothing in one vault refers to anything in another.
5. The registry of vaults lives outside every vault, and removing a vault from the registry leaves its folder untouched.

## Open questions
- OPEN: The layout inside `.apperception`: the database file, the history store, the synthetic store (synthetic material as Markdown files with provenance in frontmatter is the candidate), and the names and shapes of the settings files (ledger 59).
- OPEN: Where source material files live: a `sources` folder beside the atoms, inside `.apperception`, or elsewhere (ledger 58).
- OPEN: What adopting a folder that contains subfolders does: refuse, move every file to the root and record folder paths as tags (renaming on title collision), or tolerate subfolders read-only.
- OPEN: What adopting does with files that lack an `id` in frontmatter: assign one and write it into the file, or leave the file untouched until the user edits it.
- OPEN: Two writers: how the app behaves when a file changes on disk while it is open (a sync tool, another editor), and the rule that it never writes a file it has not read since the last change (ledger 77).
- OPEN: Whether one app process may open a vault at a time, and how a second process is told no (ledger 77).
- OPEN: What happens at open when a registered vault's folder is missing, or `.apperception` is missing or damaged: rebuild the database silently, and warn when settings, history, or synthetic material are gone.
- OPEN: Whether the registry is a SQLite file in the XDG data directory, as expected, or another format (ledger 60).
- OPEN: The vault-open time budget on a multi-million-word vault; the developer said about three seconds at most, which means the cache persists between opens and reconciles by file timestamps rather than rebuilding (ledger 88).
- OPEN: Which files in the vault root the app ignores (dot-files, non-Markdown files) and whether it shows them.
