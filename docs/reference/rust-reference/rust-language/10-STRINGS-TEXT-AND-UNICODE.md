---
maxim_schema: maxim.frontmatter.v1
id: maxim:rust-language:strings-text-and-unicode
kind: guide
module: rust-language
section: languages
title: Strings, Text, and Unicode
status: source-custody
source_custody: partial
current_path: rust-language/10-STRINGS-TEXT-AND-UNICODE.md
canonical_path: rust-language/10-STRINGS-TEXT-AND-UNICODE.md
backsource_ids: [mdloom-backfill:rust-language:10-strings-text-and-unicode]
concepts: [String, str, UTF-8, char, grapheme, formatting, OsString, Path, bytes, encoding]
root_concepts: [strings]
index_roles: [guide]
remap_from: []
remap_to: []
updated: null
---
# Strings, Text, and Unicode

Rust's string model is more honest than most languages' and therefore initially
more annoying: `String`/`&str` are guaranteed **valid UTF-8**, indexing by
integer is forbidden, and `.len()` returns *bytes*, not characters. This is a
deliberate refusal to lie about the fact that "character" is ambiguous. Once you
internalize the byte / `char` / grapheme distinction, the design is a relief —
there are no encoding surprises hiding in your data.

```
+===============================================================================+
|                    TEXT REPRESENTATIONS AND BOUNDARIES                        |
+===============================================================================+

  UTF-8 STRING TYPES
  ------------------
  OWNED                    BORROWED VIEW           RELATION
  String   heap, growable  &str  slice of UTF-8    &String derefs to &str
    ptr,len,cap              ptr,len               "literal" : &'static str

  THE THREE LEVELS OF "CHARACTER"
  -------------------------------
  bytes      s.as_bytes()      -> &[u8]     UTF-8 code units   "é" = 2 bytes
  chars      s.chars()         -> char      Unicode scalars    "é" = 1 char (if NFC)
  graphemes  (crate)           -> &str      user clusters      family emoji = 1

  OPAQUE OS / PATH TYPES (NOT GUARANTEED UTF-8)    WHY NO s[i] ?
  ---------------------------------------------    -------------
  OsString / OsStr : platform-native string        integer indexing is O(n) AND
    representation; inspect through OS APIs        could split a multi-byte char.
  Path / PathBuf   : filesystem paths              Use chars/valid byte boundaries.
    (wrap OsStr)
```

## `String` vs `&str`

`&str` is a borrowed, immutable view into UTF-8 bytes (a fat pointer: data +
length). `String` owns a growable heap buffer. This is the same owned/borrowed
split as `Vec<T>`/`&[T]` — `String` is essentially `Vec<u8>` with a UTF-8
invariant, and `&str` is `&[u8]` with the same invariant.

```rust
let literal: &str = "hello";               // &'static str, baked in the binary
let owned: String = String::from("hello"); // or "hello".to_string() / "hello".to_owned()
let view: &str = &owned;                    // &String coerces to &str (deref coercion)
```

**API rule:** take `&str` parameters, return `String` when you produce new owned
text. `&str` accepts literals, `String` slices, and substrings uniformly:

```rust
fn shout(s: &str) -> String { s.to_uppercase() }   // borrows in, owns out
shout("hi");
shout(&owned);
```

`Cow<str>` (clone-on-write, [16](16-SMART-POINTERS-INTERIOR-MUTABILITY-AND-SELF-REFERENCE.md))
is the tool when a function *usually* returns a borrowed slice but *sometimes*
must allocate a modified copy.

## Why You Cannot Index a String

`s[3]` does not compile. Two reasons: (1) UTF-8 is variable-width, so byte offset
3 might land in the middle of a multi-byte character, and (2) integer indexing
implies O(1), but finding the Nth *character* is O(n). Rust refuses to hide
either fact. You choose the level explicitly:

