use std::marker::PhantomData;

use crate::util::ParentSet;
use crate::util::galois_field_2m_elem::GaloisField2mElement;

type Result<T> = std::result::Result<T, &'static str>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GaloisField2m<'a> {
    ppoly: u16,
    _marker: PhantomData<&'a ()>,
}

impl<'a> GaloisField2m<'a> {
    pub fn new(ppoly: u16) -> Result<Self> {
        if ppoly < 0b10 {
            Err("ppoly error")
        } else {
            // assumption: ppoly is primitive
            Ok(Self {
                ppoly,
                _marker: PhantomData,
            })
        }
    }

    pub fn ppoly_deg(&self) -> u8 {
        (16 - self.ppoly.leading_zeros() - 1) as u8
    }

    pub fn order(&self) -> u16 {
        1 << self.ppoly_deg()
    }

    pub fn elem<'s>(&'s self, val: u16) -> Result<<Self as ParentSet>::ElementType<'s>> {
        <Self as ParentSet>::ElementType::new(&self, val)
    }

    pub fn zero<'s>(&'s self) -> <Self as ParentSet>::ElementType<'s> {
        self.elem(0).unwrap()
    }

    pub fn one<'s>(&'s self) -> <Self as ParentSet>::ElementType<'s> {
        self.elem(1).unwrap()
    }

    // assumption: ppoly is primitive
    pub fn primitive_element<'s>(&'s self) -> <Self as ParentSet>::ElementType<'s> {
        self.elem(2).unwrap()
    }

    pub fn modulus(&self) -> u16 {
        self.ppoly
    }
}

impl<'a> ParentSet for GaloisField2m<'a> {
    type ElementType<'b>
        = GaloisField2mElement<'b>
    where
        Self: 'b;
}
