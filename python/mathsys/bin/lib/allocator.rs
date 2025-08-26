//
//  ALLOCATOR
//

// ALLOCATOR -> STRUCT
pub struct Allocator {
    pub start: usize,
    pub end: usize,
    pub next: crate::AtomicUsize
}

// ALLOCATOR -> MULTITHREADING
unsafe impl Sync for Allocator {}

// ALLOCATOR -> IMPLEMENTATION
unsafe impl crate::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: crate::Layout) -> *mut u8 {
        let from = (self.next.load(crate::Ordering::Relaxed) + layout.align() - 1) & !(layout.align() - 1);
        let to = from.saturating_add(layout.size());
        self.next.store(to, crate::Ordering::Relaxed);
        if crate::SETTINGS.memsize.saturating_sub(from.saturating_sub(self.start)) > 5000 && crate::SETTINGS.memsize.saturating_sub(to.saturating_sub(self.start)) <= 5000 {
            crate::stdout::crash(1);
        }
        return from as *mut u8;
    }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: crate::Layout) {}
}

// ALLOCATOR -> HEAP INITIALIZATION
#[allow(static_mut_refs)]
pub unsafe fn init() {
    crate::ALLOCATOR.start = crate::HEAP.as_mut_ptr() as usize;
    crate::ALLOCATOR.end = crate::ALLOCATOR.start + crate::SETTINGS.memsize;
    crate::ALLOCATOR.next.store(crate::ALLOCATOR.start, crate::Ordering::Relaxed);
}