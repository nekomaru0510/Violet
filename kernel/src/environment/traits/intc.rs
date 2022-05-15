//! 割込みコントローラ取得・解放用のトレイト

pub trait HasIntc {
    type Device;

    fn take_intc(&mut self) -> <Self as HasIntc>::Device;
    fn release_intc(&mut self, intc: <Self as HasIntc>::Device);

}
