pub mod vshell;
pub mod sched;

/* [todo delete] 割込み用 */
use crate::Context;

pub trait TraitService {
    /* 実行 */
    fn run(&mut self) /*-> Result<i32, &str>*/ ;
    /* 割込みハンドラ */
    fn interrupt(&mut self, cont: &mut Context);
}