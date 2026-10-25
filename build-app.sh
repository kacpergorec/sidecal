#!/bin/sh
# Builds "SideCal.app" without tauri-cli. -j 2 keeps compile RAM low.
set -e
cd "$(dirname "$0")"
~/.cargo/bin/cargo build --release -j "${JOBS:-2}"

APP="SideCal.app"
rm -rf "$APP"
mkdir -p "$APP/Contents/MacOS"
cp target/release/sidecal "$APP/Contents/MacOS/"
cp Info.plist "$APP/Contents/"
mkdir -p "$APP/Contents/Resources"
cp icons/icon.icns "$APP/Contents/Resources/"
# Ad-hoc signature so macOS shows the Calendar permission prompt.
codesign --force --sign - "$APP"
echo "Built: $(pwd)/$APP"
