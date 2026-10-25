#!/bin/bash
set -euo pipefail

TARGET="${1:?Usage: package-macos.sh TARGET ARCH}"
ARCH="${2:?Usage: package-macos.sh TARGET ARCH}"
ARTIFACT="NekoSportsWorldTool-macos-$ARCH"
BIN="target/$TARGET/release/nekosportsworldtool"
APP="dist/NekoSportsWorldTool.app"

mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"
cp "$BIN" "$APP/Contents/MacOS/nekosportsworldtool"
chmod +x "$APP/Contents/MacOS/nekosportsworldtool"
cp LICENSE "$APP/Contents/Resources/LICENSE"
cp assets/fonts/OFL-1.1.txt "$APP/Contents/Resources/FONT-LICENSE.txt"

python3 - <<'PY'
import os
import plistlib
import tomllib
from pathlib import Path

version = tomllib.loads(Path("Cargo.toml").read_text(encoding="utf-8"))["package"]["version"]
info = {
    "CFBundleName": "NekoSportsWorldTool",
    "CFBundleDisplayName": "NekoSportsWorldTool",
    "CFBundleIdentifier": "io.github.two-brush.nekosportsworldtool",
    "CFBundleExecutable": "nekosportsworldtool",
    "CFBundlePackageType": "APPL",
    "CFBundleShortVersionString": version,
    "CFBundleVersion": version,
    "LSMinimumSystemVersion": os.environ.get("MACOSX_DEPLOYMENT_TARGET", "11.0"),
    "NSHighResolutionCapable": True,
}
with Path("dist/NekoSportsWorldTool.app/Contents/Info.plist").open("wb") as f:
    plistlib.dump(info, f)
PY

plutil -lint "$APP/Contents/Info.plist"
# Ad-hoc signing requires no Apple certificate; this is not notarization.
codesign --force --sign - "$APP"
codesign --verify --deep --strict --verbose=2 "$APP"
ditto -c -k --sequesterRsrc --keepParent "$APP" "dist/$ARTIFACT-app.zip"
tar czf "dist/$ARTIFACT.tar.gz" -C "$(dirname "$BIN")" "$(basename "$BIN")"
shasum -a 256 "dist/$ARTIFACT-app.zip" "dist/$ARTIFACT.tar.gz" > "dist/$ARTIFACT.sha256"
