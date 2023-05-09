//! slabアロケータ
extern crate alloc;
use alloc::alloc::Layout;

use crate::kernel::heap::TraitHeap;

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
}

pub struct SlabAllocator {
    slab_64_bytes: Slab,
    slab_128_bytes: Slab,
    slab_256_bytes: Slab,
    slab_512_bytes: Slab,
    slab_1024_bytes: Slab,
    slab_2048_bytes: Slab,
    slab_4096_bytes: Slab,
}

impl SlabAllocator {
    pub const fn empty() -> SlabAllocator {
        SlabAllocator {
            slab_64_bytes: Slab::empty(),
            slab_128_bytes: Slab::empty(),
            slab_256_bytes: Slab::empty(),
            slab_512_bytes: Slab::empty(),
            slab_1024_bytes: Slab::empty(),
            slab_2048_bytes: Slab::empty(),
            slab_4096_bytes: Slab::empty(),
        }
    }

    pub unsafe fn new(heap_start_addr: usize, heap_size: usize) -> SlabAllocator {
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
        SlabAllocator {
            slab_64_bytes: Slab::new(heap_start_addr, slab_size, 64),
            slab_128_bytes: Slab::new(heap_start_addr + slab_size, slab_size, 128),
            slab_256_bytes: Slab::new(heap_start_addr + 2 * slab_size, slab_size, 256),
            slab_512_bytes: Slab::new(heap_start_addr + 3 * slab_size, slab_size, 512),
            slab_1024_bytes: Slab::new(heap_start_addr + 4 * slab_size, slab_size, 1024),
            slab_2048_bytes: Slab::new(heap_start_addr + 5 * slab_size, slab_size, 2048),
            slab_4096_bytes: Slab::new(heap_start_addr + 6 * slab_size, slab_size, 4096),
        }
    }

    pub fn usable_size(&self, layout: &Layout) -> (usize, usize) {
        match SlabAllocator::layout_to_allocator(&layout) {
            HeapAllocator::Slab64Bytes => (layout.size(), 64),
            HeapAllocator::Slab128Bytes => (layout.size(), 128),
            HeapAllocator::Slab256Bytes => (layout.size(), 256),
            HeapAllocator::Slab512Bytes => (layout.size(), 512),
            HeapAllocator::Slab1024Bytes => (layout.size(), 1024),
            HeapAllocator::Slab2048Bytes => (layout.size(), 2048),
            HeapAllocator::Slab4096Bytes => (layout.size(), 4096),
        }
    }

    pub fn layout_to_allocator(layout: &Layout) -> HeapAllocator {
        if layout.size() > 4096 {
            panic!("Alloc more 4096Bytes!! ");
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

impl TraitHeap for SlabAllocator {
    fn allocate(&mut self, layout: Layout) -> *mut u8 {
        match SlabAllocator::layout_to_allocator(&layout) {
            HeapAllocator::Slab64Bytes => self.slab_64_bytes.allocate(layout),
            HeapAllocator::Slab128Bytes => self.slab_128_bytes.allocate(layout),
            HeapAllocator::Slab256Bytes => self.slab_256_bytes.allocate(layout),
            HeapAllocator::Slab512Bytes => self.slab_512_bytes.allocate(layout),
            HeapAllocator::Slab1024Bytes => self.slab_1024_bytes.allocate(layout),
            HeapAllocator::Slab2048Bytes => self.slab_2048_bytes.allocate(layout),
            HeapAllocator::Slab4096Bytes => self.slab_4096_bytes.allocate(layout),
        }
    }

    unsafe fn deallocate(&mut self, ptr: *mut u8, layout: Layout) {
        match SlabAllocator::layout_to_allocator(&layout) {
            HeapAllocator::Slab64Bytes => self.slab_64_bytes.deallocate(ptr),
            HeapAllocator::Slab128Bytes => self.slab_128_bytes.deallocate(ptr),
            HeapAllocator::Slab256Bytes => self.slab_256_bytes.deallocate(ptr),
            HeapAllocator::Slab512Bytes => self.slab_512_bytes.deallocate(ptr),
            HeapAllocator::Slab1024Bytes => self.slab_1024_bytes.deallocate(ptr),
            HeapAllocator::Slab2048Bytes => self.slab_2048_bytes.deallocate(ptr),
            HeapAllocator::Slab4096Bytes => self.slab_4096_bytes.deallocate(ptr),
        }
    }

}

pub struct Slab {
    block_size: usize,
    free_block_list: FreeBlockList,
}

impl Slab {
    pub const fn empty() -> Slab {
        Slab {
            block_size: 0,
            free_block_list: FreeBlockList::new_empty(),
        }
    }

    pub unsafe fn new(start_addr: usize, slab_size: usize, block_size: usize) -> Slab {
        let num_of_blocks = slab_size / block_size;
        Slab {
            block_size: block_size,
            free_block_list: FreeBlockList::new(start_addr, block_size, num_of_blocks),
        }
    }

    pub fn used_blocks(&self) -> usize {
        self.free_block_list.len()
    }

    pub unsafe fn grow(&mut self, start_addr: usize, slab_size: usize) {
        let num_of_blocks = slab_size / self.block_size;
        let mut block_list = FreeBlockList::new(start_addr, self.block_size, num_of_blocks);
        while let Some(block) = block_list.pop() {
            self.free_block_list.push(block);
        }
    }

    pub fn allocate(&mut self, layout: Layout) -> *mut u8 {
        match self.free_block_list.pop() {
            Some(block) => block.addr() as *mut u8,
            None => 0 as *mut u8,
        }
    }

    pub fn deallocate(&mut self, ptr: *mut u8) {
        let ptr = ptr as *mut FreeBlock;
        unsafe {
            self.free_block_list.push(&mut *ptr);
        }
    }
}

struct FreeBlockList {
    len: usize,
    head: Option<&'static mut FreeBlock>,
}

impl FreeBlockList {
    unsafe fn new(start_addr: usize, block_size: usize, num_of_blocks: usize) -> FreeBlockList {
        let mut new_list = FreeBlockList::new_empty();
        for i in (0..num_of_blocks).rev() {
            let new_block = (start_addr + i * block_size) as *mut FreeBlock;
            new_list.push(&mut *new_block);
        }
        new_list
    }

    const fn new_empty() -> FreeBlockList {
        FreeBlockList { len: 0, head: None }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn pop(&mut self) -> Option<&'static mut FreeBlock> {
        self.head.take().map(|node| {
            self.head = node.next.take();
            self.len -= 1;
            node
        })
    }

    fn push(&mut self, free_block: &'static mut FreeBlock) {
        free_block.next = self.head.take();
        self.len += 1;
        self.head = Some(free_block);
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }
}

impl Drop for FreeBlockList {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

struct FreeBlock {
    next: Option<&'static mut FreeBlock>,
}

impl FreeBlock {
    fn addr(&self) -> usize {
        self as *const _ as usize
    }
}