```rust
let s = "café";               // 5 bytes: c a f é(2 bytes), 4 chars
assert_eq!(s.len(), 5);        // BYTES, not chars
assert_eq!(s.chars().count(), 4);
let third_char = s.chars().nth(2);           // Some('f') — O(n)
let slice = &s[0..3];                         // "caf" — byte range on a valid boundary
// let bad = &s[0..4];                        // PANIC at runtime: not a char boundary
for c in s.chars() { /* 'c','a','f','é' */ }
for b in s.bytes() { /* 99,97,102,195,169 */ }
```

Byte-range slicing `&s[a..b]` works but **panics at runtime** if `a` or `b` is
not a UTF-8 char boundary. Use `char_indices()` to find valid boundaries, or
prefer `chars()`/`split`/`find` which stay boundary-safe.

## The Three Levels of "Character"

| Level | API | `"é"` (composed) | `"👨‍👩‍👧"` family emoji |
|-------|-----|------------------|--------------------------|
| **byte** (`u8`) | `.as_bytes()`, `.bytes()` | 2 bytes | 18+ bytes |
| **char** (Unicode scalar) | `.chars()`, `.char_indices()` | 1 char (NFC) or 2 (NFD) | many chars (ZWJ-joined) |
| **grapheme** (user-perceived) | `unicode-segmentation` crate | 1 grapheme | 1 grapheme |

The standard library gives you bytes and `char`s. **Graphemes** — what a human
calls "one character" — require the external `unicode-segmentation` crate, because
grapheme segmentation is a large, versioned Unicode algorithm the std library
deliberately does not bundle. If you are counting "characters" for a UI cursor or
deletion boundary, you usually want extended grapheme clusters, not
`chars().count()`. Display width is a separate rendering problem: East Asian
width, combining marks, emoji presentation, fonts, and terminal policy all
matter. A crate such as `unicode-width` provides a terminal-oriented estimate,
not a universal layout answer. The key caveat is that **`chars().count()` is
neither grapheme count nor display-column width.**

## Formatting

`format!`, `println!`, `write!` use the same mini-language, powered by the
`Display` and `Debug` traits ([06](06-TRAITS-GENERICS-AND-ASSOCIATED-ITEMS.md)):

```rust
let name = "Ada"; let score = 3.14159;
format!("{name}: {score:.2}");         // "Ada: 3.14"  (captured identifier + precision)
format!("{0} {1} {0}", "a", "b");      // positional: "a b a"
format!("{:>8}", "hi");                // right-align width 8
format!("{:#x}", 255);                 // "0xff"  (alternate form)
format!("{:?}", vec![1, 2]);           // "[1, 2]"  Debug
format!("{:#?}", vec![1, 2]);          // pretty Debug (multi-line)
```

`{}` needs `Display` (human-facing, you implement it); `{:?}` needs `Debug`
(dev-facing, usually `#[derive(Debug)]`). Inline captured identifiers (`{name}`)
work since Rust 1.58.

## The OS / Path Boundary

Filesystem paths and OS strings are **not guaranteed UTF-8**. Unix platform
extensions expose their arbitrary bytes. Windows APIs use potentially
ill-formed UTF-16; Rust deliberately leaves `OsStr`'s internal encoding
unspecified, so use Windows `encode_wide`/`from_wide` extensions rather than
depending on an internal WTF-8-like representation. Portable code treats
`OsStr` as opaque. The standard library therefore has parallel types:

| UTF-8 world | OS world | Filesystem |
|-------------|----------|------------|
| `String` (owned) | `OsString` | `PathBuf` |
| `&str` (borrowed) | `&OsStr` | `&Path` |

```rust
use std::path::{Path, PathBuf};
let p = Path::new("/etc/hosts");
let joined: PathBuf = p.join("subdir");         // OS-correct separator handling
if let Some(name) = p.file_name() { /* &OsStr */ }
let maybe_utf8: Option<&str> = p.to_str();      // None if not valid UTF-8
```

Always build paths with `Path`/`PathBuf` and `.join()`, never by string
concatenation — it handles separators and platform quirks. Convert to `&str` only
at the edge with `.to_str()` (fallible) or `.to_string_lossy()` (replaces invalid
sequences with U+FFFD).

## Old World -> New World Bridge

