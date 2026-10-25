# SideCal: the free sidebar calendar for macOS

**See your whole day at a glance, block your time by dragging, and never lose track of the next meeting.**

SideCal is a free, open-source **sidebar calendar** (a *side calendar*) for macOS. It shows today's
Apple Calendar as a 24-hour timeline on the edge of your screen, adds a **menu bar countdown** to your
next event, and lets you **time-block** your day by dragging events around. It works with every calendar
macOS syncs: iCloud, Google, Exchange and CalDAV. When you move the mouse away it slides to the edge and
leaves a faint strip of your day behind; move the mouse back and it opens again.

Built with Rust and [Tauri](https://tauri.app) as a lightweight productivity tool: no Electron, no account,
no network access.

- About 7 MB binary, about 100 MB of RAM including the WebKit helper processes. No Electron, no npm.
- Reads and writes through the system calendar database (EventKit). No account, no sign-in.
- No network access, no analytics, no telemetry. Settings live in one JSON file on your Mac.
- Open source under the Apache License 2.0.

**Keywords:** sidebar calendar, side calendar, macOS calendar, menu bar calendar, time blocking, timeboxing,
daily planner, productivity tool, Apple Calendar, open source.

Project website and documentation: published with GitHub Pages from the [`docs/`](docs) folder.

## Features

- **24 hours, always visible.** The timeline is fitted to the window height, so there is nothing to scroll.
- **Auto-hide with an edge preview.** The sidebar collapses to a thin strip that keeps your events as faint
  bars, with the current time marked. Width and opacity of the strip are configurable.
- **Menu bar countdown.** `Design review (23m left)` while an event runs, `Standup in 12m` before the next one.
- **Edit by dragging.** Drag on an empty spot to create an event, drag an entry to move it, drag its top or
  bottom edge to resize. Everything snaps to 15 minutes.
- **Colours.** Pick a colour per event or override a whole calendar from a palette of 12, each with a light
  and a dark variant. Text colour is chosen automatically for contrast.
- **Calendar priority.** Order your calendars; the higher one wins the left column and sits on top when
  events overlap.
- **Hide events.** Hide an event from the sidebar (it stays in your calendar) and reveal hidden events on demand.
- **Highlight the running event** with a ring on the entry and a soft glow along the sidebar edge.
- **Island mode.** The open sidebar floats as a rounded card with a transparent margin around it.
- **Light, dark or auto**, following macOS.
- **Global shortcuts**, launch at login, left or right edge, adjustable width.

## Requirements

- macOS 14 (Sonoma) or newer
- Xcode Command Line Tools: `xcode-select --install`
- Rust (stable): <https://rustup.rs>

SideCal is developed and tested on an Intel Mac running macOS 15. It should build on Apple Silicon too,
but that has not been tested yet.

## Install

Clone this repository, then from its folder:

```bash
./install.sh
```

The script builds a release binary (the first build compiles about 400 crates, expect a few minutes; later
builds take about a minute), assembles `SideCal.app` and copies it to `/Applications`. Then:

```bash
open /Applications/SideCal.app
```

Only want to try it without installing? `./build-app.sh` builds `SideCal.app` in the project folder.

### First launch

1. macOS asks for access to your calendars. Choose **Allow Full Access**.
2. SideCal has no Dock icon. Look for it in the menu bar (it shows your next event).
3. Click the menu bar text to show or hide the sidebar. Right-click it for Settings and Quit.

The app is signed ad hoc, not with an Apple Developer ID, so macOS treats every rebuilt copy as a new app
and asks for calendar access again after you update it.

To start SideCal at login, move it to `/Applications` first, then enable **Launch at Login** in
Settings → General.

## Using it

| Do this | To get this |
|---|---|
| Click the menu bar text | Show or hide the sidebar |
| Drag on an empty spot of the timeline | Create an event (a form opens with your range) |
| Click an entry | Edit it |
| Drag an entry, or its top/bottom edge | Move or resize it |
| Right-click an entry | Colour, hide, delete (asks twice) |
| Right-click the empty board | Reload, show hidden events, Settings |
| Move the mouse to the screen edge | Reopen an auto-hidden sidebar |

### Shortcuts

Defaults, changeable in Settings → Shortcuts:

| Shortcut | Action |
|---|---|
| `Ctrl` `Alt` `Cmd` `S` | Show or hide the sidebar |
| `Ctrl` `Alt` `Cmd` `N` | New event |
| `Ctrl` `Alt` `Cmd` `,` | Open settings |

## Settings

Settings has six tabs.

- **Calendars**: show or hide each calendar, drag to set priority, change its colour, mark it as productive.
- **General**: launch at login, what the menu bar shows, how new events are suggested.
- **Appearance**: light, dark or auto; island mode; header, weekday and date; 12-hour clock; time indicator;
  all-day events; highlight of the running event.
- **Sidebar**: left or right edge, width, keep above other windows, auto-hide, edge preview (on/off, width, opacity).
- **Shortcuts**: global hotkeys.
- **Productivity**: hours spent in calendars you marked as productive, today and this week.

Settings are stored in `~/Library/Application Support/com.kacpergorec.sidecal/settings.json`. Custom event
colours and hidden events are stored there too, never in your calendar.

## How it works

```
Apple Calendar (EventKit)  <->  Rust (src/calendar.rs)  <->  Tauri commands  <->  HTML/CSS/JS in WebKit
```

| Path | What it does |
|---|---|
| `src/calendar.rs` | EventKit bridge: access, calendars, read, create, update, delete |
| `src/main.rs` | Windows, menu bar item, shortcuts, auto-hide and slide animation, commands |
| `src/settings.rs` | Settings model, JSON persistence, launch-at-login agent |
| `ui/index.html` | The sidebar |
| `ui/settings.html` | The settings window |
| `ui/palette.js`, `ui/theme.css` | Shared colours and design tokens |
| `docs/` | The project website (GitHub Pages) |
| `build-app.sh`, `install.sh` | Build the `.app` bundle, install it |

There is no bundler and no npm: the interface is plain HTML, CSS and JavaScript embedded into the binary at
build time. After changing anything in `ui/` or `src/`, rebuild with `./build-app.sh`.

## Known limitations

- Not notarized and not signed with a Developer ID, so there are no prebuilt downloads. Build it yourself.
- Tested on one Intel Mac with macOS 15. Apple Silicon, other macOS versions and unusual multi-display layouts
  are untested.
- Editing or deleting a repeating event changes only the occurrence you touched.
- A custom colour or a hidden flag follows the event's identifier. If a calendar provider re-issues that
  identifier after a sync, the event loses its colour or becomes visible again.
- Global shortcuts can collide with other apps. Change them in Settings → Shortcuts.

## Contributing

Issues and pull requests are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Copyright 2026 Kacper Górec.

SideCal is licensed under the [Apache License, Version 2.0](LICENSE). You are free to use, modify and
distribute it, including commercially. If you do, you must:

- keep the [LICENSE](LICENSE) and [NOTICE](NOTICE) files, which credit **Kacper Górec** as the original author;
- state clearly which files you changed;
- not use the author's name to endorse or promote your version without permission.

SideCal is provided "as is", without warranty of any kind.
