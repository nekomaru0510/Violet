//! Slabアロケータ

pub mod heap;
pub mod slab;

extern crate alloc;
use alloc::alloc::{GlobalAlloc, Layout};

use heap::Heap;

#[global_allocator]
static ALLOCATOR: HeapOperator = HeapOperator::new();

static mut HEAP: Heap = Heap::empty();

pub fn init_allocater(start: usize, end: usize) {
    let heap_start = start;
    let heap_end = end;
    let heap_size = heap_end - heap_start;
    unsafe {
        HEAP = Heap::new(heap_start, heap_size);
    }
}

pub struct HeapOperator {}

impl HeapOperator {
    pub const fn new() -> HeapOperator {
        HeapOperator {}
    }
}

unsafe impl GlobalAlloc for HeapOperator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HEAP.allocate(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        HEAP.deallocate(ptr, layout)
    }

    /*
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 { ... } {

    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize
    ) -> *mut u8 { ... } {

    }
    */
}
