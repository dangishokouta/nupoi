---
title: ":runner: Usage"
---

## CLI

```sh
btmeister 0.1.2
Kota Dangisho
Check for null in the dataset
USAGE:
    nupoi [OPTION]　<File>
ARGS:
  <File> Path to data file
OPTIONS:
    -h, --help                       Print help information
    -V, --version                    Print version information
```
### Sample Output
```sh
$ btmeister . ~/go/src/github.com/tamada/rrh
cargo       ./Cargo.toml
make        /Users/tamada/go/src/github.com/tamada/rrh/Makefile
$ nupoi test.csv
  index: 2  null: 1
  index: 4  null: 1
  index: 6  null: 2
  index: 7  null: 3

```
## :whale: Docker
```sh
docker run --rm -it -v $PWD:/home/nupoi ghcr.io/dangishokouta/nupoi:latest
```
The working directory in the docker container is `/home/nupoi`.
The target project should be on the directory with `-v` flag of docker.
### Available versions
* `0.1.2`, `latest`
