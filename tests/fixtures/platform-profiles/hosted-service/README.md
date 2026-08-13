# Hosted Service Profile Family

Status: Controlled family under Pulse 06

Both revisions expose only an in-process owner API. They open no socket and
use no network, database, credential, TLS provider, external runtime, or
deployment system.

Revision `r1` supports one health request. Revision `r2` adds explicit
readiness state and preserves `unavailable` until the owner marks the service
ready. Both revisions retain malformed-request and cancellation outcomes.
