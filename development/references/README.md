# References
Third-party material Apperception is built against. Nothing here is part of the workspace: the root `Cargo.toml` excludes this directory and `.gitignore` keeps everything but this file and `fetch.sh` out of git. Run `./fetch.sh` from this directory to download it all.

## Atomic/ (primary reference, MIT)
- `atomic/` — https://github.com/kenforthewin/atomic (https://atomicapp.ai). Open-source note-taking app: Tauri + Rust backend, TypeScript/Vite frontend, Capacitor mobile. Full git history kept so `git log` is usable as context.

## Obsidian/ (design reference, closed source)
- `obsidian-1.15.0.tar.gz`, `bin/obsidian-1.15.0/` — the Linux x86_64 build, extracted. The app itself is `bin/obsidian-1.15.0/resources/obsidian.asar` (Electron asar archive, ~24 MB); `app.asar` is the small Electron bootstrap. These are the reverse-engineering targets for REA.
- `Obsidian-1.15.0.AppImage` — the same release as a runnable AppImage, for runtime observation.
- `obsidian-api/` — https://github.com/obsidianmd/obsidian-api. The official TypeScript typings (`obsidian.d.ts`) for the plugin API: the public surface of the app's internals.
- `obsidian-developer-docs/` — https://github.com/obsidianmd/obsidian-developer-docs. Plugin/theme developer documentation.
- `obsidian-help/` — https://github.com/obsidianmd/obsidian-help. The user-facing help vault: the feature inventory in Obsidian's own words.
- `obsidian-sample-plugin/` — https://github.com/obsidianmd/obsidian-sample-plugin. Minimal plugin, shows the plugin lifecycle.

Obsidian is proprietary. Its binaries are here for study only and must never be redistributed or copied from.
- `obsidian-omnisearch/` — https://github.com/scambier/obsidian-omnisearch (v1.32.0). Community search plugin named as a retrieval reference. GPL-3: study only, nothing may be copied into Apperception (MIT).
