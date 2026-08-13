# Why Quarantine Exists

Quarantine protects the meaning of a blind score. If implementation authors
can inspect hidden cases, retry them, or tune against them, the result no
longer measures behavior on unseen evidence.

That protection has a cost: a category-only failure may be difficult to
debug. Pulse 17 demonstrates the tradeoff. Its score is valid and immutable,
but its public category does not disclose an actionable case.

Pulse 20 improves future programs without rewriting history. A future contract
may precommit a sanitized-reproducer tier. After the one allowed score, an
independent custodian creates a new minimal public case that reproduces only
the released category and proves that it contains none of the original hidden
inputs, canaries, identifiers, paths, digests, or oracle details.

The original score never changes. Both the original fixture and the sanitized
reproducer are retired from certification. A later certification attempt uses
a completely fresh hidden package.

Pulse 17 did not precommit this process, so it remains closed.

