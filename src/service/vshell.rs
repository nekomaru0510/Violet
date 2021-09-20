//! Violet Shell Application

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

/*  ライブラリ用トレイト */
use crate::library::traits::std::TraitStd;

/* サービス用トレイト */
use crate::service::TraitService;

/* [todo delete] std以下に配置したい */
use crate::print;
use crate::println;

// Violet Shell
pub struct VShell<T: TraitStd> {
    std: T,
    prompt: String,
    cmds: Vec<Command>,
}

#[derive(Clone)]
struct Command {
    name: String,
    func: fn(),
}

const DEL: u8 = 0x7F;
const ENTER: u8 = 0x0D;
const NULL: u8 = 0x00;
const BACK_SPACE: u8 = 0x08;
const SPACE: u8 = 0x20;
//const CTRL_A:u8 = 0x01;

impl<T> TraitService for VShell<T>
where
    T: TraitStd,
{
    fn run(&mut self) {
        self.exec();
    }
}

impl<T> VShell<T>
where
    T: TraitStd,
{
    pub fn new(std: T) -> Self {
        /* コマンドの登録 */
        let mut vec = Vec::new();
        //vec.push(Command{name: String::from("test"), func:test});
        vec.push(Command{name: String::from("help"), func:help});
        
        VShell {std, prompt: String::from("Violet%"), cmds: vec}
    }

    fn exec(&mut self) {
        self.main_loop();
    }

    fn main_loop(&mut self) {
        loop{
            /* プロンプトの出力 */
            print!(self.std, "{} ", self.prompt);
            
            /* コマンドの実行 */
            let line: String = self.get_line();
            match self.search_cmd(&line) {
                Some(x) => self.execute_cmd(x),
                None => {},
            }
        }
    }

    fn get_line(&mut self) -> String{
        let mut cmd = String::from("");
        let mut c: u8;

        loop {
            /* 入力の受付 */
            c = self.std.getc();
            
            match c {
                ENTER => {
                    print!(self.std, "\n");
                    break cmd;
                },
                NULL => {},
                DEL | BACK_SPACE => {
                    print!(self.std, "{}", BACK_SPACE as char);
                    print!(self.std, "{}", SPACE as char);
                    print!(self.std, "{}", BACK_SPACE as char);
                    cmd.pop();
                },
                _ => {
                    print!(self.std, "{}", c as char);
                    cmd.push(c as char);
                }
            }
        }
    }

    fn search_cmd(&self, name: &String) -> Option<Command> {
        for (i, cmd) in self.cmds.iter().enumerate() {
            if cmd.name == *name {
                return Some(self.cmds[i].clone())
            }
        }
        return None;
    }

    fn execute_cmd(&mut self, cmd: Command) {
        (cmd.func)();
    }
}

pub fn help() {
    //println!(self.std, "Help is Working now ... ");
}

//
// 以下、無法地帯
//
//use table::Table;
//use register::{cpu::RegisterReadWrite/*, register_bitfields*/};
/*
#[no_mangle]
pub fn test() {
    init_interrupt();

    unsafe {
        let res = &mut Table::table();
        println!("mie: {:x}", res.cpu.core.mie.get());
        println!("mip: {:x}", res.cpu.core.mip.get());
        println!("mtime: {:x}", res.io.timer.get());
        println!("mstatus: {:x}", res.cpu.core.mstatus.get());
        
        res.io.timer.enable_interrupt();
        res.io.timer.set_interrupt_time(0x4000000);
        res.cpu.enable_interrupt();

        register_timer_interrupt_handler(timer_handler);

        println!("mtimecmp: {:x}", res.io.timer.timer.read_mtimecmp());
    }

}

pub fn timer_handler() {
    print!("interrupt ok!");
}
*/
/*
use intrusive_collections::container_of;

//extern crate alloc;
use alloc::rc::{Rc, Weak};
extern crate core;
use core::cell::RefCell;
use alloc::vec;

pub fn hoge() {
/*
    let leaf = Rc::new(VTreeNode {
        instance: Inst::new(3),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    */
    let leaf = Rc::new(VTreeNode::new(Inst::new(3)));
    // leafの親 = {:?} Noneになる
    //let inst = leaf.parent.borrow().upgrade().unwrap().value;    
    //println!("leaf parent = {:?}", inst.get()/*.get()*/);

    let branch = Rc::new(VTreeNode::new(Inst::new(5)));

    /*
    let branch = Rc::new(VTreeNode {
        instance: Inst::new(5),
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    */

    // leafを変更して、親へのWeak<Node>参照を与える？
    // Rc::downgradeは、Rc<Node>からWeak<Node>への変換
    // borrow_mutは、RefCellのメソッド。leafは不変なのに、可変参照できてる。
    //println!("{}", leaf.parent.borrow_mut());
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    // upgradeは、Weakのメソッドであり、WeakからRcへ変換する。
    //println!("leaf parent = {:?}", leaf.parent.borrow());
    //println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let inst = leaf.parent.borrow().upgrade().unwrap().instance;  
    println!("leaf parent = {:?}", inst.get()/*.get()*/);
}

#[derive(Copy, Clone)]
struct Inst {
    pub value :i32,
}

impl Inst {
    pub fn new(value: i32) -> Self {
        Inst {value, }
    }
    pub fn get(&self) -> i32 {
        self.value
    }
}

impl VNode for Inst {
    
}

pub trait VNode {
    //fn get_mut(&self);
}

#[derive(Debug)]
struct VTreeNode<T> {
    instance: T,
    parent: RefCell<Weak<VTreeNode<T>>>,
    children: RefCell<Vec<Rc<VTreeNode<T>>>>,
}

impl<T> VTreeNode<T> {
    pub fn new(instance: T) -> Self {
        VTreeNode {instance, parent: RefCell::new(Weak::new()), children: RefCell::new(vec![]),}
    }

    pub fn add_child(self, node: Rc<VTreeNode<T>>) {
        // 
        //*node.parent.borrow_mut() = Rc::downgrade(&Rc::new(self));
        // 親ノード(self)のchildrenに追加
        //self.children.get_mut().push(node);
    }
}
*/
*/