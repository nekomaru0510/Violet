# Violet

## 概要
Violetは、RISC-Vアーキテクチャ向けのハイパーバイザである。
ハイパーバイザとして動作するアプリケーションを作成することが可能であり、ハイパーバイザに任意の機能を持たせることが可能である。

## 現状可能なこと
* QEMU7.0.0上での動作
* Linux単体での起動

## 対応アーキテクチャ
* RISC-V

## 環境構築方法
VSCode拡張のRemote-Containersを利用することを想定。

※ VSCodeが無くても、Dockerが動作すれば、問題なく動作します。
その場合は、/workspace/Violetとなるように本ディレクトリをマウントするようにしてください。

Dockerを利用しない場合は、Dockerfileを見て環境構築をしてください。

## ビルド方法

```
% cargo make build
```

## 実行方法
```
% cargo make run
```

