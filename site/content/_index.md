---
title: "Nupoi"

description: "データセットにNULLがあるかどうかをチェックする"
# 1. To ensure Netlify triggers a build on our exampleSite instance, we need to change a file in the exampleSite directory.
theme_version: '2.8.2'
cascade:
  featured_image: '/images/gohugo-default-sample-hero-image.jpg'
---

[![build](https://github.com/dangishokouta/nupoi/actions/workflows/build.yml/badge.svg)](https://github.com/dangishokouta/nupoi/actions/workflows/build.yml)
[![Coverage Status](https://coveralls.io/repos/github/dangishokouta/nupoi/badge.svg?branch=main)](https://coveralls.io/github/dangishokouta/nupoi?branch=main)
[![License](https://img.shields.io/badge/License-MIT-green)](https://github.com/dangishokouta/nupoi/blob/main/LICENSE)
[![Rust Report Card](https://rust-reportcard.xuri.me/badge/github.com/dangishokouta/nupoi)](https://rust-reportcard.xuri.me/report/github.com/dangishokouta/nupoi)

<img src="https://user-images.githubusercontent.com/90143019/165062158-0bee35a4-c7b8-4797-8568-5b3570137c4f.png" width="320px">


# Description
データセットファイル中のNull値を持つレコードのNull値の数を出力する
# Useage
```
nupoi [OPTION]　<Path>
<Path>
  Path to data file
[OPTION]
    -h, --help             Print help information
    -p, --point <POINT>    check for null
    -V, --version          Print version information
```
## Sample Output
```
nupoi -p　data.csv
index:1,null:2
index:4,null:1
:
:
```

