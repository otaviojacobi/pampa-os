use crate::mm::MemoryType;

#[repr(align(4096))]
pub struct PageTable {
    entries: [u64; 512],
}

static mut L1: PageTable = PageTable { entries: [0; 512] };

pub fn initialize_table() {
    for i in 0..512 {
        unsafe {
            L1.entries[i] = block_descriptor(ph, kind);
        }
    }
}

fn block_descriptor(physical_address: usize, kind: MemoryType) -> u64 {
    return 0
}
