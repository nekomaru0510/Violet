pub mod vshell;

pub trait KernelThread {
    fn new() -> Self;
    fn run(&mut self) /*-> Result<i32, &str>*/ ;
}



