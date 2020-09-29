pub trait StdinIF {
    fn read_char(&self) -> u8;
    fn read_str(&self) -> &str;
}
