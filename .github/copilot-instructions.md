# Copilot Instructions for AI Agents

## Project Overview
This repository is a live learning guide for [The Rust Programming Language Book](https://rust-book.cs.brown.edu/experiment-intro.html). It is organized by chapters, with each chapter containing notes and code examples. The goal is to complete all book exercises, including optional ones.

## Structure & Key Files
- `Cargo.toml` (root): Rust workspace definition. Lists member projects.
- `chapter_1/`, `chapter_2/`: Each chapter has its own folder, notes, and subprojects.
- Example projects:
  - `chapter_1/hello_cargo/`
  - `chapter_2/guessing_game/`
- Each project has its own `Cargo.toml` and `src/main.rs`.
- Notes for each chapter are in `notes.txt`.

## Developer Workflows
- **Build & Run:**
  - Use `cargo run` in a project directory for quick testing.
  - Use `cargo build` to compile only.
  - Use `cargo check` to verify code compiles without building.
  - Release builds: `cargo build --release` (optimized, use for benchmarking).
- **Debugging:**
  - Debug binaries are in `target/debug/`.
  - Release binaries are in `target/release/`.
- **VSCode Setup:**
  - Recommended extensions: rust-analyzer, codelldb, even better toml, dependi, error lens.
  - rust-analyzer activates if a `Cargo.toml` or `rust-project.json` is present in the workspace.

## Patterns & Conventions
- Each Rust project is a crate, source code in `src/`.
- All dependencies managed via Cargo.
- No custom build scripts or nonstandard layouts detected.
- Follow Rust book conventions for code organization and naming.

## Integration Points
- No external APIs or service boundaries; all code is local and self-contained.
- No cross-crate dependencies beyond standard Cargo usage.

## Examples
- To run the guessing game:
  ```pwsh
  cd chapter_2/guessing_game
  cargo run
  ```
- To add a new exercise:
  1. Create a new folder under the relevant chapter.
  2. Initialize with `cargo new <name>`.
  3. Add to `[workspace.members]` in root `Cargo.toml`.

## References
- See `README.md` for project intent and extension recommendations.
- See chapter `notes.txt` for workflow tips and chapter-specific notes.

---
If any section is unclear or missing important project-specific details, please provide feedback to improve these instructions.
