# devstuff - A collection of useful development tools

## Install

Install or update using `cargo`
```bash
cargo install devstuff
```

## Usage
```bash
# input is a file
devstuff [category] [tool name] [file name]

# input is a raw text
devstuff [category] [tool name] [input] --raw

# or input is piped
echo "" | devstuff [category] [tool name]

```

## Available tools
```
devstuff 0.0.1
A collection of development tools

USAGE:
    devstuff <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    b64     Base64 Encoding and Decoding
    hash    Popular hash functions (Blake3, SHA1, SHA256, SHA512)
    help    Print this message or the help of the given subcommand(s)
    html    Minify or unminify html
    json    Minify or unminify json
    uuid    Generate an UUID

```