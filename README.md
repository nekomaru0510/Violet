# Violet

## 概要
Violetは、RISC-Vアーキテクチャ向けのハイパーバイザである。

## 動作環境
QEMU8.0.0 (virt)

## 対応OS
Linux5.17(単体)

## 対応アーキテクチャ
RISC-V (64bit, H拡張)

## 環境構築
VSCode拡張のRemote-Containersを利用することを想定しています。
※ VSCodeが無くても、Dockerが動作すれば、問題なく動作します。

Dockerを利用しない場合は、Dockerfileを見て環境構築をしてください。
下記がインストールされていれば、動作します。
* riscv-gnu-toolchain
* QEMU(8.0.0)
* Rust
* cargo-make

### ビルド方法
OpenSBI、Linux、Busyboxのインストール・ビルド
```
% cargo make install_another_project
% cargo make build_another_project
```

rustupにより、riscv64bitのツールチェインを追加
```
% rustup target add riscv64imac-unknown-none-elf
```

Violet本体のビルド
```
% cargo build
```

## 実行方法
```
% cargo run
```

## License
This software is released under the MIT License, see LICENSE.txt.


