pub enum MemoryType {
    Normal,  // real cacheable memory
    Device,  // peripherals
}

pub struct Region {
    pub base: usize,
    pub size: usize,
    pub kind: MemoryType,
}
