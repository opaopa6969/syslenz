#!/bin/bash
# Record a demo GIF for syslenz README.
#
# Prerequisites:
#   Option A (recommended): charmbracelet/vhs
#     go install github.com/charmbracelet/vhs@latest
#     vhs docs/demo.tape
#
#   Option B: asciinema + agg
#     pip install asciinema
#     cargo install agg
#
# This script uses Option B (asciinema + tmux).
# For Option A, just run: vhs docs/demo.tape

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ASSETS_DIR="$PROJECT_DIR/docs/assets"
CAST_FILE="$ASSETS_DIR/demo.cast"
GIF_FILE="$ASSETS_DIR/demo.gif"
SESSION="syslenz-demo"

cd "$PROJECT_DIR"

# Build
echo "Building syslenz..."
source ~/.cargo/env 2>/dev/null || true
cargo build --release 2>&1 | tail -1

mkdir -p "$ASSETS_DIR"

# Check tools
if ! command -v asciinema &>/dev/null; then
    echo "Error: asciinema not found. Install with: pip install asciinema"
    echo ""
    echo "Alternative: use vhs instead:"
    echo "  go install github.com/charmbracelet/vhs@latest"
    echo "  vhs docs/demo.tape"
    exit 1
fi

if ! command -v tmux &>/dev/null; then
    echo "Error: tmux not found. Install with: apt install tmux"
    exit 1
fi

# Kill any existing session
tmux kill-session -t "$SESSION" 2>/dev/null || true

echo "Recording demo..."
tmux new-session -d -s "$SESSION" -x 120 -y 35

# Start asciinema recording inside tmux
tmux send-keys -t "$SESSION" \
    "asciinema rec --cols 120 --rows 35 --overwrite '$CAST_FILE' -c '$PROJECT_DIR/target/release/syslenz'" Enter
sleep 3

# Dashboard (default view)
sleep 2

# Switch to Classic mode
tmux send-keys -t "$SESSION" "O"
sleep 2

# Navigate sources
for i in 1 2 3 4 5; do
    tmux send-keys -t "$SESSION" "j"
    sleep 0.4
done

# Drill into detail
tmux send-keys -t "$SESSION" Enter
sleep 1.5

# Scroll fields
for i in 1 2 3; do
    tmux send-keys -t "$SESSION" "j"
    sleep 0.3
done

# Help: Normal -> Detailed -> Extra -> Off
tmux send-keys -t "$SESSION" "?"
sleep 1.5
tmux send-keys -t "$SESSION" "?"
sleep 1.5
tmux send-keys -t "$SESSION" "?"
sleep 2.5
tmux send-keys -t "$SESSION" "?"
sleep 0.5

# Back to sidebar
tmux send-keys -t "$SESSION" "h"
sleep 0.5

# Diff view
tmux send-keys -t "$SESSION" "d"
sleep 2

# Diagnostics
tmux send-keys -t "$SESSION" "X"
sleep 2.5

# Category Guide
tmux send-keys -t "$SESSION" "C"
sleep 2
tmux send-keys -t "$SESSION" "j"
sleep 0.5
tmux send-keys -t "$SESSION" "j"
sleep 0.5

# Switch to Japanese
tmux send-keys -t "$SESSION" "L"
sleep 2

# Dashboard
tmux send-keys -t "$SESSION" "D"
sleep 2

# Quit
tmux send-keys -t "$SESSION" "q"
sleep 2

# Clean up
tmux kill-session -t "$SESSION" 2>/dev/null || true

echo ""
echo "Recording saved to: $CAST_FILE"
echo ""
echo "To convert to GIF:"
echo "  agg --cols 120 --rows 35 --theme monokai '$CAST_FILE' '$GIF_FILE'"
echo ""
echo "To play:"
echo "  asciinema play '$CAST_FILE'"
echo ""
echo "Alternative (vhs):"
echo "  vhs docs/demo.tape"
