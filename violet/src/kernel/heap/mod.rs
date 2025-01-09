//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

//! Heap Allocator

pub mod slab;

extern crate alloc;
use alloc::alloc::{GlobalAlloc, Layout};
use slab::SlabAllocator;

#[global_allocator]
static ALLOCATOR: HeapOperator = HeapOperator::new();

// Initial heap
// Initial heap used by the kernel of the root container at initialization
pub static mut HEAP: SlabAllocator = SlabAllocator::empty();

pub fn init_allocater(start: usize, end: usize) {
    unsafe {
        HEAP = SlabAllocator::new(start, end - start);
    }
}

// Select the heap to use by referring to the container when acquiring and releasing the heap
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
        HEAP.deallocate(ptr, layout);
    }
}

// Trait for heap allocator
pub trait TraitHeap {
    fn allocate(&mut self, layout: Layout) -> *mut u8;
    unsafe fn deallocate(&mut self, ptr: *mut u8, layout: Layout);
}

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    panic!("Alloc Error !");
}
