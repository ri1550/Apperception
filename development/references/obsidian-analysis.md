# Obsidian 1.15.0: how it works

Reverse-engineering notes on the Linux x86_64 build in `Obsidian/bin/obsidian-1.15.0/`, written 2026-11-08 as reference for Apperception. Everything here is for study. Nothing from Obsidian's bundle may be copied into Apperception.

Each statement is marked by how I know it:

- (observed) read from the shipped bundle, its typings, or the fuse bytes
- (documented) stated in Obsidian's own help or developer docs, shipped in `obsidian-help/` and `obsidian-developer-docs/`
- (inferred) my conclusion from the two above; not verified at runtime

I did not run the app against a vault, so nothing here is runtime evidence. REA evidence ids (same-session, retained):

| Evidence | What |
|---|---|
| `ev_817a641799800896a65d2cacef548089c2ac1e52622dd1a8d3eb47833b408b66` | Electron fuse bytes of the `obsidian` binary. |
| `ev_3d12a7137fc52948272c1f3f00d4609938cc55e557b871aed555c518588d2209` | Whole-asar static analysis. Timed out at 300 s with partial coverage: 19 of 38 JavaScript files parsed, 1277 findings, 73 modules; the vendored libraries exhausted the budget before `app.js` was reached. |
| `ev_dad0068813fc4b2bb6a677123a4ed2cbc169fb9f6b1730b1241dd8464ee21856` | `main.js` and `starter.js` only. Complete parse: 2 BrowserWindow constructions, 48 literal `ipcMain` handler channels, 0 preload scripts, 0 contextBridge APIs, 1 sender-validation observation, 0 utility processes, 0 native addon bindings in these files. |
| `ev_3ca058641eecf2e43e5f876b0692a7156f470457fc87ebc340ca1271f61f0586` | `app.js` only. Parsed fully (948,007 AST nodes, 92 findings) but the semantic graph exceeded REA's result budget, so only static and module facts were retained. The string-level observations below come from grep over the extracted bundle, not from this graph. |

## 1. Packaging and process model

Obsidian is an Electron app with two asar archives (observed):

| Archive | Size | Role |
|---|---|---|
| `resources/app.asar` | 73 KB, 22 files | Electron bootstrap. `package.json` points `main` at `main.js`. Only two native modules: `btime` (file birth time) and `get-fonts`. Depends on `@electron/remote` 2.1.3. |
| `resources/obsidian.asar` | 24.5 MB, 368 files | The actual application. Loaded by the bootstrap and logged as "Loaded main app package". |

The main app archive splits into (observed, by file):

- `main.js` (212 KB): Electron main process. Vault registry, window management, `obsidian://` protocol handler, CLI registration, web-request filtering.
- `starter.js` / `starter.html` (439 KB): the vault picker shown before a vault opens.
- `app.js` (3.4 MB, one 625-line minified esbuild bundle) and `app.css` (598 KB): the renderer. Everything users think of as Obsidian lives here.
- `help.js`, `quicklook.js`, `sim.js`, `worker.js`, `enhance.js`, `i18n.js`.
- `lib/`: vendored CodeMirror 5 (`codemirror.js`, `vim.js`, `modes.min.js`), moment, i18next, MathJax, pdf.js, Prism, pixi.js (graph rendering), Turndown and Readability (HTML to Markdown for web clipping and paste), `defuddle.full.js`, `pele.min.js`, scrypt (Sync encryption key derivation).
- `i18n/`: one `mapping.txt` of 2601 string keys and one line-aligned translation file per locale. English strings are in the bundle, not in a separate file.
- `sandbox/`: the starter vault copied to disk when the user opens the "sandbox vault".
- `public/`: Inter Variable and Source Code Pro fonts.

The renderer is a single HTML page (observed, `index.html`). It loads CodeMirror 5 scripts, moment, i18next, scrypt, turndown, then `enhance.js`, `i18n.js`, `app.js`. CodeMirror 6 is bundled inside `app.js` (documented: the editor is CM6; CM5 is kept for legacy modes and Vim). The CSP allows only `style-src 'unsafe-inline' 'self' https://fonts.googleapis.com`.

Electron fuses on the `obsidian` binary (observed, REA):

