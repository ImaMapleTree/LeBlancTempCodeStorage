use crate::leblanc::rustblanc::memory::megabyte::Megabyte;

pub static REPORT_ALL_ERRORS: bool = true;
pub const SSM_KB: usize = 2; // 256

/// Enable to allow heap to double in size if it runs out of room. Caps at `HMX_MB`
pub const ALLOW_HEAP_REALLOC: bool = true;
pub const ALLOW_HEAP_REALLOC_FOR_WILD: bool = false;


/// Default memory size for Heap (in megabytes), 1024
pub const HDEF_MB: Megabyte = Megabyte::new(1024);

/// Maximum heap size (heap doubles only if heap doubling is enabled)
#[cfg(feature = "bench")]
pub const HMX_MB: Megabyte = Megabyte::new(4096*2);

#[cfg(not(feature = "bench"))]
pub const HMX_MB: Megabyte = Megabyte::new(4096);
