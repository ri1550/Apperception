# Commands
## Purpose
Every user action is a [command](00-glossary.md), so menus, the command palette, hotkeys, and any future automation all run the same thing by name. This domain owns the command registry, hotkeys, and the palette.

## Vocabulary
**Hotkey** — a key combination bound to a command.
**Command palette** — a searchable list of every command the user can run now.

## Data
A command has a stable identifier, a display name, optional default hotkeys, and a condition under which it is available. Hotkeys are user-changeable and saved with the vault's settings (see [settings.md](settings.md)).

## Behavior rules
Nothing yet.

## Open questions
- OPEN: Which commands exist in slice 1.
- OPEN: Whether default hotkeys copy Obsidian's for actions both apps have; the developer said yes but expects to change them (ledger 98).
- OPEN: Whether hotkeys are scoped so that a modal or the editor can shadow a global hotkey.
- OPEN: Whether hotkeys are stored per vault or globally.
- OPEN: Whether the palette supports pinned commands.