| Fuse | State |
|---|---|
| RunAsNode | disabled |
| EnableCookieEncryption | enabled |
| EnableNodeOptionsEnvironmentVariable | disabled |
| EnableNodeCliInspectArguments | disabled |
| EnableEmbeddedAsarIntegrityValidation | disabled (not enforced on Linux anyway) |
| OnlyLoadAppFromAsar | disabled |
| GrantFileProtocolExtraPrivileges | enabled |
| WasmTrapHandlers | enabled |

So `--inspect` and `ELECTRON_RUN_AS_NODE` are ignored. Runtime observation of the renderer would need the app's own developer tools or a Frida session, not the Inspector flag.

Main-process behaviour I read directly in `main.js` (observed):

- The vault registry is an object keyed by random 16-char ids. Each entry has `path`, `ts` (last opened), and `open`. It is saved to `obsidian.json` in the global config dir (`~/.config/obsidian/` on Linux; the app logged an ENOENT for that file on first run).
- `obsidian://` is registered as the default protocol client. Actions parsed in main: a bare path (`open`), `vault/<name>/<file>`, `choose-vault`, `sync-setup`, and generic `action?query` which is forwarded to the renderer by injecting a `window.OBS_ACT` call. Opening a file path from the OS injects `window.OBS_OPEN_FILE`.
- The sandbox vault is created by copying `sandbox/` from the asar into a fixed path, wiping it first.
- "Register CLI" copies or symlinks `obsidian-cli` to `~/.local/bin/obsidian`, with Flatpak and AppImage special cases.
- Web-request hooks strip `x-frame-options`, `cross-origin-opener-policy`, and `frame-ancestors` from responses so the web viewer can embed pages, and rewrite `SameSite` on cookies in subframes. The hook functions are then replaced with no-ops so plugins cannot register their own.
- The permission handler allows only requests from the app's own origin, plus clipboard and fullscreen; `openExternal` is always denied.
- The about panel says "Copyright Dynalist Inc."

IPC channels used by the renderer (observed, `ipcRenderer.*("...")` in `app.js` and `starter.js`): `vault`, `vault-list`, `vault-open`, `starter`, `sandbox`, `get-sandbox-vault-path`, `open-file`, `open-url`, `file-url`, `trash`, `frame`, `set-menu`, `render-menu`, `update-menu-items`, `check-update`, `disable-update`, `update`, `insider-build`, `latest-public-version`, `version`, `set-language`, `set-icon`, `get-icon`, `policy`, `terms`, `help`, `documents-dir`, `desktop-dir`, `disable-gpu`, `create-browser-session`, `adblock-lists`, `adblock-frequency`, `cli`, `register-cli`, `plugin-script-root`, `relaunch`, `is-quitting`, `is-closing`, `copy-asar`. The list is small. Almost all logic is in the renderer; the main process is a thin shell around windows, vaults, and OS integration.

## 2. Storage model

This is the part Apperception has to get right, because it decides whether Apperception can open an existing Obsidian vault.

A vault is a folder (documented). Notes are `.md` files; anything else is an attachment. Obsidian watches the folder and picks up external edits. Vaults within vaults are discouraged because links are vault-relative.

Supported file types (documented, and the same extension strings appear in `app.js`): `.md`, `.base`, `.canvas` (JSON Canvas, an open spec at jsoncanvas.org), images `.avif .bmp .gif .jpeg .jpg .png .svg .webp`, audio `.flac .m4a .mp3 .ogg .wav .webm .3gp`, video `.mkv .mov .mp4 .ogv .webm`, and `.pdf`. Plugins can register more via `registerExtensions`.

Per-vault configuration lives in `.obsidian/` at the vault root (documented). The folder name is overridable ("Override config folder"). Files referenced by name in `app.js` (observed):

