# Problem

Supply chain attacks became very common thing these days, but we're still running untrusted code on our machines everyday. This crate provides `cargo safe` subcommand, that runs all commands in a sandboxed environment.

For now it is working on macOS only using Apple's sandboxing mechanism.

# How to use it?

## Installation

```console
$ cargo install cargo-safe
```

Using is pretty simple, you can use any `cargo` command:

```console
$ cargo safe buld
$ cargo safe test
$ cargo safe run
```

Or any other cargo command.

## What is allowed inside sandoxed environment

- reading and writing OS temporary directory
- outbound network activity to ports 80/443
- listing of files (but not their content)
- reading all files listed in `PATH` env variable (to run system binaries)
- writing to a separate `cargo` and `target` directiories in a private sandbox
- writing to `/dev/null` and some other system files.
- reading from `/dev/random`, `/dev/tty` and some other system files

Full list of permissions can be found in sources.
