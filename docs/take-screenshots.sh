#!/bin/bash
# Screenshot capture for syslenz README
# Uses termshot or manual capture
#
# Option 1: Install vhs and use demo.tape
#   go install github.com/charmbracelet/vhs@latest
#   vhs docs/demo.tape
#
# Option 2: Manual screenshots with this script
#   This script runs syslenz in different views and you take screenshots manually
#
# Option 3: Use ttyd for browser-based capture
#   ttyd -p 7681 target/release/syslenz
#   Open http://localhost:7681 and use browser screenshot tools

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ASSETS_DIR="$SCRIPT_DIR/assets"

cd "$PROJECT_DIR"

echo "=== syslenz Screenshot Guide ==="
echo ""
echo "Build first..."
source ~/.cargo/env 2>/dev/null || true
cargo build --release 2>/dev/null

echo ""
echo "Screenshots needed:"
echo "  1. docs/assets/dashboard.png    — Dashboard view (default)"
echo "  2. docs/assets/classic.png      — Classic mode with meminfo selected"
echo "  3. docs/assets/detail.png       — Detail view with fields"
echo "  4. docs/assets/diff.png         — Diff view showing changes"
echo "  5. docs/assets/graph.png        — Graph view with sparkline"
echo "  6. docs/assets/diagnostics.png  — Diagnostics view"
echo "  7. docs/assets/help-extra.png   — Help panel at EXTRA level"
echo "  8. docs/assets/category.png     — Category Guide view"
echo "  9. docs/assets/web-ui.png       — Web UI in browser"
echo ""
echo "Method 1: Use vhs (recommended)"
echo "  go install github.com/charmbracelet/vhs@latest"
echo "  vhs docs/demo.tape"
echo ""
echo "Method 2: Use ttyd (browser-based)"
echo "  ttyd is already installed. Run:"
echo "    ttyd -p 7681 target/release/syslenz"
echo "  Then open http://localhost:7681 and take screenshots"
echo ""
echo "Method 3: Manual"
echo "  Run: target/release/syslenz"
echo "  Navigate to each view, take screenshots with OS tools"
echo ""

# If ttyd available, offer to start it
if command -v ttyd &>/dev/null; then
    echo "ttyd detected! Start browser-based capture?"
    echo "  ttyd -p 7681 target/release/syslenz"
    echo ""
    read -p "Start ttyd? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Starting ttyd on http://localhost:7681 ..."
        echo "Take screenshots in your browser, then Ctrl+C to stop"
        ttyd -p 7681 target/release/syslenz
    fi
fi
