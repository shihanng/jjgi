# `gi`

[Gi (着)](https://en.wikipedia.org/wiki/Brazilian_jiu-jitsu_gi)
is a linter/formatter wrapper for [`jj fix`](https://docs.jj-vcs.dev/latest/cli-reference/#jj-fix).
In [`jj fix`](https://docs.jj-vcs.dev/latest/config/#code-formatting-and-other-file-content-transformations):

> [...] tools run as subprocesses that take file content on standard input
> and return it, with any desired changes, on standard output.

`gi` is a wrapper for such tools (linters/formatters)
that routes their output (stdout, stderr, etc.)
to the appropriate standard I/O streams as expected by `jj fix`.

## Installation

Clone this repository and run `cargo install`.

```sh
git clone https://github.com/shihanng/gi
cargo install --path .
```

## Usage

Use `gi` inside `jj`'s config file.
For example, [`luacheck`](https://github.com/mpeterv/luacheck)
can accept standard input but does not produce the file content
in standard output.
Instead, it outputs the status to standard output.

```sh
cat something.lua | luacheck -
Checking stdin                                    OK

Total: 0 warnings / 0 errors in 1 file
```

This means that when `luacheck` does not report any errors,
`jj fix` will replace the entire file with empty content.

We can set up `luacheck` using `gi` in the `jj` config file
so that it works correctly with `jj fix`:

```toml
[fix.tools.luacheck]
command = [
  "gi",
  "--on-success-stdout=std-in",
  "--",
  "luacheck",
  "-",
]
patterns = ["glob:'**/*.lua'"]
```

The `--on-success-stdout=std-in` flag tells `gi` to use the standard input
as the value of standard output when `luacheck` exits with a success status.
