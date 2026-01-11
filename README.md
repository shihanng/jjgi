# `jjgi` - Jujutsu Gi

[Jujutsu](https://docs.jj-vcs.dev/latest/) [Gi (着)](https://en.wikipedia.org/wiki/Brazilian_jiu-jitsu_gi)
is a linter/formatter wrapper for [`jj fix`](https://docs.jj-vcs.dev/latest/cli-reference/#jj-fix).

## Motivation

[`jj fix`](https://docs.jj-vcs.dev/latest/config/#code-formatting-and-other-file-content-transformations)
requires tools to follow a specific convention:

> [...] tools run as subprocesses that take file content on standard input
> and return it, with any desired changes, on standard output.

However, many linters and formatters do not work this way. They may:

- Output status messages to standard output instead of the file content
- Write results to a file instead of standard output
- Not accept standard input at all

`jjgi` is a wrapper that adapts these tools to work with `jj fix`
by routing their input/output streams appropriately.

## Installation

### GitHub release

Pre-built binaries are available
in the [GitHub releases](https://github.com/shihanng/jjgi/releases).
Download and install manually, or use tools like
[`mise`](https://mise.jdx.dev/):

```sh
mise install github:shihanng/jjgi@latest
```

### Homebrew

```sh
brew install shihanng/jjgi/jjgi
```

### Cargo

```sh
cargo install jjgi
```

### From source

Clone this repository and run `cargo install`:

```sh
git clone https://github.com/shihanng/jjgi
cargo install --path .
```

## Usage

Use `jjgi` inside `jj`'s config file. Run `jjgi --help` to see all available flags.

### The problem

For example, [`luacheck`](https://github.com/mpeterv/luacheck)
can accept standard input but does not produce the file content
in standard output.
Instead, it outputs the status to standard output:

```sh
$ cat something.lua | luacheck -
Checking stdin                                    OK

Total: 0 warnings / 0 errors in 1 file
```

This means that when `luacheck` does not report any errors,
`jj fix` will replace the entire file with empty content
(because `jj fix` expects the file content on standard output).

### --on-success-stdout/stderr

We can set up `luacheck` using `jjgi` in the `jj` config file
so that it works correctly with `jj fix`:

```toml
[fix.tools.luacheck]
command = [
  "jjgi",
  "--on-success-stdout=stdin",
  "--on-success-stderr=stdout",
  "--",
  "luacheck",
  "-",
]
patterns = ["glob:'**/*.lua'"]
```

**How it works:**

- `--on-success-stdout=stdin` - When `luacheck` exits successfully,
  `jjgi` outputs the original standard input to standard output.
  This ensures `jj fix` receives and uses the file content.
- `--on-success-stderr=stdout` - When `luacheck` exits successfully,
  `jjgi` routes the command's standard output to standard error.
  This allows status messages to be displayed to the user.
- `--` - Separates `jjgi` flags from the wrapped command.

**When the tool succeeds:**

```sh
$ jj fix
something.lua:
Checking stdin                                    OK

Total: 0 warnings / 0 errors in 1 file

Fixed 0 commits of 1 checked.
Nothing changed.
```

The status messages appear because they're routed to standard error,
and the original file content is preserved because standard input
is routed to standard output.

### --stdin-file

The `--stdin-file` flag stores the standard input into a temporary file
that the wrapped command can refer to using `{stdin_file}`.
This is useful when the command cannot read from standard input.

Here is an example using [sort-lines](https://pypi.org/project/sort-lines/).
When `sort-lines` exits successfully,
it has formatted `{stdin_file}` in the correct sort order.
We then use the content of `{stdin_file}` as standard output
so that the changes are applied to the actual file.

```toml
[fix.tools.sort-lines]
command = [
  "jjgi",
  "--stdin-file",
  "--on-success-stdout=stdin-file",
  "--",
  "sort-lines",
  "{stdin_file}",
]
patterns = ["glob:'**/*'"]
```

### --on-failure-stderr

By default, `jjgi` uses the standard error of the wrapper command
as its standard error when the command exits with an error code.
However, some commands output status or error details to standard output.
Using `--on-failure-stderr=stdout`,
we can route the standard output of the command
to the standard error of `jjgi`,
allowing us to see the error details when `jj fix` detects a failure.

## Development

### How to release `jjgi`

Use `cargo release` to simultaneously push the release commit to the `main` branch
on GitHub and publish the package to [crates.io](https://crates.io/crates/jjgi).
