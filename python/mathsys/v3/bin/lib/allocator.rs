//^
//^ ALLOCATOR
//^

//> ALLOCATOR -> HEAP
#[repr(align(128))]
struct Heap([u8; crate::SETTINGS.memsize]);
static mut HEAP: Heap = Heap([0; crate::SETTINGS.memsize]);

//> ALLOCATOR -> BITMAP
static BITMAP: [crate::AtomicBool; crate::SETTINGS.memsize / crate::SETTINGS.block] = [
    const {crate::AtomicBool::new(false)}; 
    crate::SETTINGS.memsize / crate::SETTINGS.block
];


//> ALLOCATOR -> STRUCT
pub struct Allocator {}

//> ALLOCATOR -> MULTITHREADING
unsafe impl Sync for Allocator {}

//> ALLOCATOR -> IMPLEMENTATION
unsafe impl crate::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: crate::Layout) -> *mut u8 {
        let blocks = self.blockAmount(layout.size());
        loop {
            let start = self.search(blocks);
            let raw = self.start() + start * crate::SETTINGS.block;
            let aligned = (raw + (layout.align() - 1)) & !(layout.align() - 1);
            if aligned + layout.size() <= raw + blocks * crate::SETTINGS.block {
                if self.tryAssign(start, blocks) {return aligned as *mut u8}
            }
        }
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: crate::Layout) -> () {
        let start = ((pointer as usize) - self.start()) / crate::SETTINGS.block;
        let blocks = self.blockAmount(layout.size());
        self.free(start, blocks);
    }
}

//> ALLOCATOR -> METHODS
impl Allocator {
    pub const fn new() -> Self {Allocator {}}
    fn reset(&self) -> () {self.free(0, crate::SETTINGS.memsize / crate::SETTINGS.block)}
    fn start(&self) -> usize {return unsafe {HEAP.0.as_ptr() as usize}}
    fn blockAmount(&self, size: usize) -> usize {return (size + crate::SETTINGS.block - 1) / crate::SETTINGS.block}
    fn search(&self, amount: usize) -> usize {
        let mut count = 0;
        let mut starting = 0;
        for (index, value) in BITMAP.iter().enumerate() {
            if count == 0 {
                if !value.load(crate::Ordering::Acquire) {
                    starting = index;
                    count += 1;
                }
            } else {
                if !value.load(crate::Ordering::Acquire) {
                    count += 1;
                } else {
                    count = 0;
                }
            }
            if count == amount {return starting}
        }
        self.reset();
        crate::stdout::crash(crate::stdout::Code::OutOfMemory);
    }
    fn tryAssign(&self, from: usize, amount: usize) -> bool {
        for index in from..(from + amount) {
            if BITMAP[index].compare_exchange(
                false,
                true,
                crate::Ordering::AcqRel,
                crate::Ordering::Acquire
            ).is_err() {
                for rollback in from..index {BITMAP[rollback].store(false, crate::Ordering::Release)}
                return false;
            }
        }
        return true;
    }
    fn free(&self, from: usize, amount: usize) -> () {
        for index in from..(from + amount) {BITMAP[index].store(false, crate::Ordering::Release)}
    }
}


//^
//^ STATISTICS
//^

//> STATISTICS -> MARK
pub fn mark() -> usize {
    let mut using = 0;
    for block in BITMAP.iter() {if block.load(crate::Ordering::Acquire) {using += 1}};
    return using * crate::SETTINGS.block;
}