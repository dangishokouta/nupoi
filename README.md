# nupoi

[![build](https://github.com/dangishokouta/nupoi/actions/workflows/build.yml/badge.svg)](https://github.com/dangishokouta/nupoi/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/dangishokouta/nupoi/badge.svg)](https://coveralls.io/github/dangishokouta/nupoi)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/dangishokouta/nupoi)](https://rust-reportcard.xuri.me/report/github.com/dangishokouta/nupoi)

<img src="https://user-images.githubusercontent.com/90143019/165062158-0bee35a4-c7b8-4797-8568-5b3570137c4f.png" width="320px">

Check for null in the dataset

# Description
Outputs records with null values and the number of nulls in the dataset file

# Useage
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
$ nupoi test.csv
  index: 2  null: 1
  index: 4  null: 1
  index: 6  null: 2
  index: 7  null: 3

