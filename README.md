# Violet

## 概要
Violetは、RISC-Vアーキテクチャ向けのハイパーバイザである。

## 環境構築

**Step1**
動作させるプロジェクト(projects以下)に格納されているDockerfileで環境を構築する。
VSCode拡張のRemote-Containersを利用する場合は、下記のいずれかを実施
* Dockerfileを本ディレクトリにコピーする
* .devcontainer/devcontainer.josnの"dockerFile"メンバにパスを記載する
※ Dockerを利用しない場合は、Dockerfileを見て環境構築をしてください。

**Step2**
構築した環境でRustのriscv64bitのツールチェインを追加
```
% rustup target add riscv64imac-unknown-none-elf
```

### ビルド方法
app以下のプロジェクトのREADME.mdを参照してください

## License
This software is released under the MIT License, see LICENSE.txt.
