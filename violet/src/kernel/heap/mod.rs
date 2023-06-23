//! ヒープアロケータ

pub mod slab;

extern crate alloc;
use crate::container::is_ready_container;
use crate::kernel::get_mut_kernel;
use alloc::alloc::{GlobalAlloc, Layout};
use slab::SlabAllocator;

#[global_allocator]
static ALLOCATOR: HeapOperator = HeapOperator::new();

/* 初期ヒープ(初期化時、ルートコンテナのカーネルで利用される) */
pub static mut HEAP: SlabAllocator = SlabAllocator::empty();

pub fn init_allocater(start: usize, end: usize) {
    unsafe {
        HEAP = SlabAllocator::new(start, end - start);
    }
}

/* ヒープ取得・解放操作時にコンテナを参照して、利用するヒープを選択する */
pub struct HeapOperator {}

impl HeapOperator {
    pub const fn new() -> HeapOperator {
        HeapOperator {}
    }
}

unsafe impl GlobalAlloc for HeapOperator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if is_ready_container() {
            get_mut_kernel().heap.as_mut().allocate(layout)
        } else {
            HEAP.allocate(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if is_ready_container() {
            get_mut_kernel().heap.as_mut().deallocate(ptr, layout);
        } else {
            HEAP.deallocate(ptr, layout);
        }
    }
}

/* ヒープアロケータ用のトレイト */
pub trait TraitHeap {
    fn allocate(&mut self, layout: Layout) -> *mut u8;
    unsafe fn deallocate(&mut self, ptr: *mut u8, layout: Layout);
}

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    panic!("Alloc Error !");
}
