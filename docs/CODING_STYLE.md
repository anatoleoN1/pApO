# pApO Coding Style

Version: 1.0

This document defines the coding conventions used throughout the pApO Project.

All contributors should follow these guidelines to ensure consistency across the entire codebase.

---

# 1. General Principles

The code must always be:

- Simple
- Readable
- Consistent
- Documented
- Maintainable

Readable code is preferred over clever code.

---

# 2. Programming Language

The project is primarily written in Rust.

Any additional programming language must be technically justified.

---

# 3. Formatting

The official Rust formatter (`rustfmt`) must always be used.

Formatting must never be done manually.

Before committing:

cargo fmt

---

# 4. Static Analysis

Every crate should pass Clippy without warnings whenever possible.

Before committing:

cargo clippy

Warnings should be fixed rather than ignored.

---

# 5. Naming Conventions

## Crates

Use kebab-case.

Examples:

- papo-server
- papo-config
- papo-updater

---

## Modules

Use snake_case.

Examples:

audio.rs

network.rs

version.rs

---

## Files

Use snake_case.

Never use spaces.

---

## Functions

Use snake_case.

Example:

connect_to_server()

---

## Variables

Use snake_case.

Example:

connection_state

---

## Constants

Use UPPER_SNAKE_CASE.

Example:

DEFAULT_PORT

---

## Structures

Use PascalCase.

Example:

Version

PacketHeader

AudioFrame

---

## Enums

Use PascalCase.

Example:

ConnectionState

CallStatus

---

## Traits

Traits describe behaviour.

Use PascalCase.

Example:

Serializable

AudioDevice

NetworkInterface

---

# 6. Documentation

Every public item must include Rust documentation.

Example:

/// Represents a PPP packet.

Undocumented public APIs should not be merged.

---

# 7. Comments

Comments should explain:

WHY

not

WHAT

Bad example:

// Increment x

Good example:

// Retry the connection because the server may still be starting.

---

# 8. Error Handling

Rust's Result type should be preferred.

Panics should be avoided except for unrecoverable programming errors.

---

# 9. Dependencies

Keep dependencies minimal.

Before adding one, ask:

- Is it necessary?
- Is it maintained?
- Does it improve the project?

---

# 10. Unsafe Rust

Unsafe Rust is forbidden unless absolutely necessary.

Every unsafe block must be documented.

---

# 11. Testing

Every important feature should include automated tests.

Tests should be deterministic and reproducible.

---

# 12. Git

Each commit should contain one logical change.

Good example:

Implement PPP packet serializer

Bad example:

Fix bugs + update README + change UI

---

# 13. Architecture

A crate should have a single responsibility.

Common code belongs in the common crate.

Application-specific code must remain inside its own crate.

---

# 14. Backward Compatibility

Breaking changes should be avoided.

When unavoidable, they must be documented.

---

# 15. Final Rule

Write code as if someone else will maintain it in ten years.

That person might be you.