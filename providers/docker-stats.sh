#!/bin/bash
# syslenz plugin: Docker container stats
# Place in ~/.config/syslenz/plugins/docker-stats (without .sh extension, chmod +x)

set -euo pipefail

# Check if docker is available
if ! command -v docker &>/dev/null; then
    exit 1
fi

# Get stats for all running containers (non-streaming)
STATS=$(docker stats --no-stream --format '{{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}\t{{.BlockIO}}\t{{.PIDs}}' 2>/dev/null || true)

if [ -z "$STATS" ]; then
    exit 1
fi

# Build table rows: header + data
HEADER='["Name","CPU%","MemUsage","NetIO","BlockIO","PIDs"]'
ROWS="$HEADER"

while IFS=$'\t' read -r name cpu mem net block pids; do
    ROW=$(printf '["%s","%s","%s","%s","%s","%s"]' \
        "$name" "$cpu" "$mem" "$net" "$block" "$pids")
    ROWS="$ROWS,$ROW"
done <<< "$STATS"

# Count containers
COUNT=$(echo "$STATS" | wc -l)

cat <<EOF
{
  "source": "docker stats",
  "fields": [
    {"name": "ContainerCount", "value": {"Integer": $COUNT}, "unit": "containers", "description": "Number of running Docker containers"},
    {"name": "Stats", "value": {"Table": [$ROWS]}, "unit": null, "description": "Per-container resource usage"}
  ]
}
EOF
