//! Minimal allocator
// Allocate memory as much as requested
// Can't free memory

use core::ptr;
use core::cell::UnsafeCell;
use core::alloc::GlobalAlloc;
use alloc::alloc::Layout;

struct MinimumAllocator {
    head: UnsafeCell<usize>,
    end: usize,
}

#[global_allocator]
static HEAP: MinimumAllocator = MinimumAllocator {
    head: UnsafeCell::new(0x8004_0000),
    end: 0x8006_0000,
};

unsafe impl Sync for MinimumAllocator {}

unsafe impl GlobalAlloc for MinimumAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        let head = self.head.get();
        let _size = _layout.size();
        let align = _layout.align();
        
        let start = calc_align(*head, align) as usize;

        /* Size over */
        if start + _size > self.end {
            ptr::null_mut()
        } else {
            *head = start + _size; // update head
            start as *mut u8
        }
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Can't free memory
    }
}

fn calc_align(pos: usize, n: usize) -> *mut u8 {
    ((pos+n-1) & !(n-1)) as *mut u8
}

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop{}
}