| File | Holds |
|---|---|
| `app.json` | Editor and file settings. Keys present in the bundle include `alwaysUpdateLinks`, `newFileLocation`, `newFileFolderPath`, `attachmentFolderPath`, `useMarkdownLinks`, `newLinkFormat`, `readableLineLength`, `strictLineBreaks`, `propertiesInDocument`, `userIgnoreFilters`, `promptDelete`, `trashOption`, `livePreview`, `defaultViewMode`, `spellcheck`, `vimMode`, `showLineNumber`, `foldHeading`, `foldIndent`, `showIndentGuide`, `tabSize`, `useTab`, `autoPairBrackets`, `autoPairMarkdown`, `smartIndentList`, `rightToLeft`, `showUnsupportedFiles`, `showInlineTitle`, `focusNewTab`, `pdfExportSettings`, `mobilePullAction`, `autoConvertHtml`. |
| `appearance.json` | `accentColor`, `theme` (system/light/dark), `cssTheme`, `baseFontSize`, `textFontFamily`, `monospaceFontFamily`, `interfaceFontFamily`, `enabledCssSnippets`, `nativeMenus`, `translucency`, `showRibbon`, `showViewHeader`. |
| `core-plugins.json`, `core-plugins-migration.json` | Which core plugins are enabled. |
| `community-plugins.json` | Enabled community plugin ids. Plugins live in `.obsidian/plugins/<id>/{manifest.json,main.js,styles.css,data.json}`. |
| `hotkeys.json` | User hotkey overrides keyed by command id. |
| `workspace.json`, `workspace-mobile.json` | The current layout tree. Rewritten on every file open. |
| `types.json` | Property name to property type assignments. |
| `data.json` | Per-plugin settings (inside each plugin folder). |

Global, cross-vault state lives in the OS config dir (documented): the vault registry, and File Recovery snapshots keyed by absolute path.

Caches live in IndexedDB, not on disk in the vault (documented). Object stores created by `app.js` (observed): `metadata`, `file`, `data`, `history`, `backups`, `icons`. The metadata cache is rebuilt from files if lost. File Recovery snapshots (`.md` and `.canvas` only) are taken at least 5 minutes apart and kept 7 days by default.

Deletion has three modes (documented, and `trashOption` is in `app.json`): system trash, a `.trash/` folder in the vault, or permanent.

## 3. Data model

The public typings in `obsidian-api/obsidian.d.ts` (11,539 lines, observed) describe the model plugins see, which is close to the real one.

`App` holds `vault`, `metadataCache`, `workspace`, `fileManager`, `keymap`, `scope`, `secretStorage`, `renderContext`.

`Vault` is the file layer. It wraps a `DataAdapter` (filesystem on desktop, Capacitor on mobile) and exposes a tree of `TAbstractFile` nodes, either `TFile` (with `stat {ctime, mtime, size}`, `basename`, `extension`) or `TFolder` (`children`). Operations: `create`, `createBinary`, `createFolder`, `read`, `cachedRead`, `readBinary`, `modify`, `append`, `process` (atomic read-modify-write), `copy`, `rename`, `delete`, `trash(system)`. Events: `create`, `modify`, `delete`, `rename(file, oldPath)`. `Vault.process()` exists because `read` then `modify` can clobber external edits; the developer docs tell plugin authors to prefer it.

`MetadataCache` is the derived index. For each file it holds a `CachedMetadata` with `links`, `embeds`, `tags`, `headings`, `sections` (typed blocks: paragraph, heading, list, code, callout, table, yaml, footnoteDefinition, and so on, each with a position), `listItems`, `blocks` (by `^id`), `footnotes`, `frontmatter`, `frontmatterLinks`, `referenceLinks`. Across files it holds `resolvedLinks` and `unresolvedLinks`, both maps of source path to target path to count. Events: `changed(file, data, cache)`, `deleted`, `resolve(file)`, `resolved`. Link resolution is `getFirstLinkpathDest(linkpath, sourcePath)`: shortest-path-wins by basename, scoped to the vault.

`FileManager` is the policy layer over the vault: where a new file goes (`getNewFileParent`), rename with link rewriting (`renameFile`), trash with prompt, `generateMarkdownLink` honouring the wikilink/markdown setting, `processFrontMatter`, and `getAvailablePathForAttachment`.

Properties (documented) are YAML frontmatter. Types are text, list, number, checkbox, date, datetime, tags. A property name has one type across the vault (`types.json`). `tags`, `aliases`, `cssclasses` are built in. Nested YAML, Markdown in values, and bulk editing are explicitly unsupported. Internal links inside property values must be quoted strings.

