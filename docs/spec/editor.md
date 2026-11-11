# Editor
## Purpose
The editor is where the user writes atoms. Live preview is the main view: formatting shows inline and the underlying syntax appears where the cursor is. Writing must never wait on the app.
This domain owns editing behaviour: modes, autocompletion of links and tags, find and replace, and the responsiveness rule. It does not own the construct list ([markdown.md](markdown.md)) or display in reading mode.

## Vocabulary
**Live preview** — the editing mode that renders Markdown inline while hiding most syntax until the cursor enters it.
**Source mode** — the editing mode that shows all Markdown syntax as written.

## Data
The editor edits one atom's body at a time and knows the cursor, the selection, and the undo history for that atom. It offers completion of titles when the user types `[[` and of tags when the user types `#`.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: Whether source mode exists at all in slice 1, given the developer never uses it, or only live preview.
- OPEN: Whether a reading view (no editing) exists in slice 1 or arrives with rendering in slice 2.
- OPEN: Which editing conveniences ship first: auto-pair brackets, smart lists, readable line width, line numbers, fold headings, spellcheck, Vim keys.
- OPEN: What "no delay while writing" means as a number that a test can check.
- OPEN: What pasting HTML or a file does in slice 1, before source material exists.
- OPEN: Whether find and replace works within one atom only in slice 1.
