pub trait TraitSerial {
    fn write(&self, c: u8);
    fn read(&self) -> u8;
}