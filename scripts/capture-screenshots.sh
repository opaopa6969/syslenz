#!/bin/bash
# Capture TUI screenshots for syslenz README.
#
# Methods:
#   1. vhs (recommended) -- uses docs/demo.tape with Screenshot commands
#   2. ttyd -- browser-based capture
#   3. Manual -- run syslenz, take screenshots with OS tools
#
# Output files:
#   docs/assets/dashboard.png    -- Dashboard view (default)
#   docs/assets/classic.png      -- Classic mode with source selected
#   docs/assets/detail.png       -- Detail view with fields
#   docs/assets/diff.png         -- Diff view showing changes
#   docs/assets/graph.png        -- Graph view with sparkline
#   docs/assets/diagnostics.png  -- Diagnostics view
#   docs/assets/category.png     -- Category Guide view
#   docs/assets/help-extra.png   -- Help panel at EXTRA level
#   docs/assets/welcome.png      -- Welcome screen

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ASSETS_DIR="$PROJECT_DIR/docs/assets"

cd "$PROJECT_DIR"

echo "Building syslenz..."
source ~/.cargo/env 2>/dev/null || true
cargo build --release 2>&1 | tail -1

mkdir -p "$ASSETS_DIR"

echo ""
echo "=== syslenz Screenshot Capture ==="
echo ""
echo "Target directory: $ASSETS_DIR"
echo ""

# Method 1: vhs
if command -v vhs &>/dev/null; then
    echo "vhs detected. Creating screenshot tape files..."

    for view in dashboard classic detail diff graph diagnostics category help-extra welcome; do
        TAPE_FILE=$(mktemp /tmp/syslenz-screenshot-XXXXXX.tape)
        cat > "$TAPE_FILE" <<TAPE
Output $ASSETS_DIR/${view}.png

Set Shell "bash"
Set FontSize 14
Set Width 1200
Set Height 700
Set Theme "Catppuccin Mocha"

Hide
Type "source ~/.cargo/env && cargo build --release 2>/dev/null"
Enter
Sleep 5s
Show

Type "target/release/syslenz"
Enter
Sleep 2s
TAPE

        case "$view" in
            dashboard)
                echo "Sleep 2s" >> "$TAPE_FILE"
                echo "Screenshot $ASSETS_DIR/dashboard.png" >> "$TAPE_FILE"
                ;;
            classic)
                cat >> "$TAPE_FILE" <<TAPE
Type "O"
Sleep 1.5s
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Screenshot $ASSETS_DIR/classic.png
TAPE
                ;;
            detail)
                cat >> "$TAPE_FILE" <<TAPE
Type "O"
Sleep 1s
Type "j"
Sleep 300ms
Type "j"
Sleep 300ms
Enter
Sleep 1.5s
Screenshot $ASSETS_DIR/detail.png
TAPE
                ;;
            diff)
                cat >> "$TAPE_FILE" <<TAPE
Type "O"
Sleep 1s
Type "d"
Sleep 2s
Screenshot $ASSETS_DIR/diff.png
TAPE
                ;;
            graph)
                cat >> "$TAPE_FILE" <<TAPE
Type "O"
Sleep 1s
Enter
Sleep 1s
Type "g"
Sleep 2s
Screenshot $ASSETS_DIR/graph.png
TAPE
                ;;
            diagnostics)
                cat >> "$TAPE_FILE" <<TAPE
Type "X"
Sleep 2s
Screenshot $ASSETS_DIR/diagnostics.png
TAPE
                ;;
            category)
                cat >> "$TAPE_FILE" <<TAPE
Type "C"
Sleep 2s
Screenshot $ASSETS_DIR/category.png
TAPE
                ;;
            help-extra)
                cat >> "$TAPE_FILE" <<TAPE
Type "O"
Sleep 1s
Enter
Sleep 1s
Type "?"
Sleep 500ms
Type "?"
Sleep 500ms
Type "?"
Sleep 1.5s
Screenshot $ASSETS_DIR/help-extra.png
TAPE
                ;;
            welcome)
                cat >> "$TAPE_FILE" <<TAPE
Type "W"
Sleep 2s
Screenshot $ASSETS_DIR/welcome.png
TAPE
                ;;
        esac

        cat >> "$TAPE_FILE" <<TAPE
Type "q"
Sleep 500ms
TAPE

        echo "  Capturing $view..."
        vhs "$TAPE_FILE" 2>/dev/null || echo "    WARNING: vhs capture failed for $view"
        rm -f "$TAPE_FILE"
    done

    echo ""
    echo "Screenshots saved to $ASSETS_DIR/"
    exit 0
fi

# Method 2: ttyd
if command -v ttyd &>/dev/null; then
    echo "vhs not found, but ttyd detected."
    echo ""
    echo "Steps:"
    echo "  1. Run: ttyd -p 7681 target/release/syslenz"
    echo "  2. Open http://localhost:7681 in your browser"
    echo "  3. Navigate to each view and take browser screenshots"
    echo "  4. Save them to $ASSETS_DIR/ with the names listed above"
    echo ""
    read -p "Start ttyd? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Starting ttyd on http://localhost:7681 ..."
        ttyd -p 7681 target/release/syslenz
    fi
    exit 0
fi

# Method 3: Manual
echo "Neither vhs nor ttyd found."
echo ""
echo "Install vhs (recommended):"
echo "  go install github.com/charmbracelet/vhs@latest"
echo ""
echo "Or take manual screenshots:"
echo "  1. Run: target/release/syslenz"
echo "  2. Navigate to each view"
echo "  3. Take screenshots with your OS tools"
echo "  4. Save to $ASSETS_DIR/ with the names above"
