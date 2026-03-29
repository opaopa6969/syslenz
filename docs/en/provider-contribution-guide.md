---
version: v1.3.0
lang: en
---

# Provider Contribution Guide

[🇯🇵 日本語版](../ja/provider-contribution-guide.md)

[<- Plugins](plugins.md) | [Index](index.md)


## Table of Contents

- [What is a Provider?](#what-is-a-provider)
- [Directory Structure](#directory-structure)
- [Required Files](#required-files)
- [ProcEntry Protocol](#procentry-protocol)
- [FieldValue Types](#fieldvalue-types)
- [How to Implement a Provider](#how-to-implement-a-provider)
  - [Step 1: Copy the Template](#step-1-copy-the-template)
  - [Step 2: Preflight Checks](#step-2-preflight-checks)
  - [Step 3: Collect Metrics](#step-3-collect-metrics)
  - [Step 4: Build Fields and Emit Output](#step-4-build-fields-and-emit-output)
  - [Step 5: Write the README](#step-5-write-the-readme)
- [Best Practices](#best-practices)
- [Testing](#testing)
- [New Provider Skeleton](#new-provider-skeleton)
- [Existing Providers as Examples](#existing-providers-as-examples)
- [Submitting Your Provider](#submitting-your-provider)


## What is a Provider?

A provider is an executable script (or binary) that collects metrics from an external service such as a database or middleware. Introduced in v1.3.0 as a higher-level concept built on top of the plugin system.

Providers follow the same ProcEntry JSON protocol as regular plugins, but differ in these ways:

- **Managed in the `providers/` directory** of the repository (installed to `~/.config/syslenz/plugins/` for use)
- **Connection settings via environment variables** are a standardized convention
- **README.md** documents monitored metrics, environment variables, installation, and testing
- **Naming convention**: files are named `syslenz-provider-<service-name>`

When a provider is copied to `~/.config/syslenz/plugins/`, syslenz auto-discovers it at startup and displays it in the sidebar as `plugin/syslenz-provider-<name>`.


## Directory Structure

Providers in the repository follow this layout:

```
providers/
├── template/                          # Template (base for new providers)
│   ├── syslenz-provider-template      # Executable script
│   └── README.md                      # Template usage guide
├── mysql/                             # MySQL provider
│   ├── syslenz-provider-mysql         # Executable script
│   └── README.md                      # Configuration & usage
├── postgres/                          # PostgreSQL provider
│   ├── syslenz-provider-postgres
│   └── README.md
├── redis/                             # Redis provider
│   ├── syslenz-provider-redis
│   └── README.md
└── nginx/                             # nginx provider
    ├── syslenz-provider-nginx
    └── README.md
```


## Required Files

Each provider directory must contain at minimum these 2 files:

### 1. `syslenz-provider-<name>` (Executable Script)

- Must have execute permission (`chmod +x`)
- Must output exactly one ProcEntry JSON object to stdout
- Must exit 0 on success, non-zero on failure
- Must complete within 5 seconds (killed after timeout)
- Must not print anything else to stdout (use stderr for debug logging)

### 2. `README.md`

Include the following sections:

- **Title**: `# syslenz <Service Name> Provider`
- **What it monitors**: Description of monitored metrics
- **Requirements**: Prerequisites (CLI tools, network access, etc.)
- **Configuration**: Environment variables table (variable, default, description)
- **Installation**: Install instructions with command examples
- **Testing**: How to verify the provider works


## ProcEntry Protocol

Providers output a JSON object conforming to this schema on stdout:

```json
{
  "source": "description of data source",
  "fields": [
    {
      "name": "field_name",
      "value": {"Integer": 42},
      "unit": null,
      "description": "what this field means"
    }
  ]
}
```

- `source`: String describing where the metrics come from (e.g., `"mysql SHOW GLOBAL STATUS"`, `"redis INFO"`)
- `fields`: Array of fields, each with `name`, `value`, `unit`, and `description`
- `unit`: Typically `null` (syslenz auto-formats based on the FieldValue type)


## FieldValue Types

The `value` field uses one of these tagged enum variants:

| Variant | JSON notation | Use case |
|---------|---------------|----------|
| `Integer` | `{"Integer": 42}` | Counters, connection counts, query counts |
| `Float` | `{"Float": 0.95}` | Ratios, percentages, hit rates |
| `Bytes` | `{"Bytes": 1048576}` | Memory, disk sizes (auto-formatted as KiB/MiB/GiB) |
| `Text` | `{"Text": "running"}` | Versions, statuses, names |
| `Duration` | `{"Duration": 86400.0}` | Uptime, elapsed time in seconds (auto-formatted as Xd Xh Xm Xs) |
| `Table` | `{"Table": [["a","b"],["c","d"]]}` | Tabular data |


## How to Implement a Provider

### Step 1: Copy the Template

```bash
# Run from the repository root
cp -r providers/template/ providers/my-service/
mv providers/my-service/syslenz-provider-template providers/my-service/syslenz-provider-my-service
chmod +x providers/my-service/syslenz-provider-my-service
```

### Step 2: Preflight Checks

At the top of your script, verify required CLI tools and parse connection settings:

```bash
# Check for required command
if ! command -v my-service-cli >/dev/null 2>&1; then
    echo "my-service-cli not found" >&2
    exit 1
fi

# Read connection settings from environment variables with defaults
MY_SERVICE_HOST="${MY_SERVICE_HOST:-localhost}"
MY_SERVICE_PORT="${MY_SERVICE_PORT:-9999}"
```

**Key points:**
- Exit with non-zero and a stderr message if a required CLI tool is missing
- Use the service name as an environment variable prefix (`MYSQL_`, `REDIS_`, `NGINX_`, etc.)
- Provide reasonable default values

### Step 3: Collect Metrics

Query the external service and parse the output:

```bash
# Fetch raw data from the service
RAW_OUTPUT=$(my-service-cli --host "$MY_SERVICE_HOST" --port "$MY_SERVICE_PORT" stats 2>/dev/null) || {
    echo "Failed to connect to my-service" >&2
    exit 1
}

# Parse values
active_connections=$(echo "$RAW_OUTPUT" | awk '/connections/ { print $2 }')
memory_used=$(echo "$RAW_OUTPUT" | awk '/memory/ { print $2 }')
```

**Key points:**
- Print an error to stderr and `exit 1` if the connection fails
- Redirect stderr of the query command to `/dev/null` to suppress noise
- Provide fallback values for fields that may fail to parse (`${value:-0}`)

### Step 4: Build Fields and Emit Output

Use the template's helper functions `add_field` and `emit_entry`:

```bash
add_field "active_connections" "Integer"  "${active_connections:-0}" "null" "Current active connections"
add_field "memory_used"        "Bytes"    "${memory_used:-0}"        "null" "Memory allocated by service"
add_field "uptime"             "Duration" "${uptime:-0}.0"           "null" "Service uptime"

emit_entry "my-service stats"
exit 0
```

**Key points:**
- Duration values must be floats (append `.0` if the source gives integers)
- The `emit_entry` argument describes the data source
- Always end with `exit 0`

### Step 5: Write the README

Create `providers/my-service/README.md` following this template:

```markdown
# syslenz My Service Provider

Monitors a My Service instance by querying `my-service-cli stats`.

## What it monitors

- **Connections**: active connections, idle connections
- **Memory**: allocated memory, peak memory
- **Performance**: queries per second, slow queries

## Requirements

- `my-service-cli` installed and on `$PATH`
- Network access to the My Service server

## Configuration

| Variable           | Default     | Description            |
|--------------------|-------------|------------------------|
| `MY_SERVICE_HOST`  | `localhost` | Server hostname        |
| `MY_SERVICE_PORT`  | `9999`      | Server port            |
| `MY_SERVICE_PASS`  | (none)      | Authentication password|

## Installation

\```bash
mkdir -p ~/.config/syslenz/plugins
cp syslenz-provider-my-service ~/.config/syslenz/plugins/
chmod +x ~/.config/syslenz/plugins/syslenz-provider-my-service

export MY_SERVICE_HOST=db.example.com
syslenz
\```

## Testing

\```bash
./syslenz-provider-my-service | jq .
\```
```


## Best Practices

1. **`set -euo pipefail`**: Place at the top of your script to catch errors early
2. **Mind the timeout**: The provider is killed after 5 seconds. Use `--max-time` or `--connect-timeout` on network requests
3. **Keep stdout clean**: Output only ProcEntry JSON to stdout. Direct debug logs to stderr
4. **Fallback values**: Use `${value:-0}` for fields that may fail to parse
5. **Computed metrics**: Add derived values like hit rates -- operators find these more useful than raw counters
6. **English descriptions**: Write the `description` field in English (syslenz's i18n system handles translation)
7. **Environment variable naming**: Use `<SERVICE_NAME>_<SETTING>` format consistently (e.g., `MYSQL_HOST`, `REDIS_PORT`)


## Testing

### Basic: Validate JSON Output

```bash
# Run the provider directly and format with jq
./syslenz-provider-my-service | jq .

# Verify it is valid JSON
./syslenz-provider-my-service | python3 -c "import sys,json; json.load(sys.stdin); print('OK')"

# Check exit code
./syslenz-provider-my-service; echo "Exit code: $?"
```

### Structural: Verify Required Fields

```bash
# Confirm the source field exists
./syslenz-provider-my-service | jq -e '.source'

# Confirm the fields array is non-empty
./syslenz-provider-my-service | jq -e '.fields | length > 0'

# Confirm every field has name, value, and description
./syslenz-provider-my-service | jq -e '.fields[] | select(.name and .value and .description) | .name'
```

### Integration: Verify in syslenz

```bash
# Copy the provider to the plugins directory
mkdir -p ~/.config/syslenz/plugins
cp syslenz-provider-my-service ~/.config/syslenz/plugins/
chmod +x ~/.config/syslenz/plugins/syslenz-provider-my-service

# Launch syslenz and confirm the provider appears in the sidebar
syslenz

# Select plugin/syslenz-provider-my-service in the sidebar and verify fields display correctly
```

### Error Cases

```bash
# Test behavior when the service is down
# (point to a non-existent host)
MY_SERVICE_HOST=nonexistent ./syslenz-provider-my-service
echo "Exit code: $?"
# Should return a non-zero exit code and print an error to stderr
```


## New Provider Skeleton

Below is a complete skeleton for starting a new provider. Copy it and customize the `collect_metrics` section:

```bash
#!/bin/bash
# syslenz provider: <SERVICE_NAME>
# Collects key metrics from <SERVICE_NAME> via <METHOD>.
#
# Configuration via environment variables:
#   <SERVICE>_HOST  (default: localhost)
#   <SERVICE>_PORT  (default: XXXX)
#   <SERVICE>_PASS  (optional)

set -euo pipefail

# ---------------------------------------------------------------------------
# JSON builder helpers (do not modify)
# ---------------------------------------------------------------------------
_SYSLENZ_FIELDS=""
_SYSLENZ_FIELD_COUNT=0

add_field() {
    local name="$1" type="$2" value="$3" unit="$4" desc="$5"
    local json_value
    case "$type" in
        Text)     json_value="{\"Text\": \"$value\"}" ;;
        Integer)  json_value="{\"Integer\": $value}" ;;
        Float)    json_value="{\"Float\": $value}" ;;
        Bytes)    json_value="{\"Bytes\": $value}" ;;
        Duration) json_value="{\"Duration\": $value}" ;;
        *)        json_value="{\"Text\": \"$value\"}" ;;
    esac
    local comma=""
    [ "$_SYSLENZ_FIELD_COUNT" -gt 0 ] && comma=","
    _SYSLENZ_FIELDS="${_SYSLENZ_FIELDS}${comma}{\"name\": \"${name}\", \"value\": ${json_value}, \"unit\": ${unit}, \"description\": \"${desc}\"}"
    _SYSLENZ_FIELD_COUNT=$((_SYSLENZ_FIELD_COUNT + 1))
}

emit_entry() {
    local source="$1"
    echo "{\"source\": \"${source}\", \"fields\": [${_SYSLENZ_FIELDS}]}"
}

# ---------------------------------------------------------------------------
# Preflight checks
# ---------------------------------------------------------------------------
if ! command -v <service-cli> >/dev/null 2>&1; then
    echo "<service-cli> not found" >&2
    exit 1
fi

SERVICE_HOST="${<SERVICE>_HOST:-localhost}"
SERVICE_PORT="${<SERVICE>_PORT:-XXXX}"

# Build CLI arguments
CLI_ARGS=(--host "$SERVICE_HOST" --port "$SERVICE_PORT")
if [ -n "${<SERVICE>_PASS:-}" ]; then
    CLI_ARGS+=(--password "${<SERVICE>_PASS}")
fi

# ---------------------------------------------------------------------------
# Collect raw data
# ---------------------------------------------------------------------------
RAW_OUTPUT=$(<service-cli> "${CLI_ARGS[@]}" stats 2>/dev/null) || {
    echo "Failed to connect to <SERVICE_NAME>" >&2
    exit 1
}

# Parse helper: extract a value by key from the raw output
get_value() {
    local key="$1"
    echo "$RAW_OUTPUT" | awk -v k="$key" '$1 == k { print $2; exit }'
}

# ---------------------------------------------------------------------------
# Extract metrics
# ---------------------------------------------------------------------------
# TODO: Replace with actual metric extraction
metric_a=$(get_value "metric_a")
metric_b=$(get_value "metric_b")
uptime=$(get_value "uptime")

# ---------------------------------------------------------------------------
# Build output
# ---------------------------------------------------------------------------
add_field "metric_a" "Integer"  "${metric_a:-0}"  "null" "Description of metric A"
add_field "metric_b" "Bytes"    "${metric_b:-0}"  "null" "Description of metric B"
add_field "uptime"   "Duration" "${uptime:-0}.0"  "null" "Service uptime"

emit_entry "<service-name> stats"
exit 0
```


## Existing Providers as Examples

| Provider | Approach | Worth studying for |
|----------|----------|--------------------|
| [MySQL](../../providers/mysql/) | Parses `SHOW GLOBAL STATUS` output | Status output parsing, buffer pool hit rate calculation |
| [Redis](../../providers/redis/) | Parses `redis-cli INFO` output | Multiple connection methods (host/port vs URL) |
| [nginx](../../providers/nginx/) | HTTP fetch of `stub_status` page | curl + awk text parsing, response validation |
| [PostgreSQL](../../providers/postgres/) | Runs queries via `psql` | SQL-based metric collection |
| [template](../../providers/template/) | Minimal boilerplate | Basic structure, helper function usage |


## Submitting Your Provider

To contribute a new provider to the syslenz project:

### 1. Fork and Create a Branch

```bash
git clone https://github.com/<your-username>/syslenz.git
cd syslenz
git checkout -b provider/<service-name>
```

### 2. Create the Provider

```bash
mkdir providers/<service-name>
# Follow the steps above to create the script and README
```

### 3. Pre-submission Checklist

Verify the following before submitting:

- [ ] Filename follows `syslenz-provider-<name>` format
- [ ] Execute permission is set (`chmod +x`)
- [ ] `set -euo pipefail` is at the top of the script
- [ ] Required CLI tools are checked (preflight check)
- [ ] Connection settings are configurable via environment variables
- [ ] Environment variables have reasonable defaults
- [ ] Connection failure produces a stderr message and non-zero exit
- [ ] Output is valid ProcEntry JSON
- [ ] Completes within 5 seconds
- [ ] README.md includes What it monitors / Requirements / Configuration / Installation / Testing
- [ ] Tested with `./syslenz-provider-<name> | jq .`
- [ ] Verified that metrics display correctly in syslenz

### 4. Create a Pull Request

```bash
git add providers/<service-name>/
git commit -m "feat: add <service-name> provider"
git push origin provider/<service-name>
```

Open a pull request on GitHub and include:

- What the provider monitors
- Which command/API it uses to collect metrics
- Test environment and results

---

[<- Plugins](plugins.md) | [Index](index.md)
