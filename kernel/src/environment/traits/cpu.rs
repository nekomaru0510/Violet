//! CPU取得・解放用のトレイト

pub trait HasCpu {
    type Device;

    fn take_cpu(&mut self) -> <Self as HasCpu>::Device;
    fn release_cpu(&mut self, cpu: <Self as HasCpu>::Device);
}
