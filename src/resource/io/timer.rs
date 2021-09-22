//! Timerリソース

/* リソース用トレイト */
use crate::resource::traits::timer::TraitTimerRs;

/* ドライバ用トレイト */
use crate::driver::traits::timer::TraitTimer;

#[derive(Clone)]
pub struct Timer<T: TraitTimer> {
    timer: T,
}

impl<T> TraitTimerRs for Timer<T>
where
    T: TraitTimer,
{
    #[allow(dead_code)]
    fn write(&self, t:u64) {
        self.timer.write(t);
    }

    fn read(&self) -> u64 {
        self.timer.read()
    }

    fn enable_interrupt(&self) {
        self.timer.enable_interrupt();
    }

    fn disable_interrupt(&self) {
        self.timer.disable_interrupt();
    }

    fn set_interrupt_time(&self, t: u64) {
        self.timer.set_interrupt_time(t);
    }
}

impl<T> Timer<T>
where
    T: TraitTimer,
{
    pub fn new(timer: T) -> Self {
        Timer {timer,}
    }
/*
    pub fn register(&mut self, drv: &mut Uart) {
        self.uart = drv;
    }
*/
}


