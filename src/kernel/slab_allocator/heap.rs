//! 

extern crate alloc;
use alloc::alloc::Layout;
//use alloc::alloc::GlobalAlloc;

use super::slab::Slab;

pub const NUM_OF_SLABS: usize = 8;
pub const MIN_SLAB_SIZE: usize = 4096;
pub const MIN_HEAP_SIZE: usize = NUM_OF_SLABS * MIN_SLAB_SIZE;

pub enum HeapAllocator {
    Slab64Bytes,
    Slab128Bytes,
    Slab256Bytes,
    Slab512Bytes,
    Slab1024Bytes,
    Slab2048Bytes,
    Slab4096Bytes,
    //LinkedListAllocator,
}

pub struct Heap {
    slab_64_bytes: Slab,
    slab_128_bytes: Slab,
    slab_256_bytes: Slab,
    slab_512_bytes: Slab,
    slab_1024_bytes: Slab,
    slab_2048_bytes: Slab,
    slab_4096_bytes: Slab,
    //linked_list_allocator: linked_list_allocator::Heap,
}

impl Heap {
    pub const fn empty() -> Heap {
        Heap {
            slab_64_bytes: Slab::empty(),
            slab_128_bytes: Slab::empty(),
            slab_256_bytes: Slab::empty(),
            slab_512_bytes: Slab::empty(),
            slab_1024_bytes: Slab::empty(),
            slab_2048_bytes: Slab::empty(),
            slab_4096_bytes: Slab::empty(),
            //linked_list_allocator: linked_list_allocator::Heap::new(heap_start_addr + 7 * slab_size, slab_size),
        }
    } 

    pub unsafe fn new(heap_start_addr: usize, heap_size: usize) -> Heap {
        assert!(
            heap_start_addr % 4096 == 0,
            "Start address should be page aligned"
        );
        assert!(
            heap_size >= MIN_HEAP_SIZE,
            "Heap size should be greater or equal to minimum heap size"
        );
        assert!(
            heap_size % MIN_HEAP_SIZE == 0,
            "Heap size should be a multiple of minimum heap size"
        );

        // ヒープ領域を8(NUM_OF_SLABS)等分する
        let slab_size = heap_size / NUM_OF_SLABS;
        Heap {
            slab_64_bytes: Slab::new(heap_start_addr, slab_size, 64),
            slab_128_bytes: Slab::new(heap_start_addr + slab_size, slab_size, 128),
            slab_256_bytes: Slab::new(heap_start_addr + 2 * slab_size, slab_size, 256),
            slab_512_bytes: Slab::new(heap_start_addr + 3 * slab_size, slab_size, 512),
            slab_1024_bytes: Slab::new(heap_start_addr + 4 * slab_size, slab_size, 1024),
            slab_2048_bytes: Slab::new(heap_start_addr + 5 * slab_size, slab_size, 2048),
            slab_4096_bytes: Slab::new(heap_start_addr + 6 * slab_size, slab_size, 4096),
            //linked_list_allocator: linked_list_allocator::Heap::new(heap_start_addr + 7 * slab_size, slab_size),
        }
    }

    pub fn allocate(&mut self, layout: Layout) -> *mut u8 {
        match Heap::layout_to_allocator(&layout) {
            HeapAllocator::Slab64Bytes => self.slab_64_bytes.allocate(layout),
            HeapAllocator::Slab128Bytes => self.slab_128_bytes.allocate(layout),
            HeapAllocator::Slab256Bytes => self.slab_256_bytes.allocate(layout),
            HeapAllocator::Slab512Bytes => self.slab_512_bytes.allocate(layout),
            HeapAllocator::Slab1024Bytes => self.slab_1024_bytes.allocate(layout),
            HeapAllocator::Slab2048Bytes => self.slab_2048_bytes.allocate(layout),
            HeapAllocator::Slab4096Bytes => self.slab_4096_bytes.allocate(layout),
            //HeapAllocator::LinkedListAllocator => self.linked_list_allocator.allocate_first_fit(layout),
        }
    }

    pub unsafe fn deallocate(&mut self, ptr: *mut u8, layout: Layout) {
        match Heap::layout_to_allocator(&layout) {
            HeapAllocator::Slab64Bytes => self.slab_64_bytes.deallocate(ptr),
            HeapAllocator::Slab128Bytes => self.slab_128_bytes.deallocate(ptr),
            HeapAllocator::Slab256Bytes => self.slab_256_bytes.deallocate(ptr),
            HeapAllocator::Slab512Bytes => self.slab_512_bytes.deallocate(ptr),
            HeapAllocator::Slab1024Bytes => self.slab_1024_bytes.deallocate(ptr),
            HeapAllocator::Slab2048Bytes => self.slab_2048_bytes.deallocate(ptr),
            HeapAllocator::Slab4096Bytes => self.slab_4096_bytes.deallocate(ptr),
            //HeapAllocator::LinkedListAllocator => self.linked_list_allocator.deallocate(ptr, layout),
        }
    }

    pub fn usable_size(&self, layout: &Layout) -> (usize, usize) {
        match Heap::layout_to_allocator(&layout) {
            HeapAllocator::Slab64Bytes => (layout.size(), 64),
            HeapAllocator::Slab128Bytes => (layout.size(), 128),
            HeapAllocator::Slab256Bytes => (layout.size(), 256),
            HeapAllocator::Slab512Bytes => (layout.size(), 512),
            HeapAllocator::Slab1024Bytes => (layout.size(), 1024),
            HeapAllocator::Slab2048Bytes => (layout.size(), 2048),
            HeapAllocator::Slab4096Bytes => (layout.size(), 4096),
            //HeapAllocator::LinkedListAllocator => (layout.size(), layout.size()),
        }
    }

    pub fn layout_to_allocator(layout: &Layout) -> HeapAllocator {
        if layout.size() > 4096 {
            panic!("Alloc more 4096Bytes!! ");
            //HeapAllocator::LinkedListAllocator
        } else if layout.size() <= 64 && layout.align() <= 64 {
            HeapAllocator::Slab64Bytes
        } else if layout.size() <= 128 && layout.align() <= 128 {
            HeapAllocator::Slab128Bytes
        } else if layout.size() <= 256 && layout.align() <= 256 {
            HeapAllocator::Slab256Bytes
        } else if layout.size() <= 512 && layout.align() <= 512 {
            HeapAllocator::Slab512Bytes
        } else if layout.size() <= 1024 && layout.align() <= 1024 {
            HeapAllocator::Slab1024Bytes
        } else if layout.size() <= 2048 && layout.align() <= 2048 {
            HeapAllocator::Slab2048Bytes
        } else {
            HeapAllocator::Slab4096Bytes
        }
    }


}

/*
unsafe impl GlobalAlloc for Heap {
    /*
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.deallocate(ptr, layout)
    }
    */
    /*
    fn oom(&mut self, err: AllocErr) -> ! {
        panic!("Out of memory: {:?}", err);
    }

    fn usable_size(&self, layout: &Layout) -> (usize, usize) {
        self.usable_size(layout)
    }
    */
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
 */

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    panic!("Alloc Error !");
}

