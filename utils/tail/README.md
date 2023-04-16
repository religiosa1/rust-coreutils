# tail

output the last part of files

This is a reimplementation of GNU coreutils `tail` programm in Rust.

There are several shortcommings as of now.

The program can work in two general modes:
- direct mode, where you just print the required number of last lines or bytes
  to the stdout
- follow mode, where you do the same, but then repeat the procedure when the
  file was changed and can also optionally watch for some PID to finish.

The program works in synchronous mode for the direct mode. The programm switches
to async mode (single-threaded event-loop) for the follow mode, as it involves:
- listening to filesystem notification events on file changes
- polling with the specified interval the required PID

As we don't need much from the async mode besides the interval function, the
small and simple [smol](https://crates.io/crates/smol) runtime was chosen for
that role. Additionaly, the used filesystem notification crate
[notify](https://crates.io/crates/notify) known to have some issues with the
TOKIO async runtime, it's an additional reason not to use that.


## Known shortcommings

The current version contains several known shortcommings, as addressing them
seems to be too much of an effort, and goes beyond the scope of a training, practice
project this crate is.

### -f descriptor flag is not supported
The common use-case of follow is to watch the log files, and it makes more sense
to track them by name anyway for the logrotate cases (hence the special -F flag
in the original GNU implementation). As the `notify` crate doesn't reliably
inform us about the file renames, neither we have a file descriptor available there
(unless I missed something in the crate documentation), this functionality is omited

### ---pid functionality is nix-only
Though it's probably possible to achieve the desired funcitonality on windows too
using the winapi, it isn't included in the current version, so the PID flag
is only abailable on MacOS and Linux (nix versions)