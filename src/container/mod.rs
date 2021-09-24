//! コンテナ
pub mod sample_container;
pub mod hypervisor_container;

/* コンテナトレイト */
pub trait TraitContainer {
    fn run(&mut self);
}