Tags (documented) are `#word`, case-insensitive, may contain letters, digits, `_`, `-`, `/` for nesting, and Unicode. A tag needs at least one non-digit character. Tags come from body text or the `tags` property.

Links (documented): wikilink `[[Note]]`, `[[Note|alias]]`, `[[Note#Heading]]`, `[[Note#^blockid]]`, and `![[...]]` to embed. Markdown links `[text](Note%20name.md)` are equivalent. Heading-anchor and block-anchor are Obsidian-specific. Block ids are `^` plus Latin letters, digits, dashes, either at line end or on their own line after a block. Linking to a nonexistent note creates it on click. Renames rewrite links across the vault when `alwaysUpdateLinks` is on.

Markdown flavour (documented): CommonMark plus GFM plus LaTeX plus Obsidian extensions (wikilinks, embeds, block refs, `%%comments%%`, `==highlight==`, `> [!type]` callouts with 13 named types, footnotes, task lists). Markdown inside HTML elements is deliberately not rendered.

## 4. Workspace model

(documented, with the matching classes observed in the typings)

The workspace is a tree. `WorkspaceItem` is the node type; parents are `WorkspaceSplit` (children laid out horizontally or vertically) or `WorkspaceTabs` (one child visible). Leaves are `WorkspaceLeaf`, each holding one `View`. The root has three fixed splits: `leftSplit`, `rightSplit` (sidedocks, max three levels deep, children must be tab groups) and `rootSplit` (unbounded nesting). Pop-out windows are `WorkspaceWindow`. Mobile uses `WorkspaceMobileDrawer` for the sides.

A leaf has `ViewState {type, state, active, pinned, group}` and separate ephemeral state (scroll, cursor). Since 1.7.2 every leaf starts as a `DeferredView` and the real view is constructed only when the tab becomes visible. Pinned leaves refuse navigation and open links elsewhere. Leaves can be put in a named group for linked views (local graph, backlinks, outline following a note). Layout is saved on a debounce to `workspace.json`. Tab stacks ("sliding panes") are a display mode of a tab group.

Views registered by core: markdown (with three modes: source, live preview, reading), file explorer, search, backlinks, outgoing links, outline, tags, bookmarks, graph, local graph, canvas, bases, pdf, image, audio, video, web viewer, properties, footnotes, file recovery, release notes, empty. Each is a `View` subclass with `getViewType`, `getDisplayText`, `getState`/`setState`, `onOpen`/`onClose`.

Renderer-side events fired via `trigger("...")` (observed, 70 distinct names) include `file-open`, `layout-change`, `layout-ready`, `active-leaf-change`, `editor-change`, `editor-paste`, `editor-drop`, `editor-menu`, `file-menu`, `files-menu`, `url-menu`, `css-change`, `resolve`, `resolved`, `changed`, `create`, `modify`, `delete`, `rename`, `window-open`, `window-close`, `quit`, `quick-preview`, `hover-link`, `external-watch`.

## 5. Extension model

`Plugin extends Component` (observed). `Component` gives load/unload lifecycle plus `registerEvent`, `registerDomEvent`, `registerInterval`, `addChild`, so everything a plugin registers is torn down on unload. On top of that a plugin can `addCommand` (with `callback`, `checkCallback`, `editorCallback`, hotkeys), `addRibbonIcon`, `addStatusBarItem`, `addSettingTab`, `registerView`, `registerExtensions`, `registerMarkdownPostProcessor`, `registerMarkdownCodeBlockProcessor`, `registerEditorExtension` (a raw CodeMirror 6 extension), `registerEditorSuggest`, `registerHoverLinkSource`, `registerObsidianProtocolHandler`, `registerBasesView`, `registerCliHandler`, and `loadData`/`saveData` for `data.json`.

Commands are the unit of user action. Every menu item, hotkey, palette entry, slash command, and CLI subcommand resolves to a command id. The i18n keys list 103 `commands.*` strings and 1303 `plugins.*` strings across 34 core plugin ids: `audio-recorder backlinks bases bookmarks canvas command-palette custom-css daily-notes editor-status file-explorer file-recovery footnotes-pane graph-view markdown-format-importer mermaid name note-composer open-with-default-app outgoing-links outline page-preview properties publish quick-switcher random-note search slash-command slides sync tag-pane templates translucency unique-note-creator web-viewer word-count workspaces` (observed from `i18n/mapping.txt`). Core plugins use the same API as community plugins; the difference is trust and bundling.

