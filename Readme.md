# 🚀 FileStat

---

## 📖 Overview

**FileStat** is a high-performance filesystem statistics, duplicate detection, and file integrity monitoring tool written in **Rust**.

Originally designed as a filesystem analysis utility, FileStat has evolved into a security-focused monitoring platform inspired by modern **File Integrity Monitoring (FIM)** solutions.

Whether you're analyzing storage usage, identifying duplicate files, benchmarking filesystem performance, or monitoring critical directories for unauthorized changes, FileStat provides a fast and reliable solution.

---

## ✨ Key Highlights

- Recursive filesystem scanning
- Extension, size, and ignore-based filtering
- JSON and CSV export support
- BLAKE3-based duplicate detection
- Parallel hashing with Rayon
- Real-time filesystem monitoring
- Persistent integrity baselines
- File Integrity Monitoring (FIM) foundation

---

## 🛠 Features

### 📊 Statistics Engine

* Recursive directory scanning
* Total file count
* Total directory count
* Total storage usage
* Average file size
* Largest file detection
* Extension statistics

### 🎯 Filtering System

* Hidden file filtering
* Extension filtering
* Multiple extension support
* Minimum size filtering
* Maximum size filtering
* Ignore pattern support

### 📤 Exporting

* JSON export
* CSV export

### 🔍 Duplicate Detection

* BLAKE3 content hashing
* Duplicate grouping
* Parallel hashing using Rayon

### ⚡ Performance Metrics

* Scan duration
* Files per second
* Duplicate hashing benchmarks

### 🛡️ File Integrity Monitoring (FIM)

* Real-time filesystem monitoring
* Recursive directory watching
* Baseline hash engine
* Integrity verification
* Persistent baseline storage
* Metadata tracking

---

## 📌 Feature Status

| Feature               | Status |
| --------------------- |--------|
| Recursive Scanning    | ✅      |
| Statistics Engine     | ✅      |
| Largest Files         | ✅      |
| Extension Statistics  | ✅      |
| Hidden File Filtering | ✅      |
| Ignore Rules          | ✅      |
| JSON Export           | ✅      |
| CSV Export            | ✅      |
| Extension Filters     | ✅      |
| Size Filters          | ✅      |
| Duplicate Detection   | ✅      |
| Parallel Hashing      | ✅      |
| Performance Metrics   | ✅      |
| Real-Time Monitoring  | ✅      |
| Baseline Hash Engine  | ✅      |
| Persistent Baseline   | ✅      |
| Metadata Integrity    | ✅      |
| Alert System          | ✅      |
| Critical Path Rules   | ✅      |
| Snapshot Engine       | ✅      |
| Config File Support   | ✅      |
| SQLite Storage        | ✅      |
| Agent Mode            | 🚧     |

---

## 📦 Installation

Clone the repository:

```bash
git clone https://github.com/rahulxsh/filestat.git
cd filestat
```

Build the project:

```bash
cargo build --release
```

Run:

```bash
cargo run -- scan .
```

---

## 🚀 Usage

### Basic Scan

```bash
filestat scan .
```

### Largest Files

```bash
filestat scan . --largest-files
```

```bash
filestat scan . --largest-files --top 20
```

### Extension Statistics

```bash
filestat scan . --print-extension
```

### Hidden Files

```bash
filestat scan . --hidden
```

### File Size Statistics

```bash
filestat scan . --size
```

### Total Statistics

```bash
filestat scan . --total
```

### JSON Export

Export using default output file:

```bash
filestat scan . --json
```

Export to custom path:

```bash
filestat scan . --json report.json
```

### CSV Export

```bash
filestat scan . --csv
```

### Extension Filtering

Single extension:

```bash
filestat scan . --ext rs
```

Multiple extensions:

```bash
filestat scan . --ext rs js ts
```

### Minimum Size Filter

```bash
filestat scan . --min-size 10MB
```

### Maximum Size Filter

```bash
filestat scan . --max-size 100MB
```

### Ignore Patterns

```bash
filestat scan . --ignore target .git node_modules
```

### Duplicate Detection

```bash
filestat scan . --duplicate
```

