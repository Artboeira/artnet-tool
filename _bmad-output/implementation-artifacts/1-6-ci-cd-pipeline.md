# Story 1.6: CI/CD Pipeline

Status: ready-for-dev

## Story

As a developer,
I want a GitHub Actions workflow that builds platform-native installers for Windows, macOS, and Linux on every git tag push,
so that releases are produced automatically without manual per-platform build steps.

## Acceptance Criteria

1. **Given** a git tag is pushed to the repository, **When** the GitHub Actions workflow triggers, **Then** it builds installers for Windows (`.msi`/`.exe`), macOS (`.dmg`), and Linux (`.AppImage`/`.deb`), **And** artifacts are uploaded as GitHub release assets.

2. **Given** macOS signing credentials are not configured, **When** the macOS build runs, **Then** the build completes successfully and produces an unsigned `.dmg` artifact, **And** a clear comment in the workflow file notes that notarization is stubbed pending Apple Developer account.

3. **Given** the workflow file, **When** reviewed, **Then** it uses `tauri-apps/tauri-action` with a matrix strategy across all three platforms, **And** the trigger is `on: push: tags: ['v*']`.

## Tasks / Subtasks

- [ ] Task 1: Create `.github/workflows/release.yml` at the repository root (AC: #1, #2, #3)
  - [ ] Create directory `.github/workflows/` at the repo root (NOT inside `artnet-tool/`)
  - [ ] Create `release.yml` with trigger `on: push: tags: ['v*']`
  - [ ] Add `permissions: contents: write` at the top level (required for `GITHUB_TOKEN` to create releases)
  - [ ] Add matrix strategy with `fail-fast: false` and three platform entries (see Dev Notes for exact YAML)

- [ ] Task 2: Configure job steps — toolchain, caching, Node, Linux deps (AC: #1)
  - [ ] Add `actions/checkout@v4` as first step
  - [ ] Add `actions/setup-node@v4` with `node-version: lts/*`
  - [ ] Add `dtolnay/rust-toolchain@stable` with `targets` set for macOS Universal only (see Dev Notes)
  - [ ] Add `swatinem/rust-cache@v2` with `workspaces: './artnet-tool/src-tauri -> target'`
  - [ ] Add Linux apt-get step gated with `if: matrix.platform == 'ubuntu-22.04'` (see exact package list in Dev Notes)
  - [ ] Add `npm install` step with `working-directory: artnet-tool`

- [ ] Task 3: Add macOS signing stub (AC: #2)
  - [ ] In the `tauri-apps/tauri-action` step, add commented-out macOS signing env vars
  - [ ] Add a comment above the vars: `# macOS notarization: uncomment and configure when Apple Developer account is active`
  - [ ] Signing vars to stub: `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, `APPLE_ID`, `APPLE_TEAM_ID`, `APPLE_PASSWORD`

- [ ] Task 4: Configure `tauri-apps/tauri-action@v0` step (AC: #1, #3)
  - [ ] Use `tauri-apps/tauri-action@v0` (the v0 tag supports Tauri 2.x)
  - [ ] Set `env: GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}`
  - [ ] Set `with.tagName: ${{ github.ref_name }}`
  - [ ] Set `with.releaseName: 'ARTNET-TOOL ${{ github.ref_name }}'`
  - [ ] Set `with.releaseDraft: true` (review before publishing — safer default)
  - [ ] Set `with.prerelease: false`
  - [ ] Set `with.args: ${{ matrix.args }}` (passes `--target universal-apple-darwin` for macOS)
  - [ ] Set `with.projectPath: './artnet-tool'` (required — source root is not repo root)

- [ ] Task 5: Final validation (AC: all)
  - [ ] Verify `.github/workflows/release.yml` exists at the repo root
  - [ ] Verify YAML is syntactically valid (no tabs, correct indentation)
  - [ ] Verify trigger is `on: push: tags: ['v*']` and NOT `on: push: branches`
  - [ ] Verify matrix has exactly 3 entries: `macos-latest`, `ubuntu-22.04`, `windows-latest`
  - [ ] Verify macOS signing env vars are present but commented out with explanation
  - [ ] Run `cargo test` — confirm still 26/26 Rust tests pass (no Rust changes in this story)

## Dev Notes

### Scope Boundary

**In scope:**
- `.github/workflows/release.yml` — the entire CI/CD pipeline for releases

**NOT in scope:**
- PR validation / lint CI — a separate workflow for that (future story or standalone addition)
- macOS notarization — stubbed only; requires Apple Developer account and provisioning secrets
- Code signing for Windows (Authenticode) — not required by this story
- Any Rust or TypeScript code changes — this story touches only the workflow YAML

### Critical: Project Root vs. `artnet-tool/` Subdirectory

The repository structure is:
```
ARTNET-TOOL/          ← repo root (where .github/ lives)
└── artnet-tool/      ← Tauri project root (package.json, src/, src-tauri/)
    ├── package.json
    ├── src/
    └── src-tauri/
        ├── Cargo.toml
        └── tauri.conf.json
```

This means:
- `.github/workflows/release.yml` goes at **repo root** — `ARTNET-TOOL/.github/workflows/release.yml`
- All `working-directory` for npm commands → `artnet-tool`
- `tauri-apps/tauri-action` `projectPath` input → `'./artnet-tool'`
- `swatinem/rust-cache` `workspaces` → `'./artnet-tool/src-tauri -> target'`

### Complete `release.yml` Reference

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

permissions:
  contents: write

jobs:
  release:
    strategy:
      fail-fast: false
      matrix:
        include:
          - platform: 'macos-latest'
            args: '--target universal-apple-darwin'
          - platform: 'ubuntu-22.04'
            args: ''
          - platform: 'windows-latest'
            args: ''

    runs-on: ${{ matrix.platform }}

    steps:
      - uses: actions/checkout@v4

      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: lts/*

      - name: Install Rust stable
        uses: dtolnay/rust-toolchain@stable
        with:
          # macOS Universal Binary requires both Apple Silicon and Intel targets
          targets: ${{ matrix.platform == 'macos-latest' && 'aarch64-apple-darwin,x86_64-apple-darwin' || '' }}

      - name: Cache Rust build artifacts
        uses: swatinem/rust-cache@v2
        with:
          workspaces: './artnet-tool/src-tauri -> target'

      - name: Install Linux system dependencies
        if: matrix.platform == 'ubuntu-22.04'
        run: |
          sudo apt-get update
          sudo apt-get install -y \
            libwebkit2gtk-4.1-dev \
            build-essential \
            curl \
            wget \
            file \
            libssl-dev \
            libgtk-3-dev \
            libayatana-appindicator3-dev \
            librsvg2-dev

      - name: Install frontend dependencies
        working-directory: artnet-tool
        run: npm install

      - name: Build and publish release
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
          # macOS notarization: uncomment and configure when Apple Developer account is active
          # ENABLE_CODE_SIGNING: ${{ secrets.APPLE_CERTIFICATE }}
          # APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
          # APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
          # APPLE_ID: ${{ secrets.APPLE_ID }}
          # APPLE_TEAM_ID: ${{ secrets.APPLE_TEAM_ID }}
          # APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
        with:
          tagName: ${{ github.ref_name }}
          releaseName: 'ARTNET-TOOL ${{ github.ref_name }}'
          releaseBody: 'See the assets to download this version and install.'
          releaseDraft: true
          prerelease: false
          args: ${{ matrix.args }}
          projectPath: './artnet-tool'
```

### `tauri-apps/tauri-action` Version Note

- Use `tauri-apps/tauri-action@v0` — this is the correct version for **Tauri 2.x**
- `@v5` (if you see it referenced) is for Tauri 1.x — do NOT use it
- `projectPath: './artnet-tool'` is required because the Tauri project is not at the repo root

### macOS Universal Binary

- The `args: '--target universal-apple-darwin'` flag tells the Tauri CLI to build a Universal Binary (fat binary)
- This requires both `aarch64-apple-darwin` (Apple Silicon) and `x86_64-apple-darwin` (Intel) Rust targets
- The `dtolnay/rust-toolchain` step installs both targets only on macOS (empty string for other platforms)
- The ternary in the matrix `targets` field uses GitHub Actions expression syntax — this is correct YAML

### Linux Dependencies Explanation

The `libwebkit2gtk-4.1-dev` package is required by Tauri 2.x on Linux (upgraded from `libwebkit2gtk-4.0-dev` used by Tauri 1.x). Without it, the Linux build will fail. The other packages (`libssl-dev`, `libgtk-3-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`) are required for the system tray, TLS, and icons.

### `releaseDraft: true` — Why

Setting `releaseDraft: true` creates the GitHub Release as a draft, not immediately published. This lets you:
1. Review the auto-generated release before it's public
2. Edit the release notes
3. Verify all artifacts were uploaded correctly before publishing

Change to `false` only once you trust the pipeline produces correct releases.

### Tauri App Config

From `artnet-tool/src-tauri/tauri.conf.json`:
- `productName`: `artnet-tool`
- `identifier`: `com.nodata.artnet-tool`
- `version`: `0.1.0`
- Bundle targets: `"all"` — Tauri will produce all available bundle types per platform

### No Rust or TypeScript Changes

This story creates only one file: `.github/workflows/release.yml`. No Rust source, no TypeScript source, no Cargo.toml changes. The `cargo test` baseline of 26/26 should remain unchanged — verify it after creating the file as a sanity check that nothing was accidentally touched.

### YAML Indentation Warning

GitHub Actions YAML is **space-indented only** — never use tabs. The most common breakage pattern: copy-pasting from a text editor that converts spaces to tabs. Verify with a YAML linter or use the GitHub Actions workflow editor which flags syntax errors.

### Previous Story Intelligence (Story 1.5)

From Story 1.5 completion:
1. **Rust test baseline is 26 tests** — `cargo test` must still show 26/26 after this story
2. **No active subsystem handles exist in AppState yet** — only `playback_sender` is wired; the CI pipeline doesn't need to know subsystem details
3. **`artnet-tool/` is the Tauri project subdirectory** — all Tauri and npm commands run from there, not repo root

### References

- Story AC source: [Source: _bmad-output/planning-artifacts/epics.md#Story 1.6]
- CI/CD architecture: [Source: _bmad-output/planning-artifacts/architecture.md#CI/CD: Tauri GitHub Action]
- tauri-apps/tauri-action docs: [tauri-apps/tauri-action on GitHub](https://github.com/tauri-apps/tauri-action)
- App config: [Source: artnet-tool/src-tauri/tauri.conf.json]
- Previous story: [Source: _bmad-output/implementation-artifacts/1-5-playback-engine-thread-isolation-scaffold.md#Dev Agent Record]

## Dev Agent Record

### Agent Model Used

claude-sonnet-4-6

### Debug Log References

### Completion Notes List

### File List