Security (documented): "Restricted mode" is on by default and blocks all community plugins. There is no permission model; a plugin has the app's full access. The main process's no-op replacement of `webRequest` and `protocol` interception (observed) is the one hard boundary.

Themes are CSS. `app.css` defines 1228 custom properties, 898 of them on `body`, with light and dark palettes on `.theme-light` and `.theme-dark` (observed). Snippets are extra CSS files in `.obsidian/snippets/`.

## 6. Design system (observed in app.css)

Worth studying because this is the "polished reference" part of the brief. The token layering is:

1. Raw palette per theme. Grey ramp `--color-base-00` (page) through `--color-base-100` (text), 13 steps. Eight hues `--color-red/orange/yellow/green/cyan/blue/purple/pink`, each with an `-rgb` twin. Accent is HSL from `--accent-h/s/l` so the user picker only changes three numbers; `--color-accent-1` and `-2` are derived shifts.
2. Semantic layer. `--background-primary`, `-primary-alt`, `-secondary`, `-secondary-alt`, `--background-modifier-border/hover/form-field/box-shadow/cover`, `--text-normal/muted/faint/accent/on-accent/error/success`, `--interactive-normal/hover/accent/accent-hover`, `--text-selection`, `--text-highlight-bg`.
3. Component tokens that reference the semantic layer, prefixed by component: `--bases-*` (over 70), `--canvas-*`, `--callout-*` (one colour per callout type), `--code-*` (syntax colours), `--checkbox-*`, `--dropdown-*`, `--embed-*`, `--footnote-*`, `--graph-*`, `--h1-size` through `--h6-*`, `--hotkey-*`, `--input-*`, `--link-*`, `--tab-*`, `--sidebar-*`, `--ribbon-*`, `--status-bar-*`, `--table-*`, `--titlebar-*`.
4. Scales. Spacing `--size-2-1` to `--size-4-18` (two grids, 2px and 4px base). Radii `--radius-s/m/l/xl`. Type `--font-ui-smaller` 12px, `-small` 13px, `-medium` 15px, `-large` 20px; `--font-smallest .8em` to `--font-small .933em`; heading sizes 1.618em down to 1em; weights 100 to 900 as named tokens. Icons `--icon-xs` 14px to `--icon-xl` 32px with per-size stroke widths. Motion `--anim-duration-superfast` 70ms to `-slow` .56s with five named easing curves. Z-index `--layer-cover` 5 to `--layer-dragged-item` 80. Blur `--blur-s/m/l` composed from radius, saturation, brightness tokens.

Light palette: base-00 `#ffffff`, base-100 `#222222`, red `#e93147`, blue `#086ddd`, purple `#7852ee`. Dark palette: base-00 `#1C1C1C`, base-100 `#dadada`, red `#fb464c`, blue `#027aff`, purple `#a882ff`. Dark mode lifts `--interactive-normal` to base-30 and derives secondary backgrounds with `color-mix(... white N%)`.

Interface font is Inter Variable; monospace is Source Code Pro; both are bundled.

## 7. Feature inventory

From the help vault page list and `i18n/mapping.txt` (documented). Grouped the way the product groups them.

Files and vault. Create, rename (F2, inline title, links rewritten), move, delete (three trash modes), file explorer with sort and drag, excluded files patterns, open an external `.md` without a vault (since 1.14), symbolic links.

Editing. Three modes (source, live preview, reading). Find and replace, multiple cursors, folding (headings and indents), slash commands, link autocompletion on `[[`, heading search `[[##`, block search `[[^^`, properties editor at the top of the note, spellcheck, Vim mode, readable line length, auto-pair, smart lists, RTL, drag-and-drop of files to embed, paste HTML as Markdown, attachments with four location policies.

Linking. Wikilinks and Markdown links, aliases, heading and block anchors, embeds of notes, images (with size), audio, video, PDF (with page), canvases, lists, search results (`query` code block), unlinked mentions, backlinks pane, outgoing links pane, page preview on hover.

