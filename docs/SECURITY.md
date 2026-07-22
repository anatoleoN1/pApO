# Security Policy

Version: 1.0

Security is a core objective of the pApO Project.

---

# Security Principles

The project prioritises:

- Confidentiality
- Integrity
- Availability

---

# Cryptography

Only modern and publicly reviewed cryptographic algorithms should be used.

Home-made cryptography is forbidden.

---

# Private Keys

Private keys must never leave the device.

They must never be transmitted over the network.

---

# Authentication

Every device must authenticate before accessing server services.

---

# Secure Updates

Every update package must be digitally signed.

Unsigned updates must always be rejected.

---

# Responsible Disclosure

Security vulnerabilities should be reported privately to the project maintainers.

Public disclosure should occur only after a fix has been prepared.

---

# Dependencies

Dependencies should be updated regularly to receive security fixes.

---

# Logging

Sensitive information must never appear in log files.

Examples:

- passwords
- private keys
- authentication tokens

---

# Final Rule

Security must never be sacrificed for convenience.