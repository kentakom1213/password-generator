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

| Option | Description | Default |
| --- | --- | --- |
| `-n, --length <LENGTH>` | Set the password length. | `16` |
| `-l` | Do not use lowercase letters. | Enabled by default |
| `-u` | Do not use uppercase letters. | Enabled by default |
| `-d` | Do not use digits. | Enabled by default |
| `-s` | Use symbols: `!@#$%_-`. | Disabled by default |
| `-a <ADDITIONAL_CHARACTERS>` | Add characters to the generated character set. | None |
| `-c <CUSTOM_CHARACTERS>` | Use a custom character set instead of the default set. | None |
| `-h, --help` | Print help. | - |
| `-V, --version` | Print version. | - |

## Examples

Generate a default 16-character password:

```sh
password-generator
```

Generate a 32-character password:

```sh
password-generator --length 32
```

Generate a password with symbols:

```sh
password-generator -s
```

Generate a password without digits:

```sh
password-generator -d
```

Add extra characters to the default character set:

```sh
password-generator -a '^&*'
```

Use only a custom character set:

```sh
password-generator -c 'abcdef012345'
```

If all character groups are disabled and no characters are available,
`password-generator` exits with an error:

```sh
password-generator -l -u -d
```

## Development

Build the project:

```sh
cargo build
```

Run the CLI locally:

```sh
cargo run -- --length 24 -s
```

Run tests:

```sh
cargo test
```
