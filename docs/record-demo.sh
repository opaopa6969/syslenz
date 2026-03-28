#!/bin/bash
# Automated demo recording using tmux + asciinema
# Outputs: docs/assets/demo.cast (asciinema recording)
# Convert to GIF: agg docs/assets/demo.cast docs/assets/demo.gif

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
ASSETS_DIR="$SCRIPT_DIR/assets"
CAST_FILE="$ASSETS_DIR/demo.cast"
SESSION="syslenz-demo"

cd "$PROJECT_DIR"

# Build
echo "Building syslenz..."
source ~/.cargo/env 2>/dev/null || true
cargo build --release 2>&1 | tail -1

mkdir -p "$ASSETS_DIR"

# Kill any existing session
tmux kill-session -t "$SESSION" 2>/dev/null || true

# Start tmux session with asciinema recording
echo "Recording demo..."
tmux new-session -d -s "$SESSION" -x 120 -y 35

# Start asciinema recording inside tmux
tmux send-keys -t "$SESSION" "asciinema rec --cols 120 --rows 35 --overwrite '$CAST_FILE' -c '$PROJECT_DIR/target/release/syslenz'" Enter
sleep 3

# --- Dashboard (default view) ---
sleep 2

# --- Switch to Classic mode ---
tmux send-keys -t "$SESSION" "O"
sleep 2

# --- Navigate sources ---
tmux send-keys -t "$SESSION" "j"
sleep 0.4
tmux send-keys -t "$SESSION" "j"
sleep 0.4
tmux send-keys -t "$SESSION" "j"
sleep 0.4
tmux send-keys -t "$SESSION" "j"
sleep 0.4
tmux send-keys -t "$SESSION" "j"
sleep 0.4

# --- Drill into detail ---
tmux send-keys -t "$SESSION" Enter
sleep 1.5

# --- Scroll fields ---
tmux send-keys -t "$SESSION" "j"
sleep 0.3
tmux send-keys -t "$SESSION" "j"
sleep 0.3
tmux send-keys -t "$SESSION" "j"
sleep 0.3

# --- Help Normal ---
tmux send-keys -t "$SESSION" "?"
sleep 1.5

# --- Help Detailed ---
tmux send-keys -t "$SESSION" "?"
sleep 1.5

# --- Help Extra ---
tmux send-keys -t "$SESSION" "?"
sleep 2.5

# --- Help Off ---
tmux send-keys -t "$SESSION" "?"
sleep 0.5

# --- Back to sidebar ---
tmux send-keys -t "$SESSION" "h"
sleep 0.5

# --- Diff view ---
tmux send-keys -t "$SESSION" "d"
sleep 2

# --- Diagnostics ---
tmux send-keys -t "$SESSION" "X"
sleep 2.5

# --- Category Guide ---
tmux send-keys -t "$SESSION" "C"
sleep 2
tmux send-keys -t "$SESSION" "j"
sleep 0.5
tmux send-keys -t "$SESSION" "j"
sleep 0.5
tmux send-keys -t "$SESSION" "j"
sleep 0.5

# --- Switch to Japanese ---
tmux send-keys -t "$SESSION" "L"
sleep 2

# --- Dashboard ---
tmux send-keys -t "$SESSION" "D"
sleep 2

# --- Quit ---
tmux send-keys -t "$SESSION" "q"
sleep 1

# Wait for asciinema to finish
sleep 2

# Clean up tmux
tmux kill-session -t "$SESSION" 2>/dev/null || true

echo ""
echo "Recording saved to: $CAST_FILE"
echo ""
echo "To convert to GIF:"
echo "  pip install agg  # or: cargo install agg"
echo "  agg --cols 120 --rows 35 --theme monokai '$CAST_FILE' '$ASSETS_DIR/demo.gif'"
echo ""
echo "To play recording:"
echo "  asciinema play '$CAST_FILE'"
echo ""
echo "To upload to asciinema.org:"
echo "  asciinema upload '$CAST_FILE'"
