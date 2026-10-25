#!/bin/sh
# Builds SideCal and installs it to /Applications (run again to update).
set -e
cd "$(dirname "$0")"
./build-app.sh
pkill -f "SideCal.app/Contents/MacOS" 2>/dev/null || true
sleep 1
rm -rf /Applications/SideCal.app
cp -R SideCal.app /Applications/SideCal.app
echo "Installed: /Applications/SideCal.app"
echo "Start it with:  open /Applications/SideCal.app"
