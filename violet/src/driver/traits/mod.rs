pub mod arch;
pub mod cpu;
pub mod intc;
pub mod serial;
pub mod timer;

/*
pub trait TraitDriver {
    /* ハードウェア依存処理 [todo fix] ioctlはマクロで作ったほうがいいかも。要検討 */
    //fn ioctl<T, U>(&self, id: T, arg: U);
    //fn ioctl_mut<T, U>(&mut self, id: T, arg: U);
}
*/