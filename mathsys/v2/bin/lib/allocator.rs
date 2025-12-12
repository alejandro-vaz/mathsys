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
        loop {
            let mark = self.mark();
            let from = (mark + layout.align() - 1) & !(layout.align() - 1);
            let to = from + layout.size();
            if to > self.end() {
                self.init();
                crate::stdout::crash(1);
            }
            match self.next.compare_exchange_weak(
                mark, 
                to, 
                crate::Ordering::Release, 
                crate::Ordering::Acquire
            ) {
                Ok(_) => return from as *mut u8,
                Err(_) => continue
            }
        }
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
    #[inline(always)]
    pub fn init(&self) -> () {
        self.reset(self.start())
    }
    #[inline(always)]
    pub fn start(&self) -> usize {return unsafe {crate::HEAP.as_ptr() as usize}}
    #[inline(always)]
    pub fn end(&self) -> usize {return unsafe {(crate::HEAP.as_ptr() as usize).saturating_add(crate::SETTINGS.memsize)}}
    #[inline(always)]
    pub fn mark(&self) -> usize {return self.next.load(crate::Ordering::Acquire)}
    #[inline(always)]
    pub fn reset(&self, mark: usize) -> () {self.next.store(mark, crate::Ordering::Release)}
    #[inline(always)]
    pub fn tempSpace<Function, Returns>(&self, process: Function) -> Returns where Function: FnOnce() -> Returns {
        let mark = self.mark();
        let result = process();
        self.reset(mark);
        return result;
    }
}