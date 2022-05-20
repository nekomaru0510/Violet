//! メモリ管理
pub mod sv39;
pub mod sv48;

// ビットフィールド操作用[todo fix 場所の変更]
pub struct BitField {
    pub offset: u8,
    pub width: u8,
}
impl BitField {
    pub fn new() -> Self {
        BitField {
            offset: 0,
            width: 0,
        }
    }

    /* offsetとwidthに沿ったビットパターンを生成 */
    /* valはT型にしたい */
    pub fn pattern(&self, val: u64) -> u64 {
        let mask = (2 << (self.width - 1)) - 1;
        (val & mask) << self.offset
    }

    /* offsetとwidthに沿ったビットパターンを抽出 */
    /* valはT型にしたい */
    pub fn mask(&self, val: u64) -> u64 {
        let mask = (2 << (self.width - 1)) - 1;
        (val >> self.offset) & mask
    }
}
