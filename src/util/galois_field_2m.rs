use crate::util::ParentSet;
use crate::util::galois_field_2m_elem::GaloisField2mElement;

type Result<T> = std::result::Result<T, &'static str>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GaloisField2m {
    ppoly: u16,
}

impl GaloisField2m {
    pub fn new(ppoly: u16) -> Result<Self> {
        if ppoly < 0b10 {
            Err("ppoly error")
        } else {
            // assumption: ppoly is primitive
            Ok(Self { ppoly })
        }
    }

    pub fn ppoly_deg(&self) -> u8 {
        (16 - self.ppoly.leading_zeros() - 1) as u8
    }

    pub fn order(&self) -> u16 {
        1 << self.ppoly_deg()
    }

    pub fn elem(&self) -> <Self as ParentSet>::ElementType {
        0
    }
}

impl ParentSet for GaloisField2m {
    type ElementType = u16;
}
