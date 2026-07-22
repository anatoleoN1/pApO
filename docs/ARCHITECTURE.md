# pApO Project Architecture

## Overview

The pApO Project is a modular communication system designed around a connected push-to-talk device.

The project is divided into several independent Rust crates, each having a single responsibility.

This architecture aims to provide:

- Simplicity
- Reliability
- Security
- Maintainability
- Scalability

Every component should remain as independent as possible.

---

# Repository Structure

```
papo/
│
├── common/
├── ppp/
├── papos/
├── papo-server/
├── papo-config/
├── papo-simulator/
├── papo-flash/
├── papo-factory/
├── papo-updater/
│
└── docs/
```

Each crate is responsible for one specific part of the project.

---

# Crates

## common

Shared utilities used across multiple crates.

Examples:

- Version management
- Common errors
- Shared constants
- Generic utilities

The common crate must never become a miscellaneous storage area.

---

## ppp

PPP (pApO Packet Protocol) is the communication protocol shared by every component.

Responsibilities:

- Packet serialization
- Packet deserialization
- Protocol definitions
- Protocol versioning

PPP does not know anything about the operating system, the server implementation or the user interface.

---

## papos

Embedded operating system running on every pApO device.

Responsibilities:

- Hardware management
- Audio
- Networking
- User interface
- Contacts
- Calls
- Settings
- Update client

---

## papo-server

Central server responsible for coordinating communications.

Responsibilities:

- Authentication
- Device management
- Contact synchronization
- Update distribution
- Session management

The server should remain stateless whenever possible.

---

## papo-config

Desktop application used to configure a pApO device.

Responsibilities:

- Device configuration
- Backup
- Restore
- Diagnostics

---

## papo-simulator

Software used during development.

Responsibilities:

- Simulate one or more virtual pApO devices.
- Test PPP.
- Test the server.
- Test future features before hardware exists.

The simulator should reproduce the behaviour of a real device as accurately as possible.

---

## papo-flash

Recovery utility.

Responsibilities:

- Install firmware
- Reinstall firmware
- Factory reset
- Emergency recovery

---

## papo-factory

Manufacturing software.

Responsibilities:

- Hardware validation
- Functional testing
- Device registration
- Factory programming

This crate is not distributed to end users.

---

## papo-updater

Update generation utility.

Responsibilities:

- Build update packages
- Sign updates
- Verify update integrity
- Publish updates to the server

---

# Dependency Rules

Each crate should only depend on what is strictly necessary.

Preferred dependency graph:

common

↓

ppp

↓

papos
papo-server
papo-config
papo-simulator
papo-flash
papo-factory
papo-updater

Circular dependencies are forbidden.

---

# Design Principles

Every crate should follow the Single Responsibility Principle.

A crate should only solve one problem.

When new functionality is required, prefer extending the appropriate crate rather than creating unnecessary dependencies.

---

# Documentation

Architecture decisions should always be documented before implementation.

Documentation has the same importance as source code.

Every major architectural decision should be recorded.

---

# Future Evolution

The architecture is designed to support future generations of pApO devices.

Whenever possible, new functionality should be added without breaking existing components.

Backward compatibility should be preserved unless a breaking change is fully justified and documented.

---

# Final Principle

A simple architecture is preferable to a complex architecture.

If two solutions provide the same result, the simplest one should always be chosen.