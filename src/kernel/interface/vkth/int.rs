//! Interrupt Interface

use crate::kernel::table::sig::Sig;

pub fn entry() {
    //割込みIDを判別する。
    let id: u32 = 1;

    //ハンドラを呼び出す。
    let sig = Sig::new();
    sig.call(id);

}