Navigation. Tabs, tab groups, splits, stacked tabs, pinned tabs, linked views, pop-out windows, left and right sidebars, ribbon, status bar, command palette with pinned commands, quick switcher, hotkeys editor, bookmarks (files, folders, headings, blocks, searches, graphs, groups), workspaces (saved layouts), random note, navigation history per tab.

Organising. Tags with nesting and a tag pane, properties with types and a properties pane, daily notes with date-format folders and templates, templates with `{{title}} {{date:fmt}} {{time:fmt}}`, unique note creator, note composer (merge, extract to new note), bases (table, list, cards, kanban, map views over frontmatter with filters, formulas, groups, summaries; stored as `.base` YAML), canvas (JSON Canvas: text, file, link, group nodes, edges with labels and colours, export to image).

Search. Full-text with operators `file: path: content: match-case: ignore-case: tag: line: block: section: task: task-todo: task-done:`, property queries `[prop:value]`, boolean `OR`, `-` negation, parentheses, `/regex/`, six sort orders, result embedding, "explain search term".

Graph. Global and local graph (pixi.js), filters by search, tags, attachments, orphans, existing-only; colour groups by search; display and force sliders; time-lapse.

Rendering. MathJax, Mermaid, Prism code highlighting, PDF viewer (pdf.js), footnotes pane, callouts, outline pane, word count, slides (reveal.js), web viewer (in-app browser with ad blocking), audio recorder, format converter.

Platform. Obsidian URI (`open`, `new`, `daily`, `search`, `hook-get-address`, x-callback-url), a CLI with roughly 130 subcommands including an `eval` for plugin devs, Sync and Publish (paid services, encrypted with scrypt-derived keys), web clipper (separate extension), importer plugin for Notion, Evernote, Roam, Logseq, Bear, Craft, Keep, OneNote, Apple Notes, and others.

## 8. What this means for Apperception

Inferences, not observations.

- Obsidian's architecture is "files are the database, plus a rebuildable cache". There is no index that must be trusted. Every feature degrades to re-reading `.md` files. That is the reason a vault survives being edited by other tools and synced by dumb file sync. Apperception should decide early whether it keeps this property; see the decision map.
- The `.obsidian/` folder is a de-facto public format. If Apperception can read `app.json`, `appearance.json`, `workspace.json`, `hotkeys.json`, `types.json`, and the plugin folders, a user can point it at an existing vault and keep both apps open. If it writes its own config elsewhere, the two can still coexist because Obsidian ignores unknown files.
- The metadata cache is the single most reused structure. Search, graph, backlinks, outline, bases, link resolution, rename rewriting, and tag pane all read `CachedMetadata`. Whatever Apperception's equivalent is, it should be one component with one interface.
- Commands are the universal verb. Menu, palette, hotkey, slash command, URI, and CLI are all front-ends to the command registry. Designing that first makes every later feature cheaper.
- The workspace tree (split, tabs, leaf, view state, ephemeral state, deferred views) is where most of the "polish" lives: it is what makes layouts survive restarts and makes linked panes work. It is also the piece Atomic does not have.
- Obsidian's extension model gives plugins the whole app. Apperception, being native rather than a browser, can offer a real sandbox (WASM, a scripting language, or an RPC boundary) if it wants to, but that is a product decision.

## 9. Open items from this analysis

- REA's semantic graph for `app.js` did not fit its result budget (see the evidence table at the top). A feature trace through the renderer (for example from a `trigger("file-open")` seed to its listeners) would need either a larger budget or source recovery with Wakaru first.
- No preload script and no contextBridge: the renderer has Node access directly (consistent with `@electron/remote` in `app.asar`). Main-process IPC is 48 `ipcMain` handlers, which matches the channel list in section 1 to within the few channels used only by `starter.js`.
- No runtime observation was done. Startup order, metadata cache persistence format, and how `workspace.json` is debounced are inferred from docs and typings.
- The English string catalogue inside the bundle was not extracted (only its key list). The help vault covers the same ground in prose.
- `.base` syntax, JSON Canvas, and Bases formulas were read at the overview level only. Their full grammars are in `obsidian-help/en/Bases/` and at jsoncanvas.org.
