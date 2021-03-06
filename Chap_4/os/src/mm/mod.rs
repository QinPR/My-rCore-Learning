pub use memory_set::KERNEL_SPACE;

pub fn init(){
    heap_allocator::init_heap();
    frame_allocator::init_frame_allocator();
    KERNEL_SPACE.exclusive_access().activate();
}