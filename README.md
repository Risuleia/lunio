# Lunio

Lunio is a high-performance, daemon-driven desktop file explorer built with Rust, Tauri, and React.  
It is engineered for responsiveness, large directory handling, media-rich environments, and strict separation between UI and filesystem operations.

The backend owns all state, indexing, metadata extraction, and thumbnail generation, while the frontend is a thin, reactive renderer connected through an asynchronous, versioned IPC protocol.

---

## Overview

Lunio replaces traditional, UI-bound filesystem calls with a persistent Rust daemon responsible for:

- Directory scanning and metadata extraction  
- Incremental indexing and fast lookups  
- Thumbnail generation for images, PDFs, and videos  
- Background rescanning  
- File opening and system operations  
- Runtime toolchain bootstrapping for FFmpeg and PDFium  

The frontend never accesses the filesystem directly.  
All interactions occur through a typed, binary-safe TCP protocol implemented by the `lunio_client` library.

---

## Architecture

             ┌──────────────────────────┐
             │        Frontend UI       │
             │    (Tauri + React TS)    │
             └──────────────┬───────────┘
                            │
               lunio_client │  (async TCP)
                            ▼
             ┌──────────────────────────┐
             │          Daemon          │
             │           (Rust)         │
             ├─────────────┬────────────┤
    Directory│             │ Thumbnail  │
     Queries │   Indexing  │   Worker   │ Thumbnail
             ▼   Engine    │   Thread   │ Jobs
             │ (Arc State) │            ▼
             └─────────────┼────────────┘
                           │
                           ▼
             ┌──────────────────────────┐
             │  Runtime Dependencies    │
             │  FFmpeg / PDFium Loader  │
             └──────────────────────────┘


---

## System Design

### Components

**1. Daemon (Rust)**
- Single authoritative store for file metadata  
- Manages a shared `EngineRuntime` behind `Arc`  
- Schedules and processes thumbnail tasks  
- Hosts structured, versioned IPC protocol  
- Handles runtime dependency installation  

**2. IPC Layer (`lunio_client`)**
- Asynchronous TCP communication  
- Length-prefixed, JSON-encoded messages  
- Typed request/response enums  
- Handshake with version negotiation  
- Cancelable and interruptible operations  

**3. Frontend (Tauri + React)**
- Grid, List, and Masonry views  
- Real-time thumbnail streaming  
- Virtualized rendering for large folders  
- Advanced multi-selection and lasso tools  
- Debounced and cancelable navigation flows  

---

## Detailed Diagrams

### 1. Request Lifecycle

            UI Action
                │
                ▼
          lunio_client → DirectoryListRequest → Daemon
                │
                │ (previous listing cancelled)
                ▼
          EngineRuntime → Directory scan → Metadata model
                │
                ▼
        Thumbnail jobs queued → Worker thread
                │
                ▼
        Daemon streams results → UI renders incrementally

---

### 2. Thumbnail Pipeline

            File Entry
                │
                ├── Cache Hit → Return immediately
                │
                └── Cache Miss
                │
                ▼
            Thumbnail Job Queue
                │
                ▼
          Worker Thread → Decode/Render
                │
                ▼
      Store in Memory Cache + Disk Cache
                │
                ▼
            Send to UI


---

### 3. Dependency Installation Flow

    Daemon Startup
    │
    ├─ Load embedded manifest
    ├─ Check runtime directory
    ├─ Download FFmpeg / PDFium if missing
    ├─ Validate checksums
    ├─ Extract to sandboxed directory
    └─ Register with thumbnail subsystem


---

## Daemon

The daemon runs independently of the UI and exposes structured commands for:

- Directory listing  
- Metadata lookup  
- Search queries  
- Thumbnail requests  
- File opening  
- Background scanning  
- Graceful shutdown  

It ensures:
- Cancelable queries  
- Zero blocking of the UI  
- Consistent performance across platforms  

---

## IPC Protocol

- Transport: **TCP (localhost)**  
- Encoding: **Length-prefixed frames**  
- Payload: **JSON**  
- Commands grouped into categories:
  - Directory operations  
  - Search  
  - Thumbnails  
  - System actions (open file, shutdown)  
  - Engine status/metrics  

The protocol begins with a versioned handshake allowing forward-compatible evolution.

---

## Indexing Engine

The indexing engine (`EngineRuntime`) maintains:

- File paths  
- File/folder type  
- Sizes  
- Modification timestamps  
- Thumbnail availability  
- Categorization (image, video, document, etc.)  

Capabilities:

- Thread-safe lookup and mutation  
- Fast directory queries  
- Stable ID mapping  
- Incremental rescanning  

---

## Thumbnail System

### Supported formats
- Images (PNG, JPEG, WebP, etc.)  
- PDFs (via PDFium)  
- Videos (via FFmpeg)  

### Features
- Dedicated worker thread  
- Non-blocking job queue  
- In-memory LRU cache  
- Disk cache for persisted thumbnails  
- Regeneration when stale or missing  

---

## Frontend Technology

- **Tauri** for cross-platform desktop runtime  
- **React + TypeScript** for UI  
- **Virtualized rendering** for large directories  
- **Smooth navigation & transitions**  
- **Thumbnail streaming from the daemon**  
- **State-independent, declarative rendering**  

---

## Selection & Lasso System

Lunio includes a high-precision, view-agnostic selection engine:

- Ctrl / Cmd additive selection  
- Shift range selection  
- Drag-box selection  
- Freehand lasso selection  
- Path smoothing using **Bézier curves**  
- Noise reduction via **Savitzky–Golay filter**  
- Point-in-polygon hit testing for accuracy  

---

## Performance Model

The system is designed to keep the UI responsive at all times:

- All filesystem work is asynchronous  
- `tokio::mpsc` worker model for request routing  
- One-shot channels for isolated responses  
- Abortable tasks for directory listing and search  
- Incremental results streamed to UI  
- No synchronous filesystem calls inside the frontend  

---

## Current Status

- Fully operational daemon  
- Async IPC protocol implemented  
- Thumbnail system complete (images, PDF, video)  
- Runtime installer for FFmpeg & PDFium  
- Stable indexing engine  
- Responsive Tauri + React frontend  
- Advanced selection and lasso mechanics  
- Incremental directory listing with cancellation  
- Smooth, non-blocking UI  

---

## License

Proprietary — All rights reserved.