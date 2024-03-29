//! CPU用のトレイト

pub mod context;
pub mod hypervisor;
pub mod mmu;
pub mod registers;

pub trait TraitCpu {
    //type Registers;

    /* コアごとの初期化 */
    fn core_init(&self);

    /* CPUのstart */
    fn wakeup(&self);
    /* CPUの停止 */
    fn sleep(&self);
    /* ベクタの登録 */
    //fn register_vector(&mut self, vecid: usize, func: fn(regs: &mut Self::Registers));
    fn register_vector(&mut self, vecid: usize, func: fn(regs: *mut usize));
    /* ベクタハンドラの呼出し */
    //fn call_vector(&self, vecid: usize, regs: &mut Self::Registers);
    fn call_vector(&self, vecid: usize, regs: *mut usize);
    /* 割込みの有効化 */
    fn enable_interrupt(&self);
    /* 割込みの無効化 */
    fn disable_interrupt(&self);

    /* コア間通信 */
    fn ipi(&self, core_id: usize);
}
