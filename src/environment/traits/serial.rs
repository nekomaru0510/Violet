//! シリアルデバイス取得・解放用のトレイト

pub trait HasSerial {
    type Device;

    fn take_serial(&mut self) -> <Self as HasSerial>::Device;
    fn release_serial(&mut self, serial: <Self as HasSerial>::Device);

}
