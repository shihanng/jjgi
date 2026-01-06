# `jjgi` - Jujutsu Gi

[Jujutsu](https://docs.jj-vcs.dev/latest/) [Gi (着)](https://en.wikipedia.org/wiki/Brazilian_jiu-jitsu_gi)
is a linter/formatter wrapper for [`jj fix`](https://docs.jj-vcs.dev/latest/cli-reference/#jj-fix).
In [`jj fix`](https://docs.jj-vcs.dev/latest/config/#code-formatting-and-other-file-content-transformations):

> [...] tools run as subprocesses that take file content on standard input
> and return it, with any desired changes, on standard output.

`jjgi` is a wrapper for such tools (linters/formatters)
that routes their output (stdout, stderr, etc.)
to the appropriate standard I/O streams as expected by `jj fix`.

## Installation

Clone this repository and run `cargo install`.

```sh
git clone https://github.com/shihanng/jjgi
cargo install --path .
```

## Usage

Use `jjgi` inside `jj`'s config file.
For example, [`luacheck`](https://github.com/mpeterv/luacheck)
can accept standard input but does not produce the file content
in standard output.
Instead, it outputs the status to standard output.

```sh
$ cat something.lua | luacheck -
Checking stdin                                    OK

Total: 0 warnings / 0 errors in 1 file
```

This means that when `luacheck` does not report any errors,
`jj fix` will replace the entire file with empty content.

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

The `--on-success-stdout=stdin` flag tells `jjgi` to use the standard input
as the value of standard output when `luacheck` exits with a success status.
The `--on-success-stderr=stdout` flag tells `jjgi` to use the standard output
from the wrapped command as the value of standard error
when `luacheck` exits with a success status.
This allows us to display status, debug, or log messages
from the wrapped command when running `jj fix`.
An example from `luacheck` would be the following:

```sh
$ jj fix
something.lua:
Checking stdin                                    OK

Total: 0 warnings / 0 errors in 1 file

Fixed 0 commits of 1 checked.
Nothing changed.
```

### --stdin-file

The `--stdin-file` flag stores the standard input into a temporary file
that the wrapped command can refer to using `{stdin_file}`.
This is particularly useful when the command cannot read from standard input.

Here is an example using [sort-lines](https://pypi.org/project/sort-lines/).
When sort-lines exits successfully,
it means that it has formatted `{stdin_file}` in the correct sort order.
We then use the content of `{stdin_file}` as standard output so that
the changes are applied to the actual file.

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
