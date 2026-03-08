# Scriptty — Claude Code Configuration

## Project Overview

Scriptty is an offline desktop screenwriting application built for Malayalam and English
screenwriters. It supports Hollywood single-column format and Indian two-column format
(export only). The app targets Malayalam filmmakers and writers, with built-in Malayalam
input methods requiring no external tools.

**Product name:** Scriptty  
**Primary domain:** scriptty.app  
**File extension:** .screenplay  

---

## Tech Stack

| Layer | Technology |
|---|---|
| Desktop framework | Tauri 2 |
| Frontend | Svelte 5 + TypeScript + Vite |
| Editor | ProseMirror |
| PDF generation | Typst (Rust crate) |
| Mozhi engine | Varnam JS |
| Backend language | Rust 1.93+ |
| Node.js | v22 / npm |

---

## Developer Context

The lead developer (Hrishi) is not a Rust expert. When writing Rust code:

- Always add inline comments explaining ownership, borrowing, and lifetime concepts when
  they appear
- Explain non-obvious Rust patterns briefly in a code comment
- Prefer clarity over cleverness in Rust code
- Never assume Rust knowledge — explain what each new pattern does the first time it appears
- When introducing a new Rust concept (e.g. `Result`, `Option`, `impl`, lifetimes), add a
  one-line comment explaining what it means in plain English

---

## Project Structure

```
scriptty/
├── src/                          # Svelte frontend
│   ├── lib/
│   │   ├── editor/               # ProseMirror screenplay editor
│   │   │   ├── schema.ts         # Screenplay element schema
│   │   │   ├── keymap.ts         # Tab/Enter navigation
│   │   │   └── input/            # Malayalam input engine
│   │   │       ├── InputModeManager.ts
│   │   │       ├── mozhi.ts
│   │   │       ├── inscript1.ts
│   │   │       └── inscript2.ts
│   │   ├── stores/               # Svelte stores (app state)
│   │   └── components/           # UI components
│   └── routes/                   # SvelteKit pages
├── src-tauri/                    # Rust backend
│   ├── src/
│   │   ├── main.rs               # Tauri entry point
│   │   ├── commands/             # Tauri commands (called from frontend)
│   │   │   ├── mod.rs
│   │   │   ├── file.rs           # save/open .screenplay files
│   │   │   └── export.rs         # PDF, Fountain, plain text export
│   │   ├── screenplay/           # Document model and business logic
│   │   │   ├── mod.rs
│   │   │   ├── document.rs       # .screenplay JSON schema
│   │   │   └── pdf.rs            # Typst PDF generation
│   │   └── fonts/                # Font loading for Typst
│   ├── fonts/                    # Bundled .ttf files (Noto Sans Malayalam, Manjari)
│   ├── Cargo.toml
│   └── tauri.conf.json
├── static/
│   └── fonts/                    # Fonts served to the Svelte UI
├── .claude/                      # Claude Code configuration
│   ├── CLAUDE.md                 # This file
│   └── agents/                   # Sub-agent definitions
└── package.json
```

---

## Architecture Decisions (Locked)

These are final. Do not suggest alternatives unless explicitly asked.

### Editor

- ProseMirror is the editor library — not TipTap, not CodeMirror, not contenteditable
- Editor always shows Hollywood single-column format
- Indian two-column is a PDF export option only — not an editor mode
- Element types: SceneHeading, Action, Character, Parenthetical, Dialogue, Transition

### Element Navigation (Tab/Enter behavior)

| Current element | Key | Next element |
|---|---|---|
| SceneHeading | Enter | Action |
| Action | Enter | Action (new paragraph) |
| Action | Tab | Character |
| Character | Enter | Dialogue |
| Dialogue | Enter | Action |
| Dialogue | Tab | Character |
| Parenthetical | Enter | Dialogue |
| Transition | Enter | SceneHeading |

### Malayalam Language Support

- Three input schemes: SMC Mozhi, Inscript 2, Inscript 1
- Ctrl+Space toggles English/Malayalam mode mid-sentence
- Mixed script per line is the default (e.g. "രമേഷ് Flat ലേക്ക് നടന്നു")
- Varnam JS handles Mozhi — do not write custom Mozhi rules from scratch
- Inscript 1 and Inscript 2 are static keymaps (~100 lines each)
- InputModeManager.ts is the single source of truth for input mode state
- Input scheme is user-level config, not document-level

### Fonts

- Bundled fonts: Noto Sans Malayalam (default), Manjari (alternative)
- Both fonts licensed SIL OFL 1.1 — safe to bundle in commercial software
- Single font applies to ALL text — Malayalam and English both use the same font
- Font files live in two places:
  - `src-tauri/fonts/` — for Typst PDF embedding
  - `static/fonts/` — for the Svelte UI via CSS
