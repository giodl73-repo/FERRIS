# Root cause: Pulse 58 orders execution but intentionally does not publish

Pulse 53 preserved a valid Pulse 47 witness of bounded Pulse 43 publication
failure, but it bound exact Pulse 52 ordering and retained-binary custody
rather than the later exact Pulse 58 live-capability executor. Pulse 58 closed
the public-before-private ordering gap for live Pulse 56 capabilities, yet it
returns only privacy-safe ordered events and private execution accounting. It
does not own terminal publication or witness retention.

Pulse 59 closes only that terminal gap. It delegates exact Pulse 58 production
and fake qualification orchestration unchanged, waits until Pulse 58 has
completed and removed its private runtime root, then derives one fresh sibling
terminal custody location with no caller injection. Exact Pulse 51/Pulse 47
publication semantics run once over that sibling root, preserving Pulse 53's
three completed terminal classes without adding a new execution event.

This is infrastructure only. It creates no authority, performs no real FERRIS
diagnostic, and does not alter any historical pulse disposition.
