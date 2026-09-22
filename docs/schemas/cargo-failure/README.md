# Cargo Failure Report Schema

`ferris.cargo-failure-report/v1` is the closed output record for passive
`ferris diagnose-cargo --stderr <FILE|->` classification. `-` reads one
complete input from standard input through EOF.

The record binds the complete caller-supplied input by digest and byte count.
It never retains raw stderr, executes Cargo, extracts package or path identity,
or interprets rustc and test failures. `unclassified` is a successful bounded
classification result, not a claim that the owner command or repository is
healthy.
