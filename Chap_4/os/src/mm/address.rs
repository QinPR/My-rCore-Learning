
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhysAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtAddr(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub sturct PhysPageNum(pub usize);

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtPageNum(pub usize);

const PA_WIDTH_SV39: usize = 56;
const PPN_WIDTH_SV39: usize = PA_WIDTH+SV39 - PAGE_SIZE_BITS;

impl From<usize> for PhysAddr {
    fn from(v: usize) -> Self {Self(v & ( (1 << PA_WIDTH_SV39) - 1 ))}
}

impl From<usize> for PhysPageNum {
    fn from(v: usize) -> Self {Self(v & ( (1 << PPN_WIDTH_SV39) - 1 ))}
}

impl From<PhysAddr> for usize {
    fn from(v: PhysAddr) -> Self {v.0}
}

impl From<PhysPageNum> for usize {
    fn from(v: PhysPageNum) -> Self {v.0}
}

impl PhysAddr {
    pub fn page_offset(&self) -> usize {self.0 & (PAGE_SIZE -1)}

    pub fn floor(&self) -> PhysPageNum { PhysPageNum(self.0 / PAGE_SIZE) }

    pub fn ceil(&self) -> PhysPageNum { PhysPageNum((self.0 + PAGE_SIZE -1) / PAGE_SIZE) }
}

impl From<PhysAddr> for PhysPageNum {
    fn from(v: PhysAddr) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<PhysPageNum> for PhysAddr {
    fn from(v: PhysPageNum) -> Self { Self(v.0 << PAGE_SIZE_BITS) } 
}

impl PhysPageNum {
    pub fn get_pte_array(&self) -> &'static mut [PageTableEntry] {
        let pa: PhysAddr = self.clone().into();      // into将物理页号转为物理地址
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, 512)
        }
    }
    pub fn get_bytes_array(&self) -> &'static mut [u8] {
        let pa: PhysAddr = self.clone().into();
        unsafe {
            core::slice::from_raw_parts_mut(pa.0 as *mut u8, 4096)
        }
    }
    pub fn get_mut<T>(&self) -> &'static mut T {
        let pa: PhysAddr = self.clone().into();
        unsafe {
            (pa.0 as *mut T).as_mut().unwrap()
        }
    }
}

impl VirtPageNum {
    pub fn indexes(&self) -> [usize; 3]{
        let mut vpn = self.0;
        let mut idx = [0usize; 3];
        for i in (0..3).rev(){
            idx[i] = vpn & 511;
            vpn >>= 9;
        }
        idx
    }
}