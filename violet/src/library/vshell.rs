//! Violet Shell Application

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::library::std::getc;
use crate::print;
use crate::println;

// Violet Shell
pub struct VShell {
    prompt: String,
    cmds: Vec<Command>,
}

#[derive(Clone)]
pub struct Command {
    pub name: String,
    pub func: fn(),
}

const DEL: u8 = 0x7F;
const ENTER: u8 = 0x0D;
const NULL: u8 = 0x00;
const BACK_SPACE: u8 = 0x08;
const SPACE: u8 = 0x20;
//const CTRL_A:u8 = 0x01;

impl VShell {
    pub fn new() -> Self {
        /* Register command */
        let mut vec = Vec::new();
        vec.push(Command {
            name: String::from("help"),
            func: help,
        });

        VShell {
            prompt: String::from("Violet%"),
            cmds: vec,
        }
    }

    pub fn run(&mut self) {
        self.exec();
    }

    pub fn add_cmd(&mut self, command: Command) {
        self.cmds.push(command);
    }

    fn exec(&mut self) {
        self.main_loop();
    }

    fn main_loop(&mut self) {
        loop {
            /* output prompt */
            print!("{} ", self.prompt);

            /* Run command */
            let line: String = self.get_line();
            match self.search_cmd(&line) {
                Some(x) => self.execute_cmd(x),
                None => {
                    if &line == "exit" {
                        break;
                    } else if &line != "" {
                        println!("Command not found: {}", line);
                    }
                }
            }
        }
    }

    fn get_line(&mut self) -> String {
        let mut cmd = String::from("");
        let mut c: u8;

        loop {
            c = getc();

            match c {
                ENTER => {
                    print!("\n");
                    break cmd;
                }
                NULL => {}
                DEL | BACK_SPACE => {
                    print!("{}", BACK_SPACE as char);
                    print!("{}", SPACE as char);
                    print!("{}", BACK_SPACE as char);
                    cmd.pop();
                }
                _ => {
                    print!("{}", c as char);
                    cmd.push(c as char);
                }
            }
        }
    }

    fn search_cmd(&self, name: &String) -> Option<Command> {
        for (i, cmd) in self.cmds.iter().enumerate() {
            if cmd.name == *name {
                return Some(self.cmds[i].clone());
            }
        }
        return None;
    }

    fn execute_cmd(&mut self, cmd: Command) {
        (cmd.func)();
    }
}

pub fn help() {
    println!("Help is Working now ... ");
}
