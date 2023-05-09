# Violet

## 概要
Violetは、RISC-Vアーキテクチャ向けのハイパーバイザである。

## 動作環境
QEMU7.0.0 (virt)

## 対応OS
Linux5.17(単体)

## 対応アーキテクチャ
RISC-V (64bit, H拡張)

## 環境構築方法
VSCode拡張のRemote-Containersを利用することを想定。

※ VSCodeが無くても、Dockerが動作すれば、問題なく動作します。
その場合は、/workspace/Violetとなるように本ディレクトリをマウントするようにしてください。

Dockerを利用しない場合は、Dockerfileを見て環境構築をしてください。

## ビルド方法

OpenSBI、Linux、Busyboxのビルド
```
% cargo make build_another_project
```

Violet本体のビルド
```
% rustup target add riscv64imac-unknown-none-elf
% cargo build
```
※ 以降、`cargo build`のみでビルド可能

## 実行方法
```
% cargo run
```

## License
This software is released under the MIT License, see LICENSE.txt.


