pub enum MemoryType {
    Normal,  // real cacheable memory
    Device,  // peripherals
}

pub struct Region {
    pub base: usize,
    pub size: usize,
    pub kind: MemoryType,
}

pub const MEMORY_MAP: &[Region] = &[
    Region { base: 0x0000_0000, size: 0x4000_0000, kind: MemoryType::Normal },
    Region { base: 0xC000_0000, size: 0x4000_0000, kind: MemoryType::Device },
    // TODO: discover full RAM at runtime via DTB (x0) / mailbox
    // For now we will only use the know 1GB ram which is certain for all models
    // We must parse DTB or get from a driver which is the actual memory size in the future
    // These numbers 0xC000_0000 for pheriferals and 0x4000_0000 for normal memory (which may further expand)
    // come from raspberrypi4 manual
];
