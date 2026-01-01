# realmlib-ffi

This repository provides a **C ABI / FFI wrapper** around `realmlib`, allowing it to be used from non-Rust environments such as **C# (.NET)** via P/Invoke.

It is designed to act as a **thin, stable boundary** between engine-agnostic world logic (Rust) and higher-level clients (C#, Godot, etc.).

---

## Purpose

- Expose `realmlib` functionality through a **C-compatible ABI**
- Enable **memory-based world updates** (no file or network assumptions)
- Keep `realmlib` fully in Rust while supporting other languages
- Serve as the native backend for:
  - LibreWorlds World Adapter
  - Godot hosts
  - VR or future engine integrations

This crate does **not** implement protocol logic, asset downloading, or client state machines.

---

## Architecture

