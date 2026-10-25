<p><img src="docs/icon.png" width="88" height="88" alt="SideCal icon"></p>

# SideCal

A free, open-source sidebar calendar for macOS. See your whole day on the edge of your screen, time-block it
by dragging, and get a countdown to your next event in the menu bar. It works with every calendar macOS
syncs (iCloud, Google, Exchange).

Built with Rust and [Tauri](https://tauri.app). About 7 MB, no Electron, no account, no network access.

**Documentation: <https://kacpergorec.github.io/sidecal/>**

## Features

- A 24-hour timeline that always fits the screen, no scrolling
- Slides to the screen edge when you move away and leaves a faint preview of your day
- Create, move and resize events by dragging
- Menu bar countdown: `Design review (23m left)`
- Colours for events and calendars, light and dark themes
- Global shortcuts, launch at login

## Install

You need macOS 14 or newer, the Xcode Command Line Tools (`xcode-select --install`) and
[Rust](https://rustup.rs). Clone this repository and run:

```bash
./install.sh
open /Applications/SideCal.app
```

The first build takes a few minutes. macOS will ask for access to your calendars once.

## Good to know

- There are no prebuilt downloads yet: the app is not notarized, so you build it yourself.
- It is tested on an Intel Mac with macOS 15. Apple Silicon should work but is untested.
- Everything stays on your Mac. Settings live in one JSON file in `~/Library/Application Support`.

Questions and bug reports are welcome in the issues. To contribute, see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Copyright 2026 Kacper Górec. Licensed under the [Apache License 2.0](LICENSE).
You may use, modify and share it, but keep the [LICENSE](LICENSE) and [NOTICE](NOTICE) files, which credit
the original author, and mark your changes.
