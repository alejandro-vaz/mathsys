//
//  ALLOCATOR
//

// ALLOCATOR -> STRUCT
pub struct Allocator {
    next: crate::AtomicUsize
}

// ALLOCATOR -> MULTITHREADING
unsafe impl Sync for Allocator {}

// ALLOCATOR -> IMPLEMENTATION
unsafe impl crate::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: crate::Layout) -> *mut u8 {
        let from = (self.mark() + layout.align() - 1) & !(layout.align() - 1);
        let to = from + layout.size();
        self.reset(to);
        if self.end().wrapping_sub(from) > 5000 && self.end().wrapping_sub(to) <= 5000 {crate::stdout::crash(1)}
        return from as *mut u8;
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: crate::Layout) -> () {}
}

// ALLOCATOR -> METHODS
impl Allocator {
    pub const fn new() -> Self {
        Allocator {
            next: crate::AtomicUsize::new(0)
        }
    }
    pub fn init(&self) -> () {
        self.reset(self.start())
    }
    #[inline(always)]
    pub fn start(&self) -> usize {return unsafe {crate::HEAP.as_ptr() as usize}}
    #[inline(always)]
    pub fn end(&self) -> usize {return unsafe {crate::HEAP.as_ptr() as usize + crate::SETTINGS.memsize}}
    pub fn mark(&self) -> usize {return self.next.load(crate::Ordering::Relaxed)}
    pub fn reset(&self, mark: usize) -> () {self.next.store(mark, crate::Ordering::Relaxed)}
    pub fn tempSpace<Function, Returns>(&self, process: Function) -> Returns where Function: FnOnce() -> Returns {
        let mark = self.mark();
        let result = process();
        self.reset(mark);
        return result;
    }
}