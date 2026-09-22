# Ferris Contract Compatibility Schemas

Status: Implemented incubation contracts

`ferris.contract-requirements/v1` lets an adopter name exact schema handling
that it requires from an installed Ferris binary. Each requirement asks for
either local acceptance or emission of one exact schema identifier.

`ferris.contract-compatibility-report/v1` is emitted by
`ferris contracts --requirements <JSON>`. A compatible report means every
explicit requirement is present in that binary's contract catalog with the
requested handling. An unmet or unknown requirement produces a report-bearing
`difference` result and process exit code 1.

The report does not compare record semantics, negotiate versions, migrate
records, inspect repositories, or establish a support duration. A future
Ferris release can run the same checked-in requirement set to accumulate
multi-release evidence without changing its meaning.
