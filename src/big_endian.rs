use core::fmt;

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Be32(u32);

impl From<Be32> for u32 {
    fn from(be: Be32) -> Self {
        u32::from_be(be.0)
    }
}

impl fmt::Debug for Be32 {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        u32::from(*self).fmt(f)
    }
}
