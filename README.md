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
