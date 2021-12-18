//! ハイパーバイザコンテナモジュール

/* コンテナトレイト */
use crate::container::TraitContainer;

/* デバイスドライバ */
//use crate::driver::arch::rv32::Processor;
use crate::driver::arch::rv64::Processor;
use crate::driver::board::sifive_u::clint_timer::ClintTimer;
use crate::driver::board::sifive_u::uart::Uart;
use crate::driver::board::sifive_u::plic::Plic;

/* ドライバ用トレイト */
use crate::driver::traits::serial::TraitSerial;

/* リソース */
use crate::resource::io::serial::Serial;
use crate::resource::io::timer::Timer;

/* リソース用トレイト */
use crate::resource::traits::tty::TraitTty;
use crate::resource::traits::timer::TraitTimerRs;

/* サービス */
use crate::service::vshell::VShell;
use crate::service::sched::Scheduler;

/* サービス用トレイト */
use crate::service::TraitService;

/* ライブラリ */
use crate::library::std::Std;

/* [todo delete] std以下に配置したい */
use crate::print;
use crate::println;

/* [todo delete] 割込み用 */
use crate::Context;

pub struct HypervisorContainer {
    // [todo fix] ここの記述はどんどん増えると予想されるので、解消したい
    srv: VShell<Std<Serial<Uart>, Timer<ClintTimer>>>,
    //srv: Scheduler<Std<Serial<Uart>, Timer<ClintTimer>>>,
}

/*
impl<T> SampleContainer<T>
where
    T: TraitService,
    */
impl HypervisorContainer
{
    
    /* コンテナ内システムの構築 */
    pub fn new() -> Self {
        let cpu = Processor::new(0);
        let ctimer = ClintTimer::new(0x0200_4000);
        let intc = Plic::new(0x0c00_0000);

        let timer = Timer::new(ctimer);
        
        //cpu.enable_interrupt();

        let uart = Uart::new(0x1000_0000);
        let serial = Serial::new(uart);
        let mut std = Std::new(serial, timer);

        println!(std, "Hello I'm {} ", "Violet");
        
        let mut srv = VShell::new(std);
        //let mut srv = Scheduler::new(std);
        
        HypervisorContainer { srv }
    }

    /* 割込みハンドラ */
    pub fn interrupt(&mut self, cont: &mut Context) {
        /* 各種サービスへ割込みの振分け */
        self.srv.interrupt(cont);
    }

}

/*
impl<T> TraitContainer for SampleContainer<T>
where
    T: TraitService,
    */
impl TraitContainer for HypervisorContainer
{
    /* 実行 */
    fn run(&mut self) {
        self.srv.run();
    }
}
