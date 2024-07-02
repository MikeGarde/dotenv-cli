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
$ dotenv RSA_KEY --multiline
-----BEGIN RSA PRIVATE KEY-----
MIIBOgIBAAJBAKj34GkxFhD90vcNLYLInFEX6Ppy1tPf9Cnzj4p4WGeKLs1Pt8Qu
KUpRKfFLfRYC9AIKjbJTWit+CqvjWYzvQwECAwEAAQJAIJLixBy2qpFoS4DSmoEm


$ dotenv RSA_KEY
-----BEGIN RSA PRIVATE KEY-----\nMIIBOgIBAAJBAKj34GkxFhD90vcNLYLInFE...
```

### Setting a Value

Set a value in a .env file:

```shell
dotenv <key> --set <value>
```

Quotes will be added if needed, but you can also force them:

```shell
dotenv <key> --set <value> --quote
```
