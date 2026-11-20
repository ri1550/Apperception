# Glossary
Every term more than one domain uses, defined once. Domain files link here and never restate a definition; a term used by one domain lives in that domain's Vocabulary section. How a term gets in, moves, or leaves is in [README.md](README.md), "Shared terms and the glossary". Each entry is one line: `**Term** — one-sentence definition. See [<domain>.md](<domain>.md).`

**Vault** — a folder holding a Notes folder of Markdown files, one per atom, an Attachments folder, and an `.apperception` folder with the vault's settings and database. See [vaults.md](vaults.md).
**Notes folder** — the `Notes` folder inside a vault, which holds every atom as a Markdown file with no subfolders. See [vaults.md](vaults.md).
**Knowledge base** — the Markdown files in a vault's Notes folder, that is, its atoms; the only first-class material and the only material that must be future-proof. See [vaults.md](vaults.md).
**Atom** — one Markdown file in a vault, resembling a note: an immutable ID, a title that is its filename, timestamps, tags, and a body, all written by the user. See [atoms.md](atoms.md).
**Title** — the unique, human-readable name of an atom within its vault; its filename without the extension, and the target that links name. See [atoms.md](atoms.md).
**Link** — a reference in an atom's body to another atom by title, optionally to a heading or block within it, optionally with display text. See [links.md](links.md).
**Orphaned** — the state of an atom that has no links to or from any other atom. See [links.md](links.md).
**Tag** — a word in an atom's `tags` property or body, starting with `#` in the body, that groups atoms and may nest with `/`. See [tags.md](tags.md).
**Trashed** — the state of an atom the user has deleted but that can still be restored until its retention period ends. See [atoms.md](atoms.md).
**Cache** — everything the database rebuilds from the files and can delete without losing anything the user wrote: link maps, tag counts, headings, blocks, the search index. See [vaults.md](vaults.md).
**Database** — the store inside a vault's `.apperception` folder that holds the cache and nothing else, so it can always be rebuilt from the files. See [vaults.md](vaults.md).
**Registry** — the list of vaults an installation knows about, kept outside any vault, together with global settings. See [vaults.md](vaults.md).
**View** — one way of displaying content in the workspace, such as the editor, search results, or backlinks. See [workspace.md](workspace.md).
**Command** — a named action the user can run from the command palette, a hotkey, or a menu. See [commands.md](commands.md).
