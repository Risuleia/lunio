# Lunio

Lunio is a high-performance, cross-platform desktop file explorer powered by a custom Rust daemon and a Tauri + React frontend. It is engineered for responsiveness, scalability, and media-heavy workloads, with a strict separation between UI rendering and filesystem operations. The system is capable of handling large directories, complex metadata workloads, and asynchronous thumbnail generation without blocking the interface.

---

## Overview

Lunio adopts a daemon-driven architecture where all filesystem, indexing, and processing responsibilities are delegated to a persistent background service. The frontend communicates with this daemon through a structured TCP protocol, ensuring consistent performance and a stable API surface.

Key characteristics include:
- Dedicated Rust daemon for all file operations  
- Fully asynchronous IPC communication  
- Non-blocking UI with cancelable operations  
- Scalable indexing and metadata caching  
- High-quality thumbnail generation for images, videos, and PDFs  

---

## Architecture

Lunio consists of three main components:

### 1. Daemon (Rust)
A long-running process responsible for:
- Directory scanning  
- Metadata extraction  
- File and folder operations  
- Incremental indexing  
- Thumbnail generation  
- Runtime dependency management  
- Request processing and concurrency control  

### 2. IPC Layer (`lunio_client`)
A Rust client library that provides:
- TCP-based communication over localhost  
- Length-prefixed JSON message encoding  
- Typed request/response definitions  
- Versioned handshake and compatibility checks  
- Asynchronous, cancelable operations  

### 3. Frontend (Tauri + React)
A modern user interface that supports:
- Grid, List, and Masonry layouts  
- High-performance virtualized rendering  
- Complex selection mechanics  
- Thumbnail streaming  
- Sorting and grouping by multiple attributes  

---

## Core Features

### File Operations
- Asynchronous directory listing  
- Fast metadata retrieval  
- Full-text search  
- Native OS file opening  
- Background rescanning  

### User Interface
- Grid, List, and Masonry views  
- Sorting (name, size, date, type)  
- Grouping (extension, type, date)  
- Instant visual feedback from daemon responses  

### Selection System
- Ctrl / Cmd multi-selection  
- Shift range selection  
- Box (drag) selection  
- Freeform lasso selection  
- Escape clear / Ctrl+A select all  
- High-precision hit-testing via point-in-polygon algorithms  
- Path smoothing using Bézier curves and Savitzky–Golay filtering  

---

## Daemon

The daemon owns a centralized, thread-safe indexing engine (`EngineRuntime`) stored behind an `Arc`. It processes all client requests asynchronously and ensures that no operation blocks the UI. Responsibilities include:

- Maintaining an in-memory index  
- Syncing state with disk changes  
- Scheduling and executing thumbnail jobs  
- Managing external tools (FFmpeg, PDFium)  
- Providing cancelable directory queries  

---

## IPC Protocol

Lunio’s protocol is designed for safety, forward compatibility, and performance:

- **Transport:** TCP  
- **Encoding:** Length-prefixed frames with JSON payloads  
- **Handshake:** Version-negotiated session initialization  
- **Message Types:**  
  - Directory listing  
  - Background scan  
  - Thumbnail request  
  - Search query  
  - File open request  
  - Engine status queries  
  - Shutdown  

All operations are asynchronous, and long-running actions can be cancelled by the client.

---

## Indexing Engine

The indexing engine tracks:
- File paths  
- Sizes  
- Modification timestamps  
- Types (file, directory)  
- Categories (media, documents, etc.)  
- Thumbnail availability  

It supports:
- Fast directory lookups  
- Incremental scanning  
- Concurrent reads and writes  
- Stable ID-based lookups  

---

## Thumbnails

The thumbnail subsystem is built for reliability and concurrency:

### Supported Formats
- Images (via the `image` crate)  
- PDFs (via PDFium)  
- Videos (via FFmpeg)  

### Pipeline
- Job-queue-based architecture  
- Dedicated worker thread  
- Non-blocking scheduling  
- Progressive delivery to the UI  

### Caching
- In-memory LRU cache  
- Persistent on-disk cache  
- Regeneration when data is missing or outdated  

---

## Runtime Dependencies

To maintain portability, Lunio downloads required external binaries at runtime:

- **FFmpeg** for video frame extraction  
- **PDFium** for PDF rendering  

The daemon:
1. Loads a platform manifest  
2. Downloads the required tools  
3. Verifies integrity checks  
4. Extracts them into a private runtime directory  
5. Registers them with the thumbnail subsystem  

If a dependency cannot be installed, the system continues operating with the corresponding feature disabled and retries on next launch.

---

## Frontend

The Tauri + React interface focuses on responsiveness and clarity:

- View virtualization for large directories  
- Real-time rendering updates  
- Thumbnail streaming and progressive loading  
- Advanced selection tools  
- Fully async communication layer  
- Graceful cancellation of outdated operations  

---

## Performance Characteristics

Major performance considerations:
- All IPC operations are asynchronous  
- Directory listings can be cancelled mid-processing  
- Thumbnail generation is fully offloaded to worker threads  
- The engine avoids locking bottlenecks via fine-grained concurrency  
- UI never directly touches filesystem APIs  

Navigating between folders cancels previous tasks immediately, ensuring smooth and uninterrupted interaction.

---

## Current Status

Lunio currently includes:

- [X] Functional explorer with all core navigation features  
- [X] Fully implemented async daemon and IPC client  
- [X] Thumbnail generation for images, PDFs, and videos  
- [X] Automatic FFmpeg/PDFium management  
- [X] Cancelable directory operations  
- [X] Complete selection and lasso system  
- [X] Stable, versioned protocol  
- [X] Responsive, production-grade user interface  

The foundation is complete, and further feature development is ongoing.

