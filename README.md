# Violet

## 概要
Violetは、RISC-Vアーキテクチャ向けのハイパーバイザである。

## 動作環境
QEMU8.0.0 (virt)

## 対応OS
Linux5.17(単体)

## 対応アーキテクチャ
RISC-V (64bit, H-extension)

## 環境構築
VSCode拡張のRemote-Containersを利用することを想定しています。
※ VSCodeが無くても、Dockerが動作すれば、問題なく動作します。

Dockerを利用しない場合は、Dockerfileを見て環境構築をしてください。
下記がインストールされていれば、動作します。
* QEMU(8.0.0)
* Rust
* riscv-gnu-toolchain (OpenSBIやLinux等をビルドする場合に必要)

rustupにより、riscv64bitのツールチェインを追加
```
% rustup target add riscv64imac-unknown-none-elf
```

### ビルド方法
app以下のプロジェクトのREADME.mdを参照してください

## License
This software is released under the MIT License, see LICENSE.txt.


