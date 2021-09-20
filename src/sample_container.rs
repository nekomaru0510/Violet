//! コンテナモジュール

/* コンテナトレイト */
use crate::container::TraitContainer;

/* デバイスドライバ */
use crate::driver::arch::rv32::Processor;
use crate::driver::board::sifive_u::clint_timer::ClintTimer;
use crate::driver::board::sifive_u::uart::Uart;
use crate::driver::board::sifive_u::plic::Plic;

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

pub struct SampleContainer {
    // [todo fix] ここの記述はどんどん増えると予想されるので、解消したい
    vshell: VShell<Std<Serial<Uart>>>,
}

/*
impl<T> SampleContainer<T>
where
    T: TraitService,
    */
impl SampleContainer
{
    
    /* コンテナ内システムの構築 */
    pub fn new() -> Self {
        let cpu = Processor::new(0);
        let timer = ClintTimer::new(0x0200_4000);
        let intc = Plic::new(0x0C00_0000);
        
        cpu.enable_interrupt();
        timer.enable_interrupt();
        timer.set_interrupt_time(0x4000000);

        /* 割込み処理登録 */

        let uart = Uart::new(0x1001_0000);
        let serial = Serial::new(uart);
        let mut std = Std::new(serial);

        println!(std, "Hello I'm {} ver.{}", "Violet", 0.10);

        let mut vshell = VShell::new(std);

        SampleContainer { vshell }
    }

}

/*
impl<T> TraitContainer for SampleContainer<T>
where
    T: TraitService,
    */
impl TraitContainer for SampleContainer
{
    /* 実行 */
    fn run(&mut self) {
        self.vshell.run();
    }
}
