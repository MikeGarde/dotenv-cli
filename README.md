# dotenv-cli

A simple way to retrieve and update variables from a .env file.

## Install

```shell
npm i -g @mikegarde/dotenv-cli
```

## CLI Usage

Get a value from a .env file:

```shell
dotenv <key>
```

Get a value from a .env.example file:

```shell
dotenv <key> --file .env.example
```

Return a .env file as JSON:

```shell
dotenv --json
```

Return a single value from a .env file as JSON:

```shell
dotenv <key> --json
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

### Setting a Value

Set a value in a .env file:

```shell
dotenv <key> --set <value>
```

Or pipe a value in:

```shell
echo <value> | dotenv <key>
```

This example will 
 - Generate a new RSA key pair and store it in the .env file
 - Utilizing the stored private key it will generate a public key and store it in the .env file

```shell
openssl genpkey -algorithm RSA -outform PEM -pkeyopt rsa_keygen_bits:2048 2>/dev/null | dotenv RSA_KEY
dotenv RSA_KEY --multiline | openssl rsa -pubout 2>/dev/null | dotenv RSA_PUB
```

### Deleting a Value

Delete a value from a .env file:

```shell
dotenv <key> --delete
```
