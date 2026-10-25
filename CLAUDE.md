# SideCal

Free sidebar calendar for macOS. Rust + Tauri 2 backend, plain HTML/CSS/JS UI (no npm, no bundler). Apache 2.0.

## Workflow

- Commit and push straight to `main`. No feature branches or PRs unless asked.
- Commit messages: plain and descriptive. No `Co-Authored-By` or `Claude-Session` trailers (the owner does not want session links in public history).
- Release: Actions tab, Release workflow, Run workflow, enter a tag like `v0.2.0` (tag pushes from the cloud sandbox get a 403). The workflow stamps the version from the tag, builds Apple Silicon and Intel zips, publishes the release and writes `docs/latest.json`.
- The cloud sandbox is Linux: the app cannot be compiled or run there (EventKit/objc code is macOS-only). Say so when shipping Rust changes. Pure logic can be checked in a scratch crate.

## Layout

- `src/main.rs` window, tray, sliding/auto-hide, commands. `src/calendar.rs` EventKit bridge. `src/settings.rs` settings JSON.
- `src/mcp.rs` `sidecal --mcp`: stdio MCP server (list_calendars, get_events, create_event, update_event; deliberately no delete).
- `src/updater.rs` optional update check/install via `docs/latest.json`; shells out to `/usr/bin/curl`, off by default.
- `ui/` the sidebar and settings windows. `ui/theme.css` holds the design tokens (light and dark must both work).
- `docs/` the GitHub Pages site (deployed from `main` `/docs`): `index.html`, `documentation.html`, `style.css`, `site.js`.
- `build-app.sh`, `install.sh` local build and install. `.github/workflows/release.yml` CI release.

## Rules that matter

- Privacy is a selling point: no telemetry, no analytics, no network calls unless the user enables the optional update check. Do not add network code or heavy dependencies.
- Event and calendar titles are untrusted: never put them in `innerHTML`; use `textContent`.
- Auto-hide reveals only when the cursor touches the screen edge. The collapsed strip ignores mouse events so clicks pass through.
- Keep both light and dark themes working, left and right side, expanded and auto-hidden.
- Code style: match the surrounding code. Comments explain why, not what.

## Site notes

- The landing mockup mirrors the real UI (same tokens, entry tiers, colours). Keep it faithful when the app UI changes.
- The hero mockup is a pinned, scroll-driven story driven by `site.js`; timings live in CSS variable `--t`.
- The site has a strict CSP meta tag: no inline scripts, no external resources.
- When app behaviour or settings change, update `README.md`, `docs/documentation.html`, `docs/index.html` (including the FAQ JSON-LD) and `docs/llms.txt` together.

## Checks before pushing

- Site: open `docs/index.html` and `docs/documentation.html` with Playwright, check the console for errors, and take screenshots (light, dark, 390px wide).
- Rust (on a Mac): `cargo check`, fix every warning, then `./build-app.sh`.
