# 🚀 FileStat

---

## 📖 Overview

**FileStat** is a high-performance filesystem statistics, duplicate detection, and file integrity monitoring tool written in **Rust**.

Originally designed as a filesystem analysis utility, FileStat has evolved into a security-focused monitoring platform inspired by modern **File Integrity Monitoring (FIM)** solutions.

Whether you're analyzing storage usage, identifying duplicate files, benchmarking filesystem performance, or monitoring critical directories for unauthorized changes, FileStat provides a fast and reliable solution.

---

## ✨ Key Highlights

✅ Lightning-fast recursive scanning
✅ Parallel duplicate detection using Rayon
✅ BLAKE3 cryptographic hashing
✅ Real-time filesystem monitoring
✅ Persistent integrity baselines
✅ JSON & CSV reporting
✅ Security-focused architecture
✅ Built entirely in Rust

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
| --------------------- | ------ |
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
| Alert System          | 🚧     |
| Critical Path Rules   | ⏳      |
| Snapshot Engine       | ⏳      |
| SQLite Storage        | ⏳      |
| Agent Mode            | ⏳      |

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
filestat watch .
```

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

Monitor a directory for integrity changes:

```bash
filestat watch ~/Projects
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

### 🚧 Next Stage

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
