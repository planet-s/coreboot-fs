use crate::Be32;
use plain::Plain;

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Header {
    pub magic: [u8; 4],
    pub version: Be32,
    pub romsize: Be32,
    pub bootblocksize: Be32,
    pub align: Be32,
    pub offset: Be32,
    pub arch: Be32,
    pub pad: Be32,
}

unsafe impl Plain for Header {}
