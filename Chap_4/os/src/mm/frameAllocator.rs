trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self) -> Option<PhysPageNum>;
    fn dealloc(&mut self, pnn: PhysPageNum);
}

pub struct StackFrameAllocator {
    current: usize,    // 空闲内存的起始物理页号
    end: usize,        // 空闲内存的结束物理页号
    recycled: Vec<usize>,
}

impl FrameAllocator for StackFrameAllocator {
    fn new() -> Self {
        Self {
            current: 0,
            end: 0,
            recycled: Vec::new(),
        }
    }
    fn alloc(&mut self) -> Option<PhysPageNum> {
        if let Some(pnn) = self.recycled.pop() {     // 率先检查有没有回收的物理页帧，有的话直接从里面pop出来一个
            Some(ppn.into())
        } else{
            if self.current == self.end {
                None
            } else{
                self.current += 1;
                Some((self.current - 1).into())     // 用into的方法将usize转化挣了物理页帧号
            }
        }
    }
    fn dealloc(&mut self, ppn: PhysPageNum) {
        let pnn = pnn.0;
        if pnn >= self.current || self.recycled
            .iter()
            .find(|&v| {*v == pnn})
            .is_some(){      // 用is_some是因为find会返回一个Option<T>
                panic!("Frame pnn = {:?} has not been allocated!", pnn);
        }
        self.recycled.push(pnn);
    }
}

impl StackFrameAllocator {
    pub fn init(&mut self, l: PhysPageNum, r: PhysPageNum){
        self.current = l.0;
        self.end = r.0;
    }
}

// 创建一个全局实例
use crate::sync::UpsafeCell;
type FrameAllocatorImpl = StackFrameAllocator;
lazy_static!{
    pub static ref FRAME_ALLOCATOR: UpsafeCell<FrameAllocatorImpl> = unsafe {
        UpsafeCell::new(FrameAllocatorImpl::new())
    };
}

pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }
    FRAME_ALLOCATOR
        .exclusive_access()
        .init(PhysAddr::from(ekernel as usize).ceil(), PhysAddr::from(MEMORY_END).floor());
}

pub struct FrameTracker {
    pub pnn: PhysPageNum,
}

impl FrameTracker {
    pub fn new(pnn: PhysPageNum) -> Self {
        let bytes_array = pnn.get_bytes_array();
        for i in bytes_array {
            *i = 0;      // 零初始化
        }
        Self { pnn }
    }
}

impl Drop for FrameTracker{      // 当一个FrameTracker实例被回收的时候，自动将物理页帧回收使用
    fn drop(&mut self){
        frame_dealloc(self.ppn);
    }
}

pub fn frame_alloc() -> Option<FrameTracker> {
    FRAME_ALLOCATOR
        .exclusive_access()
        .alloc()
        .map(|ppn| FrameTracker::new(pnn))
}

pub fn frame_dealloc(pnn: PhysPageNum) {
    FRAME_ALLOCATOR
        .exclusive_access()
        .dealloc(ppn);
}