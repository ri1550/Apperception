#!/usr/bin/env bash
# Download the reference material described in README.md into this directory.
set -euo pipefail
cd "$(dirname "$0")"
OBS_VER=1.15.0
OBS_URL="https://github.com/obsidianmd/obsidian-releases/releases/download/v${OBS_VER}"

mkdir -p Atomic Obsidian
[ -d Atomic/atomic ] || git clone https://github.com/kenforthewin/atomic.git Atomic/atomic

cd Obsidian
[ -f "obsidian-${OBS_VER}.tar.gz" ]   || curl -L -o "obsidian-${OBS_VER}.tar.gz"   "${OBS_URL}/obsidian-${OBS_VER}.tar.gz"
[ -f "Obsidian-${OBS_VER}.AppImage" ] || { curl -L -o "Obsidian-${OBS_VER}.AppImage" "${OBS_URL}/Obsidian-${OBS_VER}.AppImage"; chmod +x "Obsidian-${OBS_VER}.AppImage"; }
[ -d "bin/obsidian-${OBS_VER}" ]      || { mkdir -p bin; tar -xzf "obsidian-${OBS_VER}.tar.gz" -C bin; }
for repo in obsidian-api obsidian-developer-docs obsidian-help obsidian-sample-plugin; do
  [ -d "$repo" ] || git clone --depth 1 "https://github.com/obsidianmd/${repo}.git" "$repo"
done
[ -d obsidian-omnisearch ] || git clone --depth 1 https://github.com/scambier/obsidian-omnisearch.git
echo "done"
