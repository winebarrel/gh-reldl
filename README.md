# gh-reldl

CLI to download assets from GitHub release.

## Usage

```
Usage: gh-reldl URL [FILE]

Options:
    -v, --version       Print version and exit
    -h, --help          Print usage and exit
```

```sh
# NOTE: Required for download from private repository
# export GITHUB_TOKEN=...

gh-reldl https://github.com/cli/cli/releases/download/v2.2.0/gh_2.2.0_linux_amd64.tar.gz
```

## Installation

```sh
curl -sSfL https://github.com/winebarrel/gh-reldl/releases/download/v0.1.0/gh-reldl_v0.1.0_x86_64-apple-darwin > gh-reldl
chmod +x gh-reldl
./gh-reldl --help
```
