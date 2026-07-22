# Building pApO

Version: 1.0

This document explains how to build every component of the project.

---

# Requirements

- Rust (stable)
- Cargo
- Git

---

# Clone the repository

git clone <repository>

cd papo

---

# Build the workspace

cargo build

---

# Build in release mode

cargo build --release

---

# Run tests

cargo test

---

# Format the source code

cargo fmt

---

# Static analysis

cargo clippy

---

# Generate documentation

cargo doc --open

---

# Workspace

The project uses a Cargo Workspace.

All crates are built together unless specified otherwise.

---

# Individual Crates

To build one crate only:

cargo build -p crate_name

Example:

cargo build -p ppp

---

# Continuous Integration

Every commit should successfully:

- compile;
- pass all tests;
- pass Clippy;
- be correctly formatted.