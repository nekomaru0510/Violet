//! 割込みコントローラ用のトレイト

pub trait TraitIntc {
    /* 割込みの有効化 */
    fn enable_intrrupt(&self, id: u32);
    /* 割込みの無効化 */
    fn disable_interrupt(&self, id: u32);
    /* 最高優先度のペンディング状態の割込み番号を取得 */
    fn get_pend_int(&self) -> u32;
    /* 処理完了した割込み番号を格納 */
    fn set_comp_int(&self, id: u32);
}
