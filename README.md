# Rust coreutils

[GNU Coreutils](https://www.gnu.org/software/coreutils/) reimplementation in Rust.

That's a practice project, where I want to reimplement some of the GNU coreutils
programs from scratch in rust, to have a better graps of the language.

There's an existing finished project available:
https://github.com/uutils/coreutils

For each of the binaries I want to have the more-or-less full compatibility with
the original GNU version.

At the same time, I don't want to have a lot of platform-specific magic, using
only pure-rust tools provided, and trying to avoid platform-specific system calls outside
of standard library as much as possible.

## Current progress:

| Binary                    | Status      | Comments                                           |
| ------------------------- | ----------- | -------------------------------------------------- |
| [base32](./utils/base32/) | + Completed |                                                    |
| [base64](./utils/base64/) | + Completed |                                                    |
| [cat](./utils/cat/)       | + Completed |                                                    |
| [head](./utils/tail/)     | + Completed |                                                    |
| [rev](./utils/rev/)       | + Completed | Part of util-linux, not coreutils                  |
| [tac](./utils/tac/)       | - WIP       |                                                    |
| [tail](./utils/tail/)     | - WIP       | Follow and PID behavior waiting for implementation |

## Repo structure
```
├─ lib/    # common crates
├─ utils/  # specific util/bin code
```