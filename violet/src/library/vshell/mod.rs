// SPDX-License-Identifier: MIT 
// SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Violet Shell Application
//!
//! This module implements a simple shell for the Violet hypervisor, allowing users to register and execute commands interactively.

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::library::std::getc;
use crate::print;
use crate::println;

/// Violet Shell structure
///
/// Holds the prompt string and a list of registered commands.
pub struct VShell {
    prompt: String,
    cmds: Vec<Command>,
}

/// Command structure for the shell
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Command {
    /// Command name (as typed by the user)
    pub name: String,
    /// Function pointer to the command implementation
    pub func: fn(),
}

const DEL: u8 = 0x7F;
const ENTER: u8 = 0x0D;
const NULL: u8 = 0x00;
const BACK_SPACE: u8 = 0x08;
const SPACE: u8 = 0x20;

impl VShell {
    /// Create a new shell instance with the default prompt and built-in commands.
    pub fn new() -> Self {
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

    /// Run the shell main loop.
    pub fn run(&mut self) {
        self.exec();
    }

    /// Add a new command to the shell.
    pub fn add_cmd(&mut self, command: Command) {
        self.cmds.push(command);
    }

    /// Execute the shell main loop.
    fn exec(&mut self) {
        self.main_loop();
    }

    /// Main input loop: prompt, read, and execute commands.
    fn main_loop(&mut self) {
        loop {
            print!("{} ", self.prompt);
            let line: String = self.get_line();
            match self.search_cmd(&line) {
                Some(x) => self.execute_cmd(x),
                None => {
                    if &line == "exit" {
                        break;
                    } else if !line.is_empty() {
                        println!("Command not found: {}", line);
                    }
                }
            }
        }
    }

    /// Read a line of input from the user, handling backspace and enter.
    fn get_line(&mut self) -> String {
        let mut cmd = String::new();
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
                    if !cmd.is_empty() {
                        print!("{}{}{}", BACK_SPACE as char, SPACE as char, BACK_SPACE as char);
                        cmd.pop();
                    }
                }
                _ => {
                    print!("{}", c as char);
                    cmd.push(c as char);
                }
            }
        }
    }

    /// Search for a command by name.
    fn search_cmd(&self, name: &str) -> Option<Command> {
        for cmd in &self.cmds {
            if cmd.name == name {
                return Some(cmd.clone());
            }
        }
        None
    }

    /// Execute a command.
    fn execute_cmd(&mut self, cmd: Command) {
        (cmd.func)();
    }
}

/// Built-in help command.
pub fn help() {
    println!("Help is Working now ... ");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_cmd() {}

    #[test_case]
    fn test_add_and_search_cmd() -> Result<(), &'static str> {
        let mut shell = VShell::new();
        let cmd = Command {
            name: String::from("dummy"),
            func: dummy_cmd,
        };
        shell.add_cmd(cmd.clone());
        assert_eq!(shell.search_cmd("dummy"), Some(cmd));
        assert_eq!(shell.search_cmd("notfound"), None);
        Ok(())
    }

    #[test_case]
    fn test_get_line_backspace() -> Result<(), &'static str> {
        // get_line is interactive; skip direct test here
        // Could be tested with input mocking in integration tests
        Ok(())
    }
}
