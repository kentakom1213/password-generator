# password-generator

A command-line tool to generate secure random passwords.

By default, `password-generator` creates a 16-character password using lowercase
letters, uppercase letters, and digits.

## Installation

### cargo install

```sh
cargo install --git https://github.com/kentakom1213/password-generator -f
```

### wasm runtime

```sh
curl -L https://github.com/kentakom1213/password-generator/releases/latest/download/password-generator.wasm -o password-generator.wasm
wasmtime password-generator.wasm
```

## Usage

```sh
password-generator [OPTIONS]
```

### Options

| Option                       | Description                                            | Default                                       |
| ---------------------------- | ------------------------------------------------------ | --------------------------------------------- |
| `-n, --length <LENGTH>`      | Set the password length.                               | `16`                                          |
| `-l`                         | Do not use lowercase letters.                          | Enabled by default                            |
| `-u`                         | Do not use uppercase letters.                          | Enabled by default                            |
| `-d`                         | Do not use digits.                                     | Enabled by default                            |
| `-a [ADDITIONAL_CHARACTERS]` | Add characters to the generated character set.         | (If not specified, it defaults to "!@#$%\_-") |
| `-c <CUSTOM_CHARACTERS>`     | Use a custom character set instead of the default set. | None                                          |
| `-x [SEPARATOR]`             | Separate into groups of 4                              | (If not specified, it defaults to "-")        |
| `-h, --help`                 | Print help.                                            | -                                             |
| `-V, --version`              | Print version.                                         | -                                             |

## Examples

Generate a default 16-character password:

```sh
$ password-generator
jmD0rB7ZXzXtAU7n
```

Generate a 32-character password:

```sh
$ password-generator -n 32
OfZS02ZY7a4K418dky9Pj3UF4mTRVGE9
```

Generate a password without digits:

```sh
$ password-generator -d
MNKioSkTiklbXuKB
```

Add extra characters to the default character set:

```sh
$ password-generator -a
ku7IP6#_Fg-Y1NWB
```

To specify the characters you want to add yourself, follow these steps:

```sh
❯ password-generator -a '^&*|<>'
q&P1ejGl>r5R&gGV
```

Use only a custom character set:

```sh
$ password-generator -c '0123456789abcdef'
f4a55215fb762cc5
```

Separate into groups of 4:

```sh
$ password-generator -x
SwW3-l8OV-hkg9-b8tg
```

If all character groups are disabled and no characters are available,
`password-generator` exits with an error:

```sh
$ password-generator -lud
The character set cannot be empty.
```

## Development

Build the project:

```sh
$ cargo build
```

Run the CLI locally:

```sh
$ cargo run -- -n 24
```

Run tests:

```sh
$ cargo test
```
