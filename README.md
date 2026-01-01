# realmlib-ffi

**FFI wrapper for realmlib** — exposes native Rust APIs for use from C# and other hosts.

This repository provides a **C ABI / FFI wrapper** around realmlib (the core Rust world logic), allowing it to be used from non-Rust environments such as C# (.NET) via P/Invoke.

It is designed to act as a **thin, stable boundary** between engine-agnostic world logic (Rust) and higher-level clients (C#, Godot, etc.).

## Purpose

- Expose realmlib functionality through a C-compatible ABI
- Enable memory-based world updates (no file or network assumptions)
- Keep realmlib fully in Rust while supporting other languages
- Serve as the native backend for:
  - LibreWorlds World Adapter
  - Godot hosts
  - Future VR or engine integrations

> This crate does **not** implement protocol logic, asset downloading, or client state machines.

## Architecture

C# / .NET Client  
↓ (P/Invoke)  
realmlib-ffi (this repository)  
↓  
realmlib (Rust)  
↓  
Engine / Renderer

The FFI layer is intentionally **minimal and declarative**.

## Current Status

- Builds as a native shared library (cdylib)
- Exports C ABI functions via extern "C"
- Successfully called from C# using P/Invoke

**Implemented / Tested:**
- Basic initialization and shutdown
- Object addition

**In Progress / Pending:**
- Full world state initialization
- RWX parsing and geometry creation
- Transform and terrain updates

## Exported API (Current)

The API surface is intentionally small and will grow incrementally as needed.

| Function | Description |
|----------|-------------|
| realmlib_init() | Initializes the realmlib world state |
| realmlib_add_object(id: u64, model_name: *const c_char, data: *const u8, length: usize) | Adds an object with in-memory asset data |
| realmlib_shutdown() | Cleans up resources |

All asset data is passed **from memory** (no disk or URL access).

## Build

**Requirements:**
- Rust (stable)
- Cargo

To build:

cargo build --release

Artifacts are produced in target/release:

- **Windows:** realmlib_ffi.dll
- **Linux:** librealmlib_ffi.so
- **macOS:** librealmlib_ffi.dylib

## Usage from C#

The native library is consumed via **P/Invoke** from .NET.

The calling code is responsible for:
- Asset acquisition & caching
- Protocol decoding
- Lifetime management

(Example P/Invoke declarations can be added to a companion C# project like LibreWorlds.WorldAdapter.)

## Non-Goals

This repository intentionally does **not** handle:

- Networking or protocols
- Authentication or sessions
- Asset downloading or ZIP handling
- Client state machines
- UI or rendering frameworks

Those concerns belong to higher layers.

## Design Principle

> **The client decides what the world is.**  
> **realmlib decides how the world exists.**  
> This FFI layer keeps those responsibilities separate.

## Related Projects

- LibreWorlds.WorldAdapter – C# adapter that feeds world state into this FFI layer
- realmlib – Rust world/engine logic (upstream dependency)

## License

MIT License – chosen for maximum engine and language compatibility.
