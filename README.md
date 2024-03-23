# cb (x11-clipboard-rs)

[![test](https://github.com/MITSUBOSHI/x11-clipboard-rs/actions/workflows/test.yml/badge.svg)](https://github.com/MITSUBOSHI/x11-clipboard-rs/actions/workflows/test.yml)

cb (x11-clipboard-rs) is a command line tool such as `pbcopy` and `pbpaste` in MacOS.

## Usage
### Dependency to use
- [xsel](https://github.com/kfish/xsel)

### cb help

```sh
X11 cliboard copy and paste command line tool

Usage: cb <COMMAND>

Commands:
  copy   Copy data from STDIN or command argument to X11 clipboard
  paste  Paste data from X11 clipboard to STDOUT
  help   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### cb copy
Copy data from STDIN or command argument to X11 clipboard.

```sh
# from STDIN
echo "Hello, new user to cb" | cb copy

# from command argument
cb copy "Hello, new user to cb"
```

### cb paste
Paste data from X11 clipboard to STDOUT.

```sh
cb paste > /tmp/dump.txt
```

## License

This tool is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).