#!/bin/bash
# syslenz plugin: JVM metrics via jstat
# Place in ~/.config/syslenz/plugins/jvm (without .sh extension, chmod +x)

set -euo pipefail

# Find first Java process
PID=$(pgrep -o java 2>/dev/null || true)
if [ -z "$PID" ]; then
    exit 1
fi

# Run jstat -gc to get GC statistics
JSTAT_OUTPUT=$(jstat -gc "$PID" 2>/dev/null || true)
if [ -z "$JSTAT_OUTPUT" ]; then
    exit 1
fi

# Parse jstat output (second line has values)
VALUES=$(echo "$JSTAT_OUTPUT" | tail -1 | tr -s ' ')

# jstat -gc columns:
# S0C S1C S0U S1U EC EU OC OU MC MU CCSC CCSU YGC YGCT FGC FGCT CGC CGCT GCT
S0U=$(echo "$VALUES" | awk '{print $3}')
S1U=$(echo "$VALUES" | awk '{print $4}')
EU=$(echo "$VALUES" | awk '{print $6}')
OU=$(echo "$VALUES" | awk '{print $8}')
MU=$(echo "$VALUES" | awk '{print $10}')
YGC=$(echo "$VALUES" | awk '{print $13}')
YGCT=$(echo "$VALUES" | awk '{print $14}')
FGC=$(echo "$VALUES" | awk '{print $15}')
FGCT=$(echo "$VALUES" | awk '{print $16}')
GCT=$(echo "$VALUES" | awk '{print $NF}')

# Convert KB to bytes (jstat reports in KB)
S0U_BYTES=$(awk "BEGIN {printf \"%.0f\", $S0U * 1024}")
S1U_BYTES=$(awk "BEGIN {printf \"%.0f\", $S1U * 1024}")
EU_BYTES=$(awk "BEGIN {printf \"%.0f\", $EU * 1024}")
OU_BYTES=$(awk "BEGIN {printf \"%.0f\", $OU * 1024}")
MU_BYTES=$(awk "BEGIN {printf \"%.0f\", $MU * 1024}")

cat <<EOF
{
  "source": "jstat -gc (PID $PID)",
  "fields": [
    {"name": "Survivor0Used", "value": {"Bytes": $S0U_BYTES}, "unit": null, "description": "Survivor space 0 utilization"},
    {"name": "Survivor1Used", "value": {"Bytes": $S1U_BYTES}, "unit": null, "description": "Survivor space 1 utilization"},
    {"name": "EdenUsed", "value": {"Bytes": $EU_BYTES}, "unit": null, "description": "Eden space utilization"},
    {"name": "OldUsed", "value": {"Bytes": $OU_BYTES}, "unit": null, "description": "Old generation utilization"},
    {"name": "MetaspaceUsed", "value": {"Bytes": $MU_BYTES}, "unit": null, "description": "Metaspace utilization"},
    {"name": "YoungGC", "value": {"Integer": $YGC}, "unit": "count", "description": "Number of young generation GC events"},
    {"name": "YoungGCTime", "value": {"Duration": $YGCT}, "unit": "s", "description": "Young generation GC time"},
    {"name": "FullGC", "value": {"Integer": $FGC}, "unit": "count", "description": "Number of full GC events"},
    {"name": "FullGCTime", "value": {"Duration": $FGCT}, "unit": "s", "description": "Full GC time"},
    {"name": "TotalGCTime", "value": {"Duration": $GCT}, "unit": "s", "description": "Total garbage collection time"}
  ]
}
EOF
