# CLI and Configuration Profile Family

Status: Controlled family under Pulse 05

Revision `r1` resolves `--name`, then `FERRIS_FIXTURE_NAME`, then a built-in
default. Revision `r2` adds one explicit `--config` file between CLI and
environment precedence. It reads no implicit path and bounds the file at
1 KiB.

Process tests cover precedence, unknown arguments, missing files, malformed
content, oversized content, and non-UTF-8 bytes. The consumers have no
external dependencies, network, credentials, installation behavior, native
code, provider, service, or deployment.
