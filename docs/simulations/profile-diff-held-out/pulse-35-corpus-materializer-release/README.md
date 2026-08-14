# Pulse 35 Public Corpus-Materializer Release

Status: complete public synthetic infrastructure release only

This standard-library adapter prospectively closes only Pulse 34's
generation-materialization gap. It does not execute Ferris, a candidate, a
diagnostic, an owner command, or a network operation; it modifies no product
file and does not reopen the immutable Pulse 34 result.

## Private seed and verification

Both materialization and verification require `--seed-file`: a readable
regular file containing exactly 32 bytes of CSPRNG material. Verification
rejects a commitment supplied without that private seed. It recomputes
`sha256("ferris-p35-seed-commitment-v1\0" || seed)`, every 64-hex-character
case ID, order token, and profile token.

Those public tokens use HMAC-SHA256 with the 256-bit seed as key and the
domain, ASCII purpose, and eight-byte counter as message. They are
pseudorandom, domain-separated outputs, not raw seed slices. Profile IDs
therefore contain seed-derived pseudorandom values; no output contains the
seed bytes or seed source path.

## Exact corpus and interaction closure

There are exactly 70 complete-coverage descriptors (maximum 512). Each role
binds its state, raw bytes where applicable, output-relative target, and an
explicit resolution contract: request spelling, platform/custody namespace,
template, substitution rule, relative base when applicable, and resolved
output-relative target. The verifier lexically resolves every spelling.
Relative paths resolve from their declared base; drive, extended, UNC, and
Unix absolute spellings resolve through explicit custody-root templates.
UNC normalization preserves `//server/share` authority rather than collapsing
it into an ordinary rooted path. Missing and directory roles use the same
binding.

The coverage manifest explicitly enumerates every required tuple and the
concrete case IDs satisfying it. The independent verifier derives and compares
all eight tuple catalogs from the bound artifacts and descriptors:

- metadata site × metadata byte boundary (20);
- metadata site × nonempty character kind (12);
- input position × path state × path form (54);
- input position × input-byte boundary (6);
- JSON value kind × member ordering (33);
- duplicate depth × failure position (20);
- expected result class × its exact JSON route (6); and
- success/difference × JSON/human format pair (4).

It also recomputes all 18 value-domain records, public input precedence, raw
bindings, and the explicit per-pair change-count witness. The counting
algorithm recursively compares object members over the union of names,
recurses array indexes over the union of indexes, treats a missing member or
index as one differing leaf, ignores object ordering, and counts unequal
scalar leaves as one. It records `null` for pairs that are not both valid
regular JSON inputs, otherwise the count and its `9,999`, `10,000`, or
`10,001` boundary witness when applicable.

## Publication durability

Every attempted directory synchronization returns and records either
`synced` or `unsupported`, with its `posix-fsync-directory` or
`win32-directory-flush` mechanism and error code. Unexpected failures
propagate. The case manifest records stage synchronization statuses and the
materialization summary/qualification receipt records stage, parent, publish,
and rollback statuses. A post-creation staging-sync failure removes the stage
and synchronizes that cleanup. Publication makes one rename attempt and zero
logical retries; a final parent-sync failure removes the published output and
synchronizes rollback, otherwise reports an explicit indeterminate error.
An unsupported filesystem is reported as unsupported, never claimed
synchronized.

## Qualification

Run only in a disposable copy:

```powershell
python -m unittest discover -s tests -v
python qualify.py --cycles 20
```

Qualification performs 20 isolated 70-case cycles, private-seed fresh-process
reloads, same-seed identity, different-seed divergence, regular-file and
exact-length seed controls, seed-required verification, replay, residue,
extra-output, semantic fake-coverage, rename-failure, staging-sync cleanup,
and final-sync rollback controls. It retains no corpus or seed material.

`public-manifest.json`, `qualification-receipt.json`, and `release-seal.json`
bind the exact release identities. Removal deletes this release and its
governance/test records only; it creates no diagnostic, scoring,
certification, support, or PLATFORM-001 authority.
