# Browser WASM Profile Family

Status: Controlled family under Pulse 08

Both revisions escape caller text into deterministic HTML and compile for
`wasm32-unknown-unknown`. Revision `r2` adds validated language metadata and
an `aria-live` contract. No JavaScript, DOM, browser, network, bundler, or
deployment system is invoked.
