// Water Mark Allocator
// 要求されたサイズ分だけ、メモリを確保する。
// メモリの解放はできない

use core::ptr;
use core::cell::UnsafeCell;
use core::alloc::GlobalAlloc;
extern crate alloc;
use alloc::alloc::Layout;
//use core::mem::{size_of, align_of};

struct MinimumAllocator {
    head: UnsafeCell<usize>,
    end: usize,
}
/*
extern "C" {
    static mut __HEAP_BASE: u32 = 0;
}*/
//static mut HEAP: u8 = 0;
// グローバルアロケータ
#[global_allocator]
static HEAP: MinimumAllocator = MinimumAllocator {
    head: UnsafeCell::new(0x8004_0000),
    end: 0x8006_0000,
};

unsafe impl Sync for MinimumAllocator {}

unsafe impl GlobalAlloc for MinimumAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        /* データの配置 */
        let head = self.head.get();
        let _size = _layout.size();
        let align = _layout.align();
        
        let start = calc_align(*head, align) as usize;

        /* サイズオーバー */
        if start + _size > self.end {
            ptr::null_mut()
        } else {
            *head = start + _size; //headの更新
            start as *mut u8
        }
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // メモリ解放はできない
    }
}

fn calc_align(pos: usize, n: usize) -> *mut u8 {
    ((pos+n-1) & !(n-1)) as *mut u8
}

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop{}
}
