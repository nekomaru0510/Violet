/*
 * コンソールリソース用のトレイト
 */
use core::fmt;
pub trait TraitTty {
    //fn write(&self, c: u8);
    fn write(&self, s: &str) -> fmt::Result;
    fn read(&self) -> u8;
}
