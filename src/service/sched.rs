//! Simple Scheduler

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use core::ptr::{read_volatile, write_volatile};

/*  ライブラリ用トレイト */
use crate::library::traits::std::TraitStd;

/* サービス用トレイト */
use crate::service::TraitService;

/* [todo delete] std以下に配置したい */
use crate::print;
use crate::println;

/* [todo delete] 割込み用 */
use crate::Context;

// Scheduler
pub struct Scheduler<T: TraitStd + core::clone::Clone> {
    std: T,
    current: u32,
    tsks: Vec<Task<T>>,
}

#[derive(Clone)]
struct Task<T> where T: TraitStd, T: core::clone::Clone {
    id: u32,
    state: u8,
    name: String,
    context: Context,
    func: fn(T),
}

const STATE_IDLE: u8 = 0x00;
const STATE_RUNNABLE: u8 = 0x01;
const STATE_RUN: u8 = 0x02;
const STATE_WAIT: u8 = 0x03;

impl<T> TraitService for Scheduler<T>
where
    T: TraitStd + core::clone::Clone,
{
    /* 実行 */
    fn run(&mut self) {
        self.std.set_alerm(60000000);
        count(self.std.clone());
    }

    /* 割込みハンドラ */
    fn interrupt(&mut self, cont: &mut Context) {
        //print!(self.std, "Interrupt OK! {}", cont.sp as usize);

        print!(self.std, "interrupt! {}", self.std.gettime());
        //for n in 1..1000000 {}
        //時刻の設定
        let time = self.std.gettime() + 50000000;
        
        self.std.set_alerm(time);
        //self.std.set_alerm(0xffffffff);
        println!(self.std, " {} -> {}", self.std.gettime(), time);
        return ;
        
        //コンテキストの保存 
        self.tsks[self.current as usize].context = cont.clone();
        // [todo fix] アーキテクチャ依存なので、CPUドライバ内で行う
        for i in 0..self.tsks[self.current as usize].context.regsize {
            unsafe { self.tsks[self.current as usize].context.regs[i as usize] = read_volatile(self.tsks[self.current as usize].context.sp.offset(4)) }
        }
        
        //ランキューの更新
        //次タスクを決定
        self.current = self.current ^ 1;

        //次タスクのコンテキストの設定
        if (self.tsks[self.current as usize].state == STATE_IDLE) {
            //self.tsks[self.current as usize].state = RUN;
            //(self.tsks[self.current as usize].func)(self.std.clone());
        }
        else {
            //cont = *self.tsks[self.current as usize].context.clone();
            //write_volatile((self.tsks[self.current as usize].sp) as *mut u8, c);

            for i in 0..self.tsks[self.current as usize].context.regsize {
                unsafe {
                    write_volatile(self.tsks[self.current as usize].context.sp.offset(4), 
                    self.tsks[self.current as usize].context.regs[i as usize]);
                }
            }
        }
        
    }
}

impl<T> Scheduler<T>
where
    T: TraitStd + core::clone::Clone,
{
    pub fn new(std: T) -> Self {
        /* コマンドの登録 */
        let mut vec = Vec::new();
        vec.push(Task{id: 0, state: STATE_IDLE, name: String::from("idle"), context: Context::new(), func: idle});
        vec.push(Task{id: 1, state: STATE_IDLE, name: String::from("count"), context: Context::new(), func: count});
        
        //println!(std, "interrupt init {}", std.gettime());
        //std.set_alerm(0x4000000);
        //std.set_alerm(std.gettime() + 0x1000000);

        Scheduler {std, current: 0, tsks: vec}
    }

    fn start_task(&mut self, tsk: &mut Task<T>) {
        /* [todo fix] clone使うのはちょっとダサい？ */
        (tsk.func)(self.std.clone());
    }

}

pub fn idle<T:TraitStd>(mut std:T) {
    loop{}
}

pub fn count<T:TraitStd>(mut std:T) {
    let mut c: u32 = 0;
    loop{
        for n in 1..1000000 {}
        println!(std, "count: {} {}", c, std.gettime());
        c = c + 1;
    }
}
