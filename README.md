# Violet

## 概要
Violet(VIrtual machine monitor tO LET resources)は、
小型のハイパーバイザ(にしたいもの)である。
Rustの所有権システムを用いてゲストOSに資源(の所有権)を貸し、
動作~~させる~~(させたいと思っている)。
(現在開発途中)

## 現状可能なこと
* QEMU5.0.0上での動作(ターゲットは、sifive-u)
* 一部のCSRを操作
* シェルもどきによる対話的操作
* タイマ割込みの利用

## 対応アーキテクチャ
* RISC-V(rv32imac, rv64gc)

## 環境構築方法

### Rustのインストール

いつもの
```
% curl https://sh.rustup.rs -sSf | sh
```

### QEMUのインストール

prefixは、任意のディレクトリを指定
```
% wget https://download.qemu.org/qemu-5.0.0.tar.xz
% tar xvJf qemu-5.0.0.tar.xz
% cd qemu-5.0.0
% ./configure --target-list=riscv32-softmmu --prefix=${HOME}/local/
% make -j 2
% make install
```

### リンカのインストール
準備
```
% sudo apt-get install libtool-bin libncurses5-dev
```

crosstool-ngのインストール
```
cd /tmp
curl -L http://crosstool-ng.org/download/crosstool-ng/crosstool-ng-1.24.0.tar.xz | tar Jx
cd crosstool-ng-1.24.0
./configure
make -j$(nproc)
sudo make install
```

ターゲットを確認
```
ct-ng list-samples | grep riscv                                                          
```

ターゲットの選択
```
ct-ng riscv32-unknown-elf
```

menuconfigからprefixの指定(任意)
```
ct-ng menuconfig
```

クロスツールのビルド
```
ct-ng build
```

### クロスツール(rustup)のインストール・設定

クロスコンパイラのインストール
```
% rustup target add riscv32imac-unknown-none-elf
```

nightlyバージョンの指定
```
$ rustup override set nightly
```

## ビルド方法

```
% cargo build
```

## 実行方法
```
% cargo run
```

## メモ

ディスアセンブル(マングリング解除+疑似命令無効)
```
$ riscv64-unknown-elf-objdump --demangle --disassembler-options="no-aliases" -D target/riscv32i-unknown-none-elf/debug/main | less
```
