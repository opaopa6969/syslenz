---
version: v1.1.0
lang: en
---

# Plugin System

[🇯🇵 日本語版](../ja/plugins.md)

[<- Prev: Web UI](web-ui.md) | [Index](index.md) | [Next: Configuration ->](config.md)


## Table of Contents

- [Overview](#overview)
- [Plugin Protocol](#plugin-protocol)
- [Plugin Directory](#plugin-directory)
- [Writing a Plugin Step by Step](#writing-a-plugin-step-by-step)
- [Example: JVM Memory Plugin](#example-jvm-memory-plugin)
- [Example: Docker Stats Plugin](#example-docker-stats-plugin)
- [Plugin Discovery and Execution](#plugin-discovery-and-execution)
- [Debugging Plugins](#debugging-plugins)

## Overview

syslenz supports custom data sources via executable plugins. A plugin is any executable file in the plugin directory that outputs a `ProcEntry` JSON object to stdout. Plugins appear in the sidebar prefixed with `plugin/` and integrate seamlessly with all views (Dashboard, Classic, Diagnostics, etc.).

## Plugin Protocol

A plugin must:

1. Be an executable file (any language: shell, Python, Rust, Go, etc.)
2. Accept no arguments (stdin is `/dev/null`)
3. Write a single JSON object to stdout conforming to the `ProcEntry` schema
4. Exit with status 0 on success

### ProcEntry JSON Schema

```json
{
  "source": "/custom/jvm-memory",
  "fields": [
    {
      "name": "HeapUsed",
      "value": { "Bytes": 536870912 },
      "unit": "bytes",
      "description": "Current JVM heap memory used"
    },
    {
      "name": "HeapMax",
      "value": { "Bytes": 1073741824 },
      "unit": "bytes",
      "description": "Maximum JVM heap size"
    },
    {
      "name": "GCCount",
      "value": { "Integer": 42 },
      "unit": null,
      "description": "Total garbage collection count"
    }
  ]
}
```

### FieldValue Types

The `value` field must be one of these tagged enum variants:

| Variant | JSON Format | Example |
|---------|------------|---------|
| Bytes | `{"Bytes": 1024}` | Memory, disk sizes |
| Integer | `{"Integer": 42}` | Counters, counts |
| Float | `{"Float": 3.14}` | Percentages, ratios |
| Text | `{"Text": "hello"}` | Strings, status values |
| Duration | `{"Duration": 86400.0}` | Time in seconds |
| Table | `{"Table": [["col1","col2"],["a","b"]]}` | Tabular data |

### Timeout

Plugins must complete within **5 seconds**. If a plugin exceeds this deadline, it is killed and skipped with an error message to stderr.

### Exit Codes

- Exit 0: Success. stdout is parsed as JSON.
- Non-zero exit: Plugin is skipped. An error is printed to stderr.

## Plugin Directory

Plugins are stored in:

```
~/.config/syslenz/plugins/
```

Or, if `$XDG_CONFIG_HOME` is set:

```
$XDG_CONFIG_HOME/syslenz/plugins/
```

syslenz creates this directory automatically if it does not exist.

**Important:** On Unix systems, plugin files must have the executable permission bit set (`chmod +x`). Non-executable files are silently skipped.

## Writing a Plugin Step by Step

### Step 1: Create the plugin file

```bash
mkdir -p ~/.config/syslenz/plugins
touch ~/.config/syslenz/plugins/my-plugin
chmod +x ~/.config/syslenz/plugins/my-plugin
```

### Step 2: Write the plugin logic

The simplest possible plugin (bash):

```bash
#!/bin/bash
cat <<'EOF'
{
  "source": "/custom/my-plugin",
  "fields": [
    {
      "name": "example_field",
      "value": {"Integer": 42},
      "unit": null,
      "description": "An example field"
    }
  ]
}
EOF
```

### Step 3: Test the plugin

```bash
~/.config/syslenz/plugins/my-plugin | python3 -m json.tool
```

Verify the output is valid JSON matching the ProcEntry schema.

### Step 4: Run syslenz

```bash
syslenz
```

The plugin will appear in the sidebar as `plugin/my-plugin`.

## Example: JVM Memory Plugin

A Python plugin that reads JVM metrics via `jcmd`:

```python
#!/usr/bin/env python3
"""syslenz plugin: JVM heap memory for the first running Java process."""
import json
import subprocess
import sys

def main():
    # Find first Java PID
    result = subprocess.run(['pgrep', '-f', 'java'], capture_output=True, text=True)
    if result.returncode != 0:
        print(json.dumps({
            "source": "/custom/jvm-memory",
            "fields": [
                {"name": "status", "value": {"Text": "No Java process found"},
                 "unit": None, "description": "Plugin status"}
            ]
        }))
        return

    pid = result.stdout.strip().split('\n')[0]

    # Get heap info via jcmd
    result = subprocess.run(
        ['jcmd', pid, 'GC.heap_info'],
        capture_output=True, text=True, timeout=3
    )

    fields = []
    for line in result.stdout.split('\n'):
        if 'used' in line.lower():
            # Parse "used 536870912" style output
            parts = line.split()
            for i, p in enumerate(parts):
                if p == 'used' and i + 1 < len(parts):
                    try:
                        used = int(parts[i + 1].rstrip(','))
                        fields.append({
                            "name": "HeapUsed",
                            "value": {"Bytes": used},
                            "unit": "bytes",
                            "description": "Current JVM heap memory used"
                        })
                    except ValueError:
                        pass

    if not fields:
        fields = [{"name": "status", "value": {"Text": "Could not parse heap info"},
                   "unit": None, "description": "Plugin status"}]

    print(json.dumps({"source": "/custom/jvm-memory", "fields": fields}))

if __name__ == '__main__':
    main()
```

Save as `~/.config/syslenz/plugins/jvm-memory` and `chmod +x`.

## Example: Docker Stats Plugin

A shell plugin that collects Docker container resource usage:

```bash
#!/bin/bash
# syslenz plugin: Docker container stats summary

[🇯🇵 日本語版](../ja/plugins.md)

if ! command -v docker &>/dev/null; then
    echo '{"source":"/custom/docker-stats","fields":[{"name":"status","value":{"Text":"docker not found"},"unit":null,"description":"Plugin status"}]}'
    exit 0
fi

# Get container stats (no-stream for single snapshot)

[🇯🇵 日本語版](../ja/plugins.md)
stats=$(docker stats --no-stream --format '{{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.NetIO}}' 2>/dev/null)

if [ -z "$stats" ]; then
    echo '{"source":"/custom/docker-stats","fields":[{"name":"status","value":{"Text":"No running containers"},"unit":null,"description":"Plugin status"}]}'
    exit 0
fi

# Build table rows

[🇯🇵 日本語版](../ja/plugins.md)
rows="["
first=true
while IFS=$'\t' read -r name cpu mem net; do
    if [ "$first" = true ]; then
        first=false
    else
        rows+=","
    fi
    rows+="[\"$name\",\"$cpu\",\"$mem\",\"$net\"]"
done <<< "$stats"
rows+="]"

count=$(echo "$stats" | wc -l)

cat <<EOF
{
  "source": "/custom/docker-stats",
  "fields": [
    {
      "name": "container_count",
      "value": {"Integer": $count},
      "unit": null,
      "description": "Number of running Docker containers"
    },
    {
      "name": "containers",
      "value": {"Table": $rows},
      "unit": null,
      "description": "Container resource usage (Name, CPU%, Memory, Net I/O)"
    }
  ]
}
EOF
```

Save as `~/.config/syslenz/plugins/docker-stats` and `chmod +x`.

## Plugin Discovery and Execution

When syslenz starts (and on each refresh), it:

1. Reads all files in `~/.config/syslenz/plugins/`
2. Skips non-files (directories, symlinks to directories)
3. Skips files without the executable permission bit (on Unix)
4. Executes each remaining file with:
   - stdin: `/dev/null`
   - stdout: piped (captured)
   - stderr: piped (captured, printed on failure)
   - Timeout: 5 seconds
5. Parses stdout as `ProcEntry` JSON
6. Inserts the entry as `plugin/<filename_without_extension>`

Plugins run in parallel with the main `/proc` parsing. Failed plugins are silently skipped with an error message printed to stderr.

## Debugging Plugins

### Test manually

```bash
# Run the plugin directly

[🇯🇵 日本語版](../ja/plugins.md)
~/.config/syslenz/plugins/my-plugin

# Validate JSON

[🇯🇵 日本語版](../ja/plugins.md)
~/.config/syslenz/plugins/my-plugin | python3 -m json.tool

# Check exit code

[🇯🇵 日本語版](../ja/plugins.md)
~/.config/syslenz/plugins/my-plugin; echo "Exit: $?"
```

### Check permissions

```bash
ls -la ~/.config/syslenz/plugins/
# Ensure +x is set on your plugin

[🇯🇵 日本語版](../ja/plugins.md)
```

### Check syslenz stderr

Run syslenz in a terminal and watch stderr for plugin errors:

```bash
syslenz 2>/tmp/syslenz-errors.log
# After quitting:

[🇯🇵 日本語版](../ja/plugins.md)
cat /tmp/syslenz-errors.log
```

Error messages look like:

```
[syslenz] plugin "my-plugin" skipped: exited with status 1
[syslenz] plugin "slow-plugin" skipped: plugin timed out after 5s
```

### Common issues

| Problem | Cause | Fix |
|---------|-------|-----|
| Plugin not appearing | File not executable | `chmod +x plugin-file` |
| Plugin not appearing | Not in plugins directory | Move to `~/.config/syslenz/plugins/` |
| "exited with status 1" | Plugin script has an error | Run plugin manually to see error |
| "plugin timed out" | Plugin takes > 5s | Optimize or cache results |
| "failed to parse" | Invalid JSON output | Validate JSON output manually |
| Empty fields in sidebar | JSON schema mismatch | Check field types match FieldValue variants |

---

[<- Prev: Web UI](web-ui.md) | [Index](index.md) | [Next: Configuration ->](config.md)
