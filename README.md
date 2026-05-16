# dotenv-cli
[![Version](https://img.shields.io/github/v/release/mikegarde/dotenv-cli)](https://github.com/MikeGarde/dotenv-cli)
[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/mikegarde/dotenv-cli/test.rust.yml)](https://github.com/MikeGarde/dotenv-cli/actions)
[![Codecov](https://img.shields.io/codecov/c/github/mikegarde/dotenv-cli)](https://app.codecov.io/gh/MikeGarde/dotenv-cli)
[![NPM Downloads](https://img.shields.io/npm/dy/%40mikegarde%2Fdotenv-cli?logo=npm&color=blue)](https://www.npmjs.com/package/@mikegarde/dotenv-cli)
[![Crates.io Downloads](https://img.shields.io/crates/d/dotenv-cli?logo=crates&color=blue)](https://crates.io/crates/dotenv-cli)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/mikegarde/dotenv-cli/total?logo=github&color=blue)](https://github.com/MikeGarde/dotenv-cli/releases)

A simple way to retrieve, update, or delete .env variables directly from the command line.

## Install

Find it on
[GitHub](https://github.com/MikeGarde/dotenv-cli),
[crates.io](https://crates.io/crates/dotenv-cli), or
[npm](https://www.npmjs.com/package/@mikegarde/dotenv-cli),

```bash
# Using Homebrew (macOS/Linux)
brew install mikegarde/tap/dotenv-cli

# Using npm (Node.js)
npm i -g @mikegarde/dotenv-cli

# Using Cargo (Rust)
cargo install dotenv-cli
```

## Usage

Get a value from a .env file:

```shell
dotenv <key>
```

Get a value from a .env.example file:

```shell
dotenv <key> --file .env.example
```

### Setting a Value

Set a value in a .env file:

```shell
dotenv <key> --set <value>
```

Or pipe a value in:

```shell
echo <value> | dotenv <key>
```

### Deleting a Value

Delete a value from a .env file:

```shell
dotenv <key> --delete
```

## Examples

### RSA Key Pair

1. **Private Key:** Generate a new key using the `openssl` command. The private key is then stored in the .env file under the variable `RSA_KEY`.
2. **Public Key** The `dotenv` command, with the `--multiline` flag, retrieves the stored private key and pipes it back to openssl. `openssl` then generates a corresponding public key. This public key is stored in the `.env` file under the variable `RSA_PUB`.

```shell
openssl genpkey -algorithm RSA -outform PEM -pkeyopt rsa_keygen_bits:2048 2>/dev/null | dotenv RSA_KEY
dotenv RSA_KEY -m | openssl rsa -pubout 2>/dev/null | dotenv RSA_PUB
```

### App Version

This demonstrates two methods for updating the `APP_VERSION` in your `.env` file. The `sed` command is versatile and powerful, allowing for complex text manipulations. On the other hand, `dotenv` provides a more readable and straightforward syntax.

```shell
NEW_VERSION=3.22.1

# Using sed
sed -i "s/^APP_VERSION=.*$/APP_VERSION=$NEW_VERSION/" .env

# Using dotenv
dotenv APP_VERSION --set $NEW_VERSION
```

### JSON Output

Make it pretty with `jq`:

```shell
dotenv | jq
```

Or filter the output:

```shell
$ dotenv | jq 'to_entries | map(select(.key | startswith("DB_")))[] | "\(.key)=\(.value)"'
"DB_HOST=localhost"
"DB_USER=root"
"DB_PASS=password"
```

## Other Stuff

### JSON

By default multiple keys are returned as a JSON object. To return a single key as a JSON object, use the `--json` flag.
To not return a JSON object, use the `--no-json` flag.

Return a .env file as JSON:

```shell
dotenv
```

Wildcard search:

```shell
dotenv "DB_*"
```

### Multiline Values

The default behavior is to output a single line value. If you want to output a multiline value,
you can use the `--multiline` flag:

```shell
$ dotenv RSA_KEY
-----BEGIN RSA PRIVATE KEY-----\nMIIBOgIBAAJBAKj34GkxFhD90vcNLYLInFEX6Ppy1tPf...

$ dotenv RSA_KEY --multiline
-----BEGIN RSA PRIVATE KEY-----
MIIBOgIBAAJBAKj34GkxFhD90vcNLYLInFEX6Ppy1tPf9Cnzj4p4WGeKLs1Pt8Qu
KUpRKfFLfRYC9AIKjbJTWit+CqvjWYzvQwECAwEAAQJAIJLixBy2qpFoS4DSmoEm
```

### Using DOTENV_FILE Environment Variable

You can define the `DOTENV_FILE` environment variable in your shell or script to specify the `.env` file to use, instead
of passing the `--file` option every time.

```shell
export DOTENV_FILE=.env.example
dotenv <key>
```

This will use the `.env.example` file automatically. If the `--file` option is provided, it will override the
`DOTENV_FILE` environment variable.
