use crate::Be32;
use core::{mem, slice};
use plain::Plain;

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct File {
    pub magic: [u8; 8],
    pub len: Be32,
    pub kind: Be32,
    pub checksum: Be32,
    pub offset: Be32,
}

impl File {
    pub fn name(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                (self as *const Self as *const u8).add(mem::size_of::<Self>()),
                u32::from(self.offset) as usize - mem::size_of::<Self>()
            )
        }
    }

    pub fn data(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(
                (self as *const Self as *const u8).add(u32::from(self.offset) as usize),
                u32::from(self.len) as usize
            )
        }
    }
}

unsafe impl Plain for File {}
