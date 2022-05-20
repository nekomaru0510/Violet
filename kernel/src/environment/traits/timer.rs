//! タイマ取得・解放用のトレイト

pub trait HasTimer {
    type Device;

    fn take_timer(&mut self) -> <Self as HasTimer>::Device;
    fn release_timer(&mut self, timer: <Self as HasTimer>::Device);
}
