# Contributing to SideCal

Thanks for wanting to help. SideCal is a small project, so the process is light.

## Before you start

- For anything bigger than a small fix, open an issue first so we can agree on the approach.
- Keep the app small and quiet: no network access, no telemetry, no heavy dependencies, no npm.

## Setup

You need macOS 14+, the Xcode Command Line Tools and Rust (see the README). Then:

```bash
./build-app.sh                  # builds SideCal.app in the project folder
open SideCal.app
```

After changing anything in `src/` or `ui/`, rebuild. The first build takes a few minutes, later ones about a minute.

## Making a change

- Match the style of the surrounding code. Comments explain *why*, not *what*.
- Rust: run `cargo check` and fix every warning before you send a pull request.
- UI: plain HTML, CSS and JavaScript. Use the tokens in `ui/theme.css`, and keep both light and dark themes working.
- Try your change with the sidebar on the left and on the right, expanded and auto-hidden.
- Say in the pull request what you tested and on which Mac and macOS version.

## Licence

By contributing you agree that your work is released under the [Apache License 2.0](LICENSE), the same as the
rest of the project. Keep the existing copyright notices and the [NOTICE](NOTICE) file intact.
