#!/bin/sh
# syslenz Web UI + MCP server launcher
# Usage: run.sh (PORT env var sets the port, default 3009)
exec /home/opa/syslenz/target/release/syslenz --web ${PORT:-3009} --lang en
