<p align="center"><img src="docs/icon.png" width="96" alt="SideCal icon"></p>

<h1 align="center">SideCal</h1>

<p align="center"><b>A free sidebar calendar for macOS.</b><br>Your whole day on the edge of your screen. Time-block by dragging. See your next event in the menu bar.</p>

<p align="center">
<a href="https://github.com/kacpergorec/sidecal/releases/latest"><b>Download</b></a> ·
<a href="https://kacpergorec.github.io/sidecal/">Website</a> ·
<a href="https://kacpergorec.github.io/sidecal/documentation.html">Docs</a>
</p>

<p align="center">
<a href="https://github.com/kacpergorec/sidecal/releases/latest"><img src="https://img.shields.io/github/v/release/kacpergorec/sidecal?style=flat-square" alt="Latest release"></a>
<a href="https://github.com/kacpergorec/sidecal/stargazers"><img src="https://img.shields.io/github/stars/kacpergorec/sidecal?style=flat-square" alt="Stars"></a>
<a href="LICENSE"><img src="https://img.shields.io/github/license/kacpergorec/sidecal?style=flat-square" alt="License"></a>
<img src="https://img.shields.io/badge/macOS-14%2B-000000?style=flat-square&logo=apple&logoColor=white" alt="macOS 14+">
<img src="https://img.shields.io/badge/built%20with-Rust-dea584?style=flat-square&logo=rust&logoColor=white" alt="Built with Rust">
<img src="https://img.shields.io/badge/telemetry-none-34c759?style=flat-square" alt="No telemetry">
</p>

- **No telemetry.**
- **No network by default.** The only network call is an optional update check (off by default): one small file, once a day.
- **Built in Rust.**
- **Local only.** It reads your calendar through Apple's EventKit.
- **No account, no analytics, no Electron.**

## Features

| Feature | What it does |
| --- | --- |
| **24 hours in view** | The whole day fits the screen with a live "now" line. No scrolling. |
| **Time blocking** | Drag to create, move or resize events. Snaps to 15 minutes, saves to Apple Calendar. |
| **Auto-hide** | Slides off the screen edge when you move away, returns when you touch the edge. Left or right side. |
| **Edge preview** | A thin, faded strip of your day stays on the edge while hidden. |
| **Menu bar countdown** | "Design review (23m left)" while an event runs, "Standup in 12m" before the next. |
| **All your calendars** | iCloud, Google, Exchange and CalDAV, through the macOS Calendar app. |
| **Native look** | Light, dark or follow macOS. Twelve colours, island mode, highlight for the running event. |
| **Your arrangement** | Recolour calendars and events, set calendar priority, hide single events. |
| **Global shortcuts** | Show the sidebar, add an event or open settings from anywhere. |
| **Fully customizable** | Side, width, island mode, auto-hide, edge preview, theme, 12 colours, timeline and menu bar options, remappable shortcuts. |
| **AI assistants (MCP)** | `sidecal --mcp` is a local MCP server: Claude, Cursor or any MCP client can read your day and time-block it. No network. |
| **Optional updates** | Check from the menu bar and install in one click. Off by default. |
| **Tiny** | One native binary of about 7 MB. No background services. |

## Install

### Option 1: download (no tools needed)

1. Get the latest **SideCal-apple-silicon.zip** (M1 and newer) or **SideCal-intel.zip** from [Releases](https://github.com/kacpergorec/sidecal/releases/latest).
2. Unzip it and drag **SideCal.app** to **Applications**.
3. First launch: right-click the app, choose **Open**, then **Open** again. The app is not notarized with an Apple Developer ID, so macOS asks once.
4. Allow calendar access when asked.

If macOS says the app is damaged, run `xattr -dr com.apple.quarantine /Applications/SideCal.app`.

### Option 2: build from source

Needs macOS 14+, the Xcode Command Line Tools (`xcode-select --install`) and [Rust](https://rustup.rs).

```bash
git clone https://github.com/kacpergorec/sidecal.git
cd sidecal
./install.sh
```

The first build takes a few minutes. `./build-app.sh` builds `SideCal.app` without installing it.

## Use with AI assistants (MCP)

SideCal can run as a local [MCP](https://modelcontextprotocol.io) server, so any MCP-capable assistant (Claude, Cursor and others) can read your calendar and plan your day. It talks over stdio only, with no network.

Point your client at this command:

```
/Applications/SideCal.app/Contents/MacOS/sidecal --mcp
```

Examples:

```bash
# Claude Code
claude mcp add sidecal -- /Applications/SideCal.app/Contents/MacOS/sidecal --mcp
```

```json
{ "mcpServers": { "sidecal": { "command": "/Applications/SideCal.app/Contents/MacOS/sidecal", "args": ["--mcp"] } } }
```

The JSON block works for clients that use an `mcpServers` config, such as Claude Desktop and Cursor.

Tools: `list_calendars`, `get_events`, `create_event`, `update_event`. There is no delete tool. Events and calendars you hide in SideCal stay hidden from the assistant. Open SideCal once and allow calendar access first.

## Updates

Updates are optional. Use **Check for Updates…** in the menu bar menu, or turn on **Check for Updates Automatically** in Settings. SideCal fetches [`latest.json`](https://kacpergorec.github.io/sidecal/latest.json) from GitHub Pages once a day, verifies the download with SHA-256, and only installs when you click.

## Privacy

SideCal has no telemetry and no analytics. It makes no network calls unless you turn on the optional update check. Events are read through EventKit and stay in your calendar. Its own settings live in one JSON file on your Mac. You can verify this: the source is short and has no HTTP client.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[Apache 2.0](LICENSE). Keep the [NOTICE](NOTICE) file. Copyright 2026 Kacper Górec.
