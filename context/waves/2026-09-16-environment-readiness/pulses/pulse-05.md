# Pulse 05: Private Enterprise Readiness Shadow

Status: Complete

## User outcome

Demonstrate that the implemented readiness path can identify a real
enterprise-consumer prerequisite gap before owner validation starts, then
confirm that the same declaration becomes ready when the missing tool is made
visible only to the invoking process.

## Authority

The user's fresh `continue` after Pulse 04 authorizes one local Windows,
read-only shadow over the two custody-bound private enterprise consumers,
publicly identified only as `EO-01` and `EO-02`:

- build Ferris at the exact current public revision;
- verify both private source trees are clean and bind their exact revisions in
  private session custody;
- create explicit temporary READINESS-001 declarations outside both consumers;
- run `doctor --requirements` first with the inherited environment and then
  with only the existing Cargo directory prepended process-locally;
- repeat successful observations to test deterministic identity;
- verify no private path or environment value appears in retained reports;
- remove every temporary declaration and raw result; and
- retain only public-safe aggregate evidence and an eleven-role closeout.

No owner command, asdf or plugin invocation, installation, persistent
environment change, source-consumer mutation, schema or product change,
performance measurement, CI replacement, support claim, or Pulse 06 is
authorized.

## Maximum effort

One exact Ferris build, two private consumers, one missing-versus-visible Cargo
comparison per selected workspace, deterministic reruns, cleanup, one
public-safe result, and one eleven-role review.

## Completion test

- exact Ferris and private source revisions are privately bound;
- all private source trees are clean before and after the shadow;
- inherited-environment reports identify absent Cargo before owner work;
- process-local Cargo visibility produces ready reports when every other
  declared prerequisite is present;
- repeated equivalent runs preserve report and selection identities;
- reports contain no private path, environment value, resolved path, username,
  or machine name;
- no owner command or candidate executable is launched by readiness;
- all temporary inputs and raw outputs are removed;
- targeted readiness tests and `git diff --check` pass; and
- the public record states the exact claim and limitations.

## Stop condition

Stop on any dirty source tree, failed build, invalid declaration, unexpected
result class, privacy leak, mutation, or cleanup failure. Do not repair an
environment, alter a consumer, execute owner validation, or retry with changed
inputs inside this pulse.

## Result

The exact public Ferris revision
`1cee93da0572ddb5d9c9f69629ce0bb6852590f1` was built with the locked
workspace. Both private consumers were clean at privately retained exact
revisions.

For each consumer, the inherited Windows environment produced a report-bearing
`blocked` result at exit 7 with exactly one non-satisfied required observation:
Cargo was `missing`. All other declared properties were satisfied. Prepending
the existing Cargo directory only to the Ferris child process changed each
result to `success` at exit 0. An identical repeat was byte-stable and preserved
report, selection, invocation, and result identities.

The retained-output scan found no private root, session path, user name,
machine name, resolved executable path, or process `PATH` value. Every evidence
retention flag remained false. No owner command ran, both source trees remained
clean, and all temporary declarations and raw outputs were removed.

The `EO-02` shadow also preserves a product limitation: V1 path observations
are rooted at one selected Cargo manifest and therefore do not establish
application-root readiness for a multi-workspace application.

Findings `FERRIS-794` through `FERRIS-797` are recorded in the
[public-safe result](../../../../docs/research/2026-09-19-private-enterprise-readiness-shadow.md).
The
[eleven-role review](../../../../docs/plans/reviews/FERRIS_PRIVATE_ENTERPRISE_READINESS_SHADOW_REVIEW.md)
accepts this bounded Windows evidence. Pulse 05 authority is exhausted; no
Pulse 06, application composition, adoption, support, or production claim
follows.
