# pApO Versioning

Version: 1.0

This document describes the versioning policy used throughout the pApO Project.

---

# 1. Semantic Versioning

All software components follow Semantic Versioning.

MAJOR.MINOR.PATCH

Example:

1.0.0

---

# 2. Major Version

Increment the MAJOR version when incompatible changes are introduced.

Example:

1.0.0 → 2.0.0

---

# 3. Minor Version

Increment the MINOR version when adding new backwards-compatible features.

Example:

1.0.0 → 1.1.0

---

# 4. Patch Version

Increment the PATCH version when fixing bugs without changing functionality.

Example:

1.0.0 → 1.0.1

---

# 5. Independent Versioning

Each crate has its own version.

Examples:

- common
- ppp
- papos
- papo-server

may evolve independently.

---

# 6. Device Compatibility

Every pApO device has a hardware generation.

Examples:

pApO 1

pApO 2

pApO 3

Software updates must always verify hardware compatibility before installation.

---

# 7. PPP Compatibility

PPP must negotiate the highest protocol version supported by both devices.

Communication should remain possible whenever compatible versions exist.

---

# 8. Final Rule

Never publish a breaking release without documenting the changes.