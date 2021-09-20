/*
 * コンテナモジュール
 */

/* デバイスドライバ */
use crate::driver::board::sifive_u::uart::Uart;

/* ドライバ用トレイト */
use crate::driver::traits::serial::TraitSerial;

/* リソース */
use crate::resource::io::serial::Serial;

/* リソース用トレイト */
use crate::resource::traits::tty::TraitTty;

/* サービス */
use crate::service::vshell::VShell;

/* サービス用トレイト */
use crate::service::TraitService;

/* ライブラリ */
use crate::library::std::Std;
/* [todo delete] std以下に配置したい */
use crate::print;
use crate::println;

pub struct Container {
    base: usize,
}

impl Container {
    pub fn new() -> Self {
        /* システムの構築 */
        let uart = Uart::new(0x1001_0000);
        let serial = Serial::new(uart);
        let mut std = Std::new(serial);

        println!(std, "hello I'm {} ver.{}", "Violet", 0.10);

        let mut vshell = VShell::new(std);
        
        /* 実行 */
        vshell.run();

        Container { base: 0x01 }
    }
}
