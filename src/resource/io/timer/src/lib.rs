//use crate::kernel::driver::board::sifive_u::clint_timer::ClintTimer;
#![no_std]
use clint_timer::ClintTimer;
use class::timer::TimerAttr;

pub struct Timer
{
    pub timer: ClintTimer,
}


impl Timer 
{
    pub fn new() -> Self {
        Timer {timer: ClintTimer::new(),}
    }
/*
    pub fn register(&mut self, drv: &mut Uart) {
        self.uart = drv;
    }
*/
    #[allow(dead_code)]
    pub fn set(&self, t:u64) {
        self.timer.write(t);
    }

    pub fn get(&self) -> u64 {
        self.timer.read()
    }

    pub fn enable_interrupt(&self) {
        self.timer.enable_interrupt();
    }

    pub fn disable_interrupt(&self) {
        self.timer.disable_interrupt();
    }

    pub fn set_interrupt_time(&self, t: u64) {
        self.timer.set_interrupt_time(t);
    }

}


