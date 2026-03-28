# syslenz-jvm Plugin

A comprehensive JVM monitoring plugin for [syslenz](https://github.com/opaopa6969/syslenz).

## What It Monitors

For **every running Java process** on the system, the plugin collects:

| Category | Metrics | Tool |
|----------|---------|------|
| Heap Memory | used, max, committed (bytes) | `jstat -gc`, `jstat -gccapacity` |
| Garbage Collection | young GC count/time, full GC count/time, total GC time | `jstat -gc` |
| Threads | thread count | `jcmd Thread.print` (fallback: `/proc/<pid>/task`) |
| Class Loading | loaded class count | `jstat -class` |
| CPU | process CPU % | `/proc/<pid>/stat` (two-sample delta) |
| Uptime | JVM uptime in seconds | `jcmd VM.uptime` |

When multiple JVMs are running, summary totals are included alongside a table
with one row per JVM.

## Requirements

- **JDK tools** on `$PATH`: `jstat`, `jcmd` (included in any standard JDK installation)
- **Linux** (uses `/proc` for CPU calculation)
- Permission to attach to the target JVM processes (same user, or root)

Verify the tools are available:

```bash
which jstat jcmd
```

If you only have a JRE (not a JDK), these tools will be missing. Install a full
JDK package, e.g.:

```bash
sudo apt install openjdk-17-jdk    # Debian/Ubuntu
sudo dnf install java-17-openjdk-devel  # Fedora/RHEL
```

## Installation

```bash
cp plugins/jvm/syslenz-jvm ~/.config/syslenz/plugins/
# The file must be executable:
chmod +x ~/.config/syslenz/plugins/syslenz-jvm
```

Restart syslenz and the plugin will appear as **plugin/syslenz-jvm**.

## Output Fields

### Summary Fields

| Field | Type | Description |
|-------|------|-------------|
| `jvm_count` | Integer | Number of running JVM processes |
| `total_heap_used` | Bytes | Sum of heap used across all JVMs |
| `total_young_gc` | Integer | Sum of young GC counts |
| `total_full_gc` | Integer | Sum of full GC counts |
| `total_gc_time` | Duration | Sum of GC time (seconds) |
| `jvm_table` | Table | Per-JVM overview (PID, heap, GC, threads, classes, CPU, uptime) |

### Per-JVM Fields

Each JVM also gets individual fields prefixed with `jvm_<pid>_`:

| Field | Type | Description |
|-------|------|-------------|
| `jvm_<pid>_heap_used` | Bytes | Current heap memory in use |
| `jvm_<pid>_heap_max` | Bytes | Maximum heap the JVM can grow to (`-Xmx`) |
| `jvm_<pid>_heap_committed` | Bytes | Heap memory committed by the OS |
| `jvm_<pid>_gc_young_count` | Integer | Number of young generation GC events |
| `jvm_<pid>_gc_young_time` | Duration | Cumulative time spent in young GC |
| `jvm_<pid>_gc_full_count` | Integer | Number of full GC events |
| `jvm_<pid>_gc_full_time` | Duration | Cumulative time spent in full GC |
| `jvm_<pid>_threads` | Integer | Number of live threads |
| `jvm_<pid>_classes_loaded` | Integer | Number of loaded classes |
| `jvm_<pid>_cpu_percent` | Float | Process CPU utilization (%) |
| `jvm_<pid>_uptime` | Duration | JVM uptime (seconds) |

## Understanding the Metrics

### Heap Memory

The JVM heap is divided into generations:

- **Young Generation** (Eden + Survivor spaces): short-lived objects are allocated
  here. Most objects die young, so the young generation is collected frequently
  with fast "minor" (young) GCs.
- **Old Generation**: objects that survive multiple young GCs are promoted here.
  Collected less frequently with "major" (full) GCs that pause the application
  longer.

**heap_used** is the sum of Eden + Survivor + Old usage. **heap_max** is the
upper bound set by `-Xmx`. **heap_committed** is the amount of OS memory
currently reserved.

If `heap_used` is consistently close to `heap_max`, the application may be
under memory pressure and experiencing frequent full GCs.

### Garbage Collection

- **Young GC (YGC)**: fast, typically < 50ms. High frequency is normal.
- **Full GC (FGC)**: expensive, can cause noticeable pauses. Ideally close to
  zero in a healthy application.
- **GC Time**: cumulative wall-clock time spent in GC. If this grows rapidly
  relative to uptime, the JVM is spending too much time collecting garbage.

A rule of thumb: if GC time exceeds 5% of uptime, investigate heap sizing or
GC tuning (`-XX:+UseG1GC`, `-XX:MaxGCPauseMillis`, etc.).

### Threads

Thread count includes all live threads (application threads, GC threads, JIT
compiler threads, etc.). A steadily increasing thread count may indicate a
thread leak (threads being created but never terminated).

### Class Loading

The number of loaded classes grows as the application discovers and loads new
code. In steady state, this should plateau. Continuously increasing class
counts may signal a classloader leak (common in application servers with
hot-deploy).

### CPU Usage

Process CPU is computed by sampling `/proc/<pid>/stat` twice with a 200ms gap.
The value is a percentage of total CPU capacity (0-100% per core, so a 4-core
system could reach 400%).

## Graceful Failure

- If no Java processes are found, the plugin outputs a valid ProcEntry with a
  status message and exits 0 (syslenz shows it cleanly).
- If individual JDK tools fail for a specific PID (permissions, process exited),
  zeros are reported for that JVM and collection continues for others.
