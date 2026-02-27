# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

CodeCrafters "Build Your Own Shell" challenge implemented in Rust. A POSIX-like shell that parses commands, runs builtins (`echo`, `exit`, `type`), and executes external programs found in `$PATH`.

**Purpose**: The author is practicing Rust. Your role is to evaluate code and suggest improvements in idiomatic Rust patterns and practices.

## Build & Run Commands

- **Build**: `cargo build --release --target-dir=/tmp/codecrafters-build-shell-rust`
- **Run locally**: `./your_program.sh`
- **Check**: `cargo check`
- **Lint**: `cargo clippy`
- **Submit to CodeCrafters**: `git push origin master`

No tests exist yet. The project is validated by CodeCrafters' remote test suite on push.

## Architecture

The crate is structured as a library (`lib.rs`) exposing two modules, with `main.rs` consuming them:

- **`main.rs`** — Contains the `Shell` struct with the REPL loop. Parses input into command + args, dispatches to builtins or external commands via `std::process::Command`.
- **`commands.rs`** — `Command` enum for builtins. Uses `TryFrom<&str>` for parsing and a `run(&self, args)` method for dispatch. Each builtin has a standalone `run_*` function.
- **`utils.rs`** — `find_os_executable` searches `$PATH` directories for an executable matching a given name using the `is_executable` crate.

## Key Dependencies

- `is_executable` — cross-platform check for file executability
- `anyhow` / `thiserror` — declared but not yet used; `CommandError` is manually implemented

## Conventions

- Rust 2021 edition, minimum version 1.80
- Release builds go to `/tmp/codecrafters-build-shell-rust`
- Main branch is `master`
