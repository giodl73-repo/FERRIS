# Pulse 37 checkout normalization receipt

Disposition: **pass**.

Pulse 37 preserves the historical Pulse 35 cutoff identities that caused
Pulse 36's immutable 2/8-versus-6/8 copy-verification failure, while rebinding
the current public release to the exact LF Git-clean bytes required by its
release-root `.gitattributes` rule.

A disposable Windows alternate index used `core.autocrlf=true`, Git's clean
filter, and `git checkout-index`; it did not copy the ambient working tree.
All 8/8 current manifest file size/hash bindings passed. The six text files
materialized with LF and zero CR bytes; `qualification-receipt.json` and
`root-cause-report.json` remained byte-identical.

The machine-readable receipt is
`PULSE-37-CHECKOUT-NORMALIZATION-RECEIPT.json`. Its raw SHA-256 is
`sha256:9c6f61340af9d6e7bcd4d294c7916d34c16c226d0c4ccf7d28c812465658bff6`;
its sealed payload identity is
`sha256:e312d8265c406c6330d537e24913168508cab6dd40018bcb36bbbc1e2116bfae`.

This evidence records no FERRIS or diagnostic execution, no new diagnostic
authority, no product-file modification, and no alteration of the permanently
invalid Pulse 36 result.
