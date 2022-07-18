# nupoi

[![build_products](https://github.com/dangishokouta/nupoi/actions/workflows/build.yaml/badge.svg)](https://github.com/dangishokouta/nupoi/actions/workflows/build.yaml)
[![Coverage Status](https://coveralls.io/repos/github/dangishokouta/nupoi/badge.svg)](https://coveralls.io/github/dangishokouta/nupoi)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/dangishokouta/nupoi)](https://rust-reportcard.xuri.me/report/github.com/dangishokouta/nupoi)
[![License](https://img.shields.io/badge/License-MIT-green)](https://github.com/dangishokouta/nupoi/blob/main/LICENSE)
[![DOI](https://zenodo.org/badge/483072806.svg)](https://zenodo.org/badge/latestdoi/483072806)


<img src="https://user-images.githubusercontent.com/90143019/165062158-0bee35a4-c7b8-4797-8568-5b3570137c4f.png" width="320px">

データセットファイル中のNull値を持つレコードのNull値の数を出力する

# Description
データセットファイル中のnull値を持つレコードに注目することで不要なレコードかどうかを判断する。

# Useage
```sh
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
      
```