### File Integrity Monitoring

```bash
filestat watch --config ./config.toml
```

### Configuration File

FileStat supports configuration-driven monitoring and snapshot management.

Example `config.toml`:

```toml
monitor_paths = [
    "/Users/rahulsharma/.ssh",
    "/Users/rahulsharma/.config",
    "/Users/rahulsharma/Projects/filestat"
]

critical_paths = [
    "/Users/rahulsharma/.ssh"
]

snapshot_paths = [
    "/Users/rahulsharma/.ssh"
]

ignore = [
    "node_modules",
    ".git",
    "target",
    "src"
]
```

#### Configuration Options

| Option | Description |
|----------|-------------|
| `monitor_paths` | Directories monitored by the File Integrity Monitoring (FIM) engine. |
| `critical_paths` | Sensitive files or directories that generate elevated severity alerts when modified. |
| `snapshot_paths` | Directories included when creating filesystem snapshots. |
| `ignore` | Directory names ignored during scanning, monitoring, baseline creation, and snapshot generation. |

#### Start Monitoring Using Configuration

```bash
filestat watch --config config.toml
```

#### Create Snapshot

```bash
filestat snapshot save --config config.toml
```

#### Compare Snapshot

```bash
filestat snapshot diff --config config.toml
```

#### Get Recent Alerts
Limit flag --limit is optional default it will be 20
```bash
filestat alerts --limit 30
```

#### Ignored Directories

Ignored entries are matched by directory name and skipped recursively.

Examples:

```toml
ignore = [
    "target",
    "node_modules",
    ".git",
    ".filestat"
]
```

This prevents temporary build artifacts, package caches, repository metadata, and FileStat internal files from being included in:

- Baseline creation
- Real-time monitoring
- Snapshot generation
- Integrity verification
---

## 💡 Example Commands

Analyze Rust source files larger than 1 KB:

```bash
filestat scan . --ext rs --min-size 1KB
```

Find duplicate files:

```bash
filestat scan . --duplicate
```

Snapshot diff to show changed file path

```bash
filestat snapshot --show-paths diff
```

---

## ⚡ Performance

Current benchmark:

```text
Files Scanned: 20,512
Scan Duration: 0.43s
Files/sec: ~47,171

Duplicate Duration: 1.03s
Hashes/sec: ~4,616
```

### Benchmark Snapshot

```text
┌──────────────────────────────┐
│      FILESTAT PERFORMANCE    │
├──────────────────────────────┤
│ Files Scanned   : 20,512     │
│ Scan Time       : 0.43 sec   │
│ Throughput      : 47k+/sec   │
│ Hash Algorithm  : BLAKE3     │
│ Parallel Engine : Rayon      │
└──────────────────────────────┘
```

---

## 🛡️ File Integrity Monitoring Roadmap

### ✅ Completed

* Recursive filesystem monitoring
* Event detection
* Baseline hash creation
* Integrity verification
* Persistent baseline storage
* Metadata tracking


#### Alert System

Planned alert types:

* FileCreated
* FileDeleted
* DirectoryCreated
* DirectoryDeleted
* HashChanged
* PermissionChanged
* OwnershipChanged

Planned alert metadata:

* Timestamp
* Severity
* Path
* Old/New Hash
* Old/New Size

---

## 🗺 Future Roadmap

### Stage 6 — Alert Intelligence

* Structured alert engine
* Severity classification

### Stage 7 — Security Rules

* Critical path rules
* Sensitive file monitoring

### Stage 8 — Snapshot Engine

* Filesystem state snapshots
* Historical comparison

### 🚧 Next Stage

### Stage 9 — Persistence Layer

* SQLite event storage
* Historical search
* Audit trail generation

### Stage 10 — Enterprise Features

* Rule engine
* Security policies
* Wazuh-inspired architecture
* Background agent mode

---

## 🌟 Vision

FileStat is steadily evolving from a filesystem analytics utility into a complete **security-focused File Integrity Monitoring platform** capable of detecting unauthorized changes, tracking filesystem activity, and providing actionable security insights.

---

## 📄 License

MIT License

---
