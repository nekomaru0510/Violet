# Single Linux

## 概要
単体のLinuxが動作するプロジェクト

## 動作環境
QEMU8.0.0 (virt)

## 対応OS
Linux5.17(単体)

## アーキテクチャ
RISC-V (64bit, H-extension)

## 環境構築
VSCode拡張のRemote-Containersを利用することを想定しています。
※ VSCodeが無くても、Dockerが動作すれば、問題なく動作します。

Dockerを利用しない場合は、Dockerfileを見て環境構築をしてください。
下記がインストールされていれば、動作します。
* QEMU(8.0.0)
* Rust
* cargo-make
* riscv-gnu-toolchain (OpenSBIやLinux等をビルドする場合に必要)

### ビルド方法
本プロジェクトのビルド
```
% cargo build
```

## 実行方法
```
% cargo run
```
