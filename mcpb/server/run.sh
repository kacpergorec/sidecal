#!/bin/sh
# Claude Desktop starts this. It hands over to the installed SideCal app, which owns the calendar permission,
# so one extension covers Apple Silicon and Intel.
for APP in /Applications/SideCal.app "$HOME/Applications/SideCal.app"; do
  if [ -x "$APP/Contents/MacOS/sidecal" ]; then exec "$APP/Contents/MacOS/sidecal" --mcp; fi
done
echo "SideCal.app was not found in Applications. Install it first: https://kacpergorec.github.io/sidecal/" >&2
exit 1