| Old world | Rust | Difference |
|-----------|------|-----------|
| C# `string` (UTF-16, indexable) | `String`/`&str` (UTF-8, not indexable) | No `s[i]`; byte vs char explicit |
| C# `char` (UTF-16 code unit) | `char` (Unicode scalar, 4 bytes) | Rust `char` holds any scalar; C# `char` is half a surrogate pair |
| `StringBuilder` | `String` + `push_str`/`write!` | `String` is already growable |
| C `char*` (bytes, NUL-terminated) | `&[u8]` / `CStr` ([17](17-UNSAFE-RUST-FFI-AND-ABI.md)) | No implicit NUL terminator |
| `string.Length` (UTF-16 units) | `.len()` (bytes) / `.chars().count()` | Neither equals "grapheme count" |
| `Path.Combine` (.NET) | `Path::join` | Same intent, OS-correct |
| `string.Format` / interpolation | `format!` / `{name}` | Trait-driven (`Display`/`Debug`) |

For a C# reader the mental jolt is that `.len()` is bytes and there is no
indexing; for a C reader the relief is that `&str` carries its length and has no
NUL-termination footguns. Both must learn that "number of characters" is a
grapheme question the std library does not answer.

## Common Confusion Points

- **`.len()` is bytes.** `"café".len() == 5`. Use `.chars().count()` for scalar
  count, a grapheme crate for user-perceived characters.
- **No `s[i]`.** Integer indexing is a compile error. Use `chars().nth(i)` (O(n))
  or byte ranges on known boundaries.
- **Byte-range slicing panics off-boundary.** `&s[0..4]` on `"café"` panics; the
  cut splits `é`. Prefer boundary-safe iterators.
- **`chars().count()` != grapheme count.** Emoji families and combining marks are
  multiple `char`s but one grapheme.
- **Paths are not strings.** Use `Path`/`PathBuf`; `.to_str()` is fallible.
- **`Display` vs `Debug`.** `{}` needs `Display` (write it yourself); `{:?}` needs
  `Debug` (derive it). Missing `Display` on your type is a common formatting error.

## Decision Cheat Sheet

| I want to... | Use |
|--------------|-----|
| A function param for read-only text | `&str` |
| Return newly built text | `String` |
| Maybe-borrow, maybe-own | `Cow<str>` |
| Count bytes / storage size | `.len()` |
| Count Unicode scalars | `.chars().count()` |
| Segment user-perceived character clusters | `unicode-segmentation` graphemes |
| Estimate terminal display columns | `unicode-width` plus an explicit terminal policy |
| Iterate characters | `.chars()` (with `.char_indices()` for offsets) |
| Work with raw bytes | `.as_bytes()` / `&[u8]` |
| Build a filesystem path | `Path::join` / `PathBuf` |
| Interpolate/format | `format!` / `println!` with `Display`/`Debug` |
| Cross an FFI boundary | `CStr`/`CString` ([17](17-UNSAFE-RUST-FFI-AND-ABI.md)) |

## Primary Sources

- The Book, Ch. 8.2 (Strings): https://doc.rust-lang.org/book/ch08-02-strings.html
- std::string::String: https://doc.rust-lang.org/std/string/struct.String.html
- std::primitive.str: https://doc.rust-lang.org/std/primitive.str.html
- std::fmt (formatting syntax): https://doc.rust-lang.org/std/fmt/index.html
- std::path::Path / std::ffi::OsStr: https://doc.rust-lang.org/std/path/struct.Path.html

## Related Guides

- Previous: [09-COLLECTIONS-ITERATORS-AND-RANGES.md](09-COLLECTIONS-ITERATORS-AND-RANGES.md)
- Next: [11-ERRORS-RESULT-OPTION-AND-PANIC.md](11-ERRORS-RESULT-OPTION-AND-PANIC.md)
- `char` and scalars: [02-BINDINGS-TYPES-AND-INFERENCE.md](02-BINDINGS-TYPES-AND-INFERENCE.md)
- FFI C strings: [17-UNSAFE-RUST-FFI-AND-ABI.md](17-UNSAFE-RUST-FFI-AND-ABI.md)
