//! シリアルデバイス用のトレイト

pub trait TraitSerial {
    fn write(&self, c: u8);
    fn read(&self) -> u8;
    fn enable_interrupt(&self);
    fn disable_interrupt(&self);
}
