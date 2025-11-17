# minigrep

`minigrep` - a tiny `grep`-like tool for searching lines in text files.

This crate is implements the `minigrep` crate as described by
[_The Rust Programming Language_](https://doc.rust-lang.org/book/) book (chapters 12-13).

The core search logic is included as reusable functions.

## Features

- Simple case-sensitive search via `search`
- Optional case-insensitive search via `search_case_insensitive`

## Command-line usage

The binary expects two positional arguments: a query string and a file path.
Case sensitivity is controlled via the `IGNORE_CASE` environment variable.

```bash
IGNORE_CASE=1 minigrep to poem.txt
```

By default, search is case-sensitive:

```bash
minigrep to poem.txt
```
