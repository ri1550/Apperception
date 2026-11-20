# Vaults
## Purpose
A [vault](00-glossary.md) is the unit of ownership: one folder the user can copy, move, back up, put under git, sync, and open with any other Markdown tool, holding one [knowledge base](00-glossary.md) as plain `.md` files in its [Notes folder](00-glossary.md). The files are the source of truth. Beside them, an `.apperception` folder holds the vault's settings, its [database](00-glossary.md) (the [cache](00-glossary.md), rebuilt from the files), and its history and synthetic material, none of which is ever the truth about an atom.
This domain covers the life of a vault (create, open, adopt, close), the [registry](00-glossary.md) that knows which vaults exist, independence between vaults, what the app may write into the folder, and what the user loses if the `.apperception` folder is deleted.

## Vocabulary
**Adopt** — open an existing folder of Markdown files as a vault for the first time.

## Data
A vault knows its location on disk, its name, and when it was last opened. Its root holds a `Notes` folder with the atoms as Markdown files, one file per atom and no subfolders; an `Attachments` folder for source material files; and room for other folders settled later. An `.apperception` folder at the root, like Obsidian's `.obsidian`, holds the vault's settings as plain JSON files that describe how the vault is set up and used, and the database. Nothing in one vault refers to anything in another vault.
The registry lives outside every vault, as a JSON file, and lists each known vault with its location and last-opened time; it also holds global settings. Removing a vault from the registry does not touch the vault's folder.
The database holds only the cache, which is rebuilt from the files whenever it is missing or stale. History, every revision of every atom the app has observed, and synthetic material, which is regenerable at model cost, are kept inside `.apperception` outside the database; both are durable and neither is ever the truth about an atom.
Deleting the database loses nothing but rebuild time. Deleting the whole `.apperception` folder loses settings, history, synthetic material, and any atoms waiting in the trash (atoms rule 3); it never loses an atom in `Notes`.

## Behavior rules
1. A vault is a folder containing a `Notes` folder of Markdown files, one per atom with no subfolders, an `Attachments` folder for source material files, and an `.apperception` folder holding the vault's settings as plain files and its database.
2. The database holds only what can be rebuilt from the vault's files; deleting it loses nothing the user wrote and nothing the app cannot rebuild.
3. History and synthetic material are kept inside `.apperception` outside the database, and deleting them loses only history and synthetic material, never an atom.
4. A user may have any number of vaults, and nothing in one vault refers to anything in another.
5. The registry of vaults lives outside every vault, and removing a vault from the registry leaves its folder untouched.
6. Adopting a folder moves every Markdown file it contains, at any depth, into the vault's `Notes` folder; Apperception keeps no folder structure and creates none.
7. Adopting a folder assigns an ID to every Markdown file that lacks one and writes it into the file's frontmatter.
8. A Markdown file that appears in the `Notes` folder without an ID is given one, written into its frontmatter, when the app first sees it.
9. When an open atom's file changes on disk, the app merges the external change into the editor automatically and tells the user, as Obsidian does.
10. The app never writes an atom's file without having read its current content since the file last changed.
11. A vault whose folder cannot be found stays in the registry, is shown as unavailable, and offers a locate action that lets the user browse for the folder.
12. If `.apperception` is missing when a vault opens, the app recreates it with default settings and rebuilds the database; if its database or settings files are damaged, the app rebuilds or repairs them; both happen silently and no copy of a damaged file is kept.
13. The registry is a JSON file in the user's configuration directory.
14. Opening a vault shows the workspace within three seconds regardless of vault size; indexing continues in the background.
15. A vault's name is its folder's name; renaming the vault renames the folder.
16. A vault is created only in a new or empty folder; opening a folder that already contains files is an adoption.
17. Files in the vault that the app does not understand are ignored and never modified; they are hidden by default, with a setting to show them.
18. A setting lets the user replace the automatic merge (vaults rule 9) with a side-by-side choice whenever an open file changes on disk; by default the merge is automatic.
19. The folder paths of adopted files are discarded; they do not become tags or anything else.

## Open questions
- OPEN: What adoption does when two files from different folders have the same title (vaults rule 6 moves both into `Notes`): rename one, prompt, or refuse the adoption.
- OPEN: The layout inside `.apperception`: the database file, the history store, the synthetic store (synthetic material as Markdown files with provenance in frontmatter is the candidate), and the names and shapes of the settings files (ledger 59).
- OPEN: Whether adoption moves the non-Markdown files it finds into `Attachments` and what it does to embeds that referenced them by their old relative path.
- OPEN: Which other folders the vault root will hold besides `Notes`, `Attachments`, and `.apperception`; the developer has deferred them.
- OPEN: Whether one app process may open a vault at a time, and how a second process is told no (ledger 77).
