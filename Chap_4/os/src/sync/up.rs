use core::cell::RefCell;
use core::cell::RefMut;

pub struct UPSafeCell<T> {       // 允许我们在单核上安全使用可变全局变量
    inner: RefCell<T>,      // 不允许多个读写操作同时存在，但推迟这种检查到运行的时候：使用全局变量前需要先borrow_mut()
}

unsafe impl<T> Sync for UPSafeCell<T> {}     // 标记为Sync使其可以作为一个全局变量

impl<T> UPSafeCell<T> {

    pub unsafe fn new(value: T) -> Self {
        Self { inner: RefCell::new(value) }
    }
    
    pub fn exclusive_access(&self) -> RefMut<'_, T> {    // 可以获得其包裹数据的独占访问权
        self.inner.borrow_mut()
    }
}