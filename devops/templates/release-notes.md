{NOTES}

## Install

### Brew

```bash
brew install mikegarde/tap/dotenv-cli
```

### Node

```bash
npm i -g @mikegarde/dotenv-cli
```

## Download

Download the appropriate archive for your platform, extract it, and place the `dotenv` binary in your system `PATH`.

| OS | Intel | Arm |
| --- | --- | --- |
{LINK_MATRIX}

### Linux

For most distributions, the `glibc` build is correct. Use the `musl` build for Alpine Linux or fully static environments.

```bash
# Download and extract
curl -L https://github.com/MikeGarde/dotenv-cli/releases/download/{VERSION}/dotenv-cli-{VERSION}-unknown-linux-gnu-x86_64.tar.gz \
  | tar -xz

# Make executable
chmod +x dotenv

# Move into PATH
sudo mv dotenv /usr/local/bin/dotenv
```

### Windows

Download the appropriate archive, extract `dotenv.exe`, and place it in a directory already included in your `PATH`.
