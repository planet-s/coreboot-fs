use plain::Plain;

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct FmapArea {
    pub offset: u32,
    pub size: u32,
    pub name: [u8; 32],
    pub flags: u16,
}

#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Fmap {
    pub signature: [u8; 8],
    pub ver_major: u8,
    pub ver_minor: u8,
    pub base: u64,
    pub size: u32,
    pub name: [u8; 32],
    pub nareas: u16,
}

impl Fmap {
    pub fn area(&self, i: u16) -> &FmapArea {
        unsafe {
            &*((self as *const Self).add(1) as *const FmapArea).add(i as usize)
        }
    }
}

unsafe impl Plain for Fmap {}
