# Workspace
## Purpose
The workspace is the arrangement of [views](00-glossary.md) on screen: tabs, sidebars, and splits, remembered between sessions so the user returns to where they were. Its shape is being settled by UI prototyping, so this domain starts almost entirely open.

## Vocabulary
**Leaf** — one slot in the workspace that holds one view.
**Tab group** — a set of leaves of which one is visible at a time.
**Split** — a container that lays out its children side by side or one above another.

## Data
The workspace is a tree of splits, tab groups, and leaves, with a main area and a left and right sidebar. Each leaf holds one view and the state that view needs to be restored, such as which atom is open. The layout is saved per vault.

## Behavior rules
Nothing yet.

## Open questions
- OPEN: Whether slice 1 shows one tab group with tabs or the full split tree, and whether the data model is the full tree from the start (ledger 123).
- OPEN: Which views exist in slice 1: an all-atoms list, a tag tree, orphaned atoms, recent atoms, search, backlinks, outline (ledger 124).
- OPEN: Whether tabs can be pinned, and what a pinned tab does with links.
- OPEN: Whether pop-out windows exist.
- OPEN: Whether views are created lazily when first shown, as Obsidian does since 1.7.2.
- OPEN: Whether there is a ribbon and a status bar.
