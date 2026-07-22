# Contributing to pApO

Welcome to the pApO Project.

This document defines the development rules and conventions used throughout the project.

Every contributor is expected to follow these guidelines in order to keep the codebase clean, maintainable and consistent.

---

# 1. Project Philosophy

pApO is designed to be:

- Simple
- Reliable
- Maintainable
- Efficient
- Secure

Every design decision should respect these five principles.

When several solutions exist, the simplest one should be preferred.

---

# 2. Development Philosophy

Before writing code:

Design

↓

Validate

↓

Implement

↓

Test

↓

Document

Code must never be written before the architecture has been validated.

---

# 3. Programming Language

The entire project is primarily written in Rust.

Introducing another language must be technically justified.

---

# 4. Repository Organization

Each crate has a single responsibility.

common/
    Shared utilities and common types.

ppp/
    pApO communication protocol.

papos/
    Embedded operating system.

papo-server/
    Server implementation.

papo-config/
    Desktop configuration software.

papo-simulator/
    Virtual pApO devices.

papo-flash/
    Firmware recovery utility.

papo-factory/
    Manufacturing and validation tools.

papo-updater/
    Update packaging and deployment tools.

No crate should contain unrelated functionality.

---

# 5. Code Style

The official Rust formatting rules must always be respected.

Formatting is performed using:

cargo fmt

Static analysis is performed using:

cargo clippy

Warnings should be fixed whenever possible.

---

# 6. Documentation

Every public function, type and module must be documented.

Documentation is considered part of the source code.

Undocumented public APIs should not be merged.

---

# 7. Testing

Every important feature should include automated tests.

A feature is considered complete only if:

- it compiles;
- it passes all tests;
- it is documented.

---

# 8. Dependencies

External dependencies should remain limited.

Before adding a dependency, ask:

- Is it actively maintained?
- Is it necessary?
- Can the same functionality be implemented reasonably without it?

Avoid unnecessary dependencies.

---

# 9. Shared Code

Code duplication should be avoided.

Reusable components belong in the appropriate shared crate.

The "common" crate must remain organized and should never become a miscellaneous storage area.

---

# 10. Commits

Each commit should represent one logical change.

Commit messages should be short and descriptive.

Example:

Initialize Cargo workspace

or

Implement PPP packet serializer

Avoid combining unrelated modifications in a single commit.

---

# 11. Pull Requests

Every pull request should:

- solve one problem;
- remain reasonably small;
- include documentation updates when necessary;
- include tests when applicable.

---

# 12. Backward Compatibility

Backward compatibility should be preserved whenever reasonably possible.

Breaking changes must be documented.

Older supported pApO devices should continue to function whenever technically possible.

---

# 13. Security

Security has priority over convenience.

Private keys must never leave a pApO device.

All network communications must follow the PPP specification.

---

# 14. Project License

This project is distributed under the pApO Public License Version 1.0 (PPL-1.0).

All contributors agree that their contributions may be modified, integrated or rejected by the project maintainers.

Submitting code does not transfer ownership of the project.

---

# 15. Final Rule

When in doubt:

Choose the simplest solution that remains reliable.

The goal of pApO is not to become the most feature-rich communication system.

The goal is to build the best connected walkie-talkie.