- No system font dependency — app works on a fresh OS install

### PDF Export

- Typst Rust crate handles all PDF generation — not printpdf, not any other crate
- Flow: Rust receives ProseMirror JSON → generates Typst markup string → Typst compiles
  to PDF bytes in memory → bytes returned to frontend
- No temp files written to disk during export
- Selected font is embedded in every PDF
- Format (Hollywood or Indian two-column) is chosen at export time, not document creation

### File Format (.screenplay)

JSON with three top-level keys:

```json
{
  "content": {},
  "meta": {
    "title": "",
    "author": "",
    "contact": "",
    "draft_number": 1,
    "draft_date": "",
    "created_at": "",
    "updated_at": ""
  },
  "settings": {
    "font": "noto-sans-malayalam",
    "default_language": "malayalam",
    "input_scheme": "mozhi"
  }
}
```

`content` is the ProseMirror document JSON serialization.

### Export Formats (v1)

- **PDF** — Hollywood or Indian two-column layout, chosen at export time
- **Fountain** — UTF-8 plain text, interoperability with other tools
- **Plain text** — readable draft sharing

### Title Page

- Auto-generated at PDF export time from `meta` fields
- Not editable inside the editor
- Fields printed: title, author, contact, draft number, draft date
- Editor starts directly at FADE IN:

### Scene Navigator

- Collapsible left panel, toggle with Ctrl+B
- Shows: auto-computed scene number + scene heading text
- Scene numbers derived from document order — never stored
- Drag to rearrange scenes triggers auto-renumber
- Click to jump to scene

### Character Autocomplete

- Triggers after 2 characters typed in a Character element
- Suggests only names already present in the document
- Unicode-aware — Malayalam names and English names in the same pool
- Dismissed with Escape, accepted with Enter or Tab

---

## Commands

### Development

```bash
# Start frontend dev server only
npm run dev

# Start full Tauri dev (frontend + backend + opens window)
cargo tauri dev

# Type check frontend only (no emit)
npx tsc --noEmit

# Lint frontend
npm run lint

# Check Rust compilation without full build (fast)
cd src-tauri && cargo check

# Lint Rust code
cd src-tauri && cargo clippy

# Format Rust code
cd src-tauri && cargo fmt
```

### Build

```bash
# Production build — outputs installer to src-tauri/target/release/bundle/
cargo tauri build
```

---

## Coding Standards

### TypeScript / Svelte

- TypeScript strict mode — no `any` types anywhere
- Svelte 5 runes syntax only: `$state`, `$derived`, `$effect`, `$props`
- Do not use legacy Svelte 4 syntax (`onMount` is acceptable, reactive `$:` is not)
- All Tauri command calls must handle errors explicitly — never assume success
- Use `invoke` from `@tauri-apps/api/core` for all Rust command calls

### Rust

- Follow standard Rust naming: `snake_case` for functions/variables, `PascalCase` for
  types and structs
- Never use `unwrap()` in Tauri command handlers — always use `?` or match on errors
- All public functions must have a doc comment (`///`)
- Return `Result<T, String>` from all Tauri commands so errors surface cleanly to the
  frontend
- Use `serde` for all JSON serialization/deserialization

---

## What is Tauri? (Reference)

Tauri is a desktop app framework. The frontend (Svelte) runs inside the OS's built-in
WebView (WebKit on Mac, WebView2 on Windows, WebKitGTK on Linux). The backend is a
compiled Rust binary. The two communicate via **commands** — the frontend calls a Rust
function using `invoke()`, Rust executes it and returns a result.

This means:
- UI logic, editor, input methods → Svelte/TypeScript
- File I/O, PDF generation, OS integration → Rust
- No server, no network, fully offline

---

## Deferred to Phase 2 (Do Not Implement in v1)

- Revision tracking / colored revision pages
- FDX (Final Draft XML) export
- Courier font / Hollywood submission mode
- Rachana font / traditional Malayalam orthography
- Beat board / scene cards
- Statistics report (character dialogue count, scene lengths)
- Import from Final Draft / Fountain
- Real-time collaboration
- Cloud sync
- Mobile support

---

## Key Constraints (Non-negotiable)

- Fully offline — zero network calls at runtime
- No server setup required for end users
- Single installable binary — works on macOS, Windows, Linux out of the box
- Malayalam rendering via bundled fonts only — no OS font dependency
- Malayalam input works without any OS IME installation
- All data stored locally in .screenplay files — no database, no cloud
