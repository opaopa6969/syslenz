# syslenz Provider Template

Write a new syslenz provider in 5 minutes.

## What is a provider?

A provider is an executable script (or binary) that collects metrics from some
source and outputs a single JSON object (`ProcEntry`) to stdout. syslenz
discovers providers automatically from `~/.config/syslenz/plugins/`.

## Quick start

1. Copy the template:

```bash
cp syslenz-provider-template ~/.config/syslenz/plugins/syslenz-provider-myapp
chmod +x ~/.config/syslenz/plugins/syslenz-provider-myapp
```

2. Edit the `collect_metrics()` function to gather your data.

3. Run syslenz -- your provider appears under `plugin/syslenz-provider-myapp`.

## ProcEntry JSON format

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

### FieldValue types

| Type       | JSON example              | When to use                    |
|------------|---------------------------|--------------------------------|
| `Integer`  | `{"Integer": 42}`         | Counters, counts, IDs          |
| `Float`    | `{"Float": 0.95}`         | Ratios, percentages, averages  |
| `Bytes`    | `{"Bytes": 1048576}`      | Memory, disk, network sizes    |
| `Text`     | `{"Text": "hello"}`       | Versions, names, states        |
| `Duration` | `{"Duration": 86400.0}`   | Uptime, elapsed time (seconds) |
| `Table`    | `{"Table": [["a","b"]]}`  | Tabular/multi-row data         |

## Helper functions

The template includes these helpers:

- `add_field "name" "Type" "value" "unit_or_null" "description"` -- queue a field
- `emit_entry "source description"` -- print the final JSON to stdout

## Rules

- Output exactly one JSON object to stdout (the ProcEntry).
- Exit 0 on success. Any non-zero exit code causes syslenz to skip the provider silently.
- Do not print anything else to stdout. Use stderr for debug logging if needed.
- The script must complete within 5 seconds or it will be killed.
- The filename becomes the plugin key: `plugin/<filename>`.

## Example: monitoring a custom app

```bash
collect_metrics() {
    local status
    status=$(curl -s http://localhost:8080/health | jq -r '.status')
    add_field "status" "Text" "$status" "null" "Application health status"

    local req_count
    req_count=$(curl -s http://localhost:8080/metrics | jq '.requests_total')
    add_field "requests_total" "Integer" "$req_count" "null" "Total HTTP requests served"

    local mem
    mem=$(curl -s http://localhost:8080/metrics | jq '.memory_bytes')
    add_field "memory_used" "Bytes" "$mem" "null" "Application memory usage"
}

emit_entry "myapp health endpoint"
```

## Testing

Run the script directly and pipe through `jq` to verify output:

```bash
./syslenz-provider-myapp | jq .
```

Verify it parses correctly:

```bash
./syslenz-provider-myapp | python3 -c "import sys,json; json.load(sys.stdin); print('OK')"
```
