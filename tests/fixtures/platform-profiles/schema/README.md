# Platform Profile Schema Controls

Status: Frozen inputs for Pulse 02

`valid/pure-data-r1.json` is a schema exemplar, not a completed pure-data
family. Its placeholder digests and `not-observed` lifecycle controls make
that limitation explicit.

`controls.json` defines exact single-operation variants that the Pulse 03
test harness must construct without retaining copied fixture trees. The raw
duplicate, malformed, and oversized controls operate on bytes before ordinary
JSON deserialization.

Pulse 03 corrected the malformed control from one-byte to three-byte
truncation. The base file ends with a platform checkout newline, so removing
one or two bytes may remove only line-ending bytes on CRLF checkouts. Removing
three bytes removes the closing object delimiter on both LF and CRLF
checkouts and creates the intended single-defect malformed input.

Expected classes are:

| Class | Meaning |
|---|---|
| `valid` | The record satisfies the v1 schema and program policy |
| `unsupported` | The schema identity is well-formed but not supported |
| `invalid` | Syntax, shape, metadata, ambiguity, duplicate, or state is invalid |
| `blocked` | The canonical record exceeds the frozen 4 MiB bound |

The secret-bearing metadata string is intentionally synthetic and must never
be copied into logs beyond the exact negative fixture. It is not a credential.
