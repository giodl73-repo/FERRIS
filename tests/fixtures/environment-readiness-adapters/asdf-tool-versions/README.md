# asdf `.tool-versions` Adapter Evaluation Fixtures

These fixtures support Environment Readiness Pulse 04. They evaluate whether
asdf `.tool-versions` can be translated losslessly into READINESS-001 V1
without invoking asdf or a plugin.

`upstream-asdf.tool-versions` is an exact copy of the root `.tool-versions`
file at asdf revision
[`ca98e44ff49cb0a38966b23b42db962203478b59`](https://github.com/asdf-vm/asdf/blob/ca98e44ff49cb0a38966b23b42db962203478b59/.tool-versions).
The other files are public synthetic controls derived from the forms documented
in asdf's
[`configuration.md`](https://github.com/asdf-vm/asdf/blob/ca98e44ff49cb0a38966b23b42db962203478b59/docs/manage/configuration.md).

The test-only evaluator deliberately does not produce Ferris requirements.
Every fixture retains at least one semantic dependency that READINESS-001 V1
cannot represent without guessing:

- a `.tool-versions` tool name is an asdf plugin identifier, not a guaranteed
  executable leaf;
- ordinary versions and ordered alternatives require version-selection
  semantics absent from V1;
- `system` delegates selection back to asdf and the host;
- `ref:` requires plugin-owned resolution; and
- `path:` identifies an asdf installation source, not a Ferris
  repository-relative path requirement.

The fixtures contain no private repository or adopter data.
