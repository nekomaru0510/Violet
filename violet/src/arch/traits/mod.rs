//! CPU用のトレイト

pub mod context;
pub mod hypervisor;
pub mod mmu;
pub mod registers;

// Processor Core specific processing
pub trait TraitCpu {
    /* コアごとの初期化 */
    fn setup(&self);
}

// Architecture specific processing
pub trait TraitArch {
    fn get_cpuid() -> usize;
    /* CPUのstart */
    fn wakeup(cpuid: usize);
    /* CPUの停止 */
    fn sleep();
    /* ベクタの登録 */
    fn register_vector(vecid: usize, func: fn(regs: *mut usize)) -> Result<(), ()>;
    /* ベクタハンドラの呼出し */
    fn call_vector(vecid: usize, regs: *mut usize) -> Result<(), ()>;
    /* 割込みの有効化 */
    fn enable_interrupt();
    /* 割込みの無効化 */
    fn disable_interrupt();

    /* コア間通信 */
    fn ipi(core_id: usize);
}
