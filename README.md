realmlib-ffi

This repository provides a C ABI / FFI wrapper around realmlib, allowing it to be used from non-Rust environments such as C# (.NET) via P/Invoke.

It is designed to act as a thin, stable boundary between engine-agnostic world logic (Rust) and higher-level clients (C#, Godot, etc.).

Purpose

• Expose realmlib functionality through a C-compatible ABI
• Enable memory-based world updates (no file or network assumptions)
• Keep realmlib fully in Rust while supporting other languages
• Serve as the native backend for LibreWorlds World Adapter, Godot hosts, and future VR or engine integrations

This crate does not implement protocol logic, asset downloading, or client state machines.

Architecture

C# / .NET Client
↓ (P/Invoke)
realmlib-ffi (this repository)
↓
realmlib (Rust)
↓
Engine / Renderer

The FFI layer is intentionally minimal and declarative.

Current Status

• Builds as a native shared library (cdylib)
• Exports C ABI functions via extern "C"
• Successfully called from C# using P/Invoke
• realmlib world state initialization – pending
• RWX parsing and geometry creation – pending
• Transform and terrain updates – pending

Exported API (current)

The API surface is intentionally small and will grow as needed.

Functions currently exported:

realmlib_init
realmlib_add_object(id, model_name, data, length)
realmlib_shutdown

All asset data is passed from memory, not from disk or URLs.

Build

Requirements:
• Rust (stable)
• Cargo

To build the native library:

cargo build --release

Artifacts are produced in target/release.

Platform outputs:
• Windows: realmlib_ffi.dll
• Linux: librealmlib_ffi.so
• macOS: librealmlib_ffi.dylib

Usage from C#

The native library is consumed via P/Invoke from .NET. The calling code is responsible for asset acquisition, caching, protocol decoding, and lifetime management.

Non-Goals

This repository intentionally does not handle:

• Networking or protocols
• Authentication or sessions
• Asset downloading or ZIP handling
• Client state machines
• UI or rendering frameworks

Those concerns belong to higher layers.

License

MIT (recommended for maximum engine and language compatibility).

Related Projects

• LibreWorlds.WorldAdapter – C# adapter that feeds world state into this FFI layer
• realmlib – Rust world/engine logic (upstream dependency)

Design Principle

The client decides what the world is.
realmlib decides how the world exists.
This FFI layer keeps those responsibilities separate.
