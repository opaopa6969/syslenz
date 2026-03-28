#!/bin/bash
# syslenz Web UI launcher
# Usage: ./run-web.sh [port] [lang]
#   ./run-web.sh          → port 3000, English
#   ./run-web.sh 8080     → port 8080, English
#   ./run-web.sh 3000 ja  → port 3000, Japanese

set -e

PORT="${1:-3000}"
LANG="${2:-en}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

cd "$SCRIPT_DIR"

# Build with web feature if needed
if [ ! -f target/release/syslenz ] || ! target/release/syslenz --help 2>&1 | grep -q web; then
    echo "Building syslenz with web feature..."
    source ~/.cargo/env 2>/dev/null || true
    cargo build --release --features web
fi

echo "Starting syslenz Web UI on http://localhost:${PORT}"
echo "Language: ${LANG}"
echo "Press Ctrl+C to stop"
echo ""

exec target/release/syslenz --web "$PORT" --lang "$LANG"
