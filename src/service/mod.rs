pub mod vshell;

pub trait TraitService {
    fn run(&mut self) /*-> Result<i32, &str>*/ ;
}