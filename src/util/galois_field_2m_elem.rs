use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign},
};

use crate::util::{Element, galois_field_2m::GaloisField2m};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GaloisField2mElement<'a> {
    parent: &'a <Self as Element>::ParentType,
    value: u16,
}

impl<'a> Display for GaloisField2mElement<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0b}", self.value)
    }
}

impl<'a> Element for GaloisField2mElement<'a> {
    type ParentType = GaloisField2m<'a>;
}

impl<'a> GaloisField2mElement<'a> {
    pub fn new(
        parent: &'a <Self as Element>::ParentType,
        value: u16,
    ) -> Result<Self, &'static str> {
        if value.leading_zeros() <= parent.modulus().leading_zeros() {
            Err("degree of value must be smaller than degree of primitive polynomial")
        } else {
            Ok(Self { parent, value })
        }
    }

    pub fn parent(&self) -> &'a <Self as Element>::ParentType {
        self.parent
    }

    pub fn value(&self) -> u16 {
        self.value
    }

    fn add(&mut self, rhs: Self) {
        self.value ^= rhs.value;
    }

    fn xtime(&mut self) {
        match self.parent.modulus() {
            0 | 0b10 => {
                self.value = 0;
            }
            1 | 0b11 => {}
            _ => {
                self.value <<= 1;
                if ((self.value >> self.parent.ppoly_deg()) & 1) == 1 {
                    self.value ^= self.parent.modulus();
                }
            }
        }
    }

    fn mul(&mut self, rhs: Self) {
        match rhs.value {
            0 => {
                self.value = 0;
            }
            1 => {}
            0b10 => {
                self.xtime();
            }
            _ => {
                let mut base = *self;
                let mut rhs = rhs;
                self.value = 0;
                while rhs.value > 0 {
                    if (rhs.value & 1) == 1 {
                        *self += base;
                    }
                    base.xtime();
                    rhs.value >>= 1;
                }
            }
        }
    }

    fn inv(&mut self) {
        // self^x = self^{-1}
        let mut x: u16 = (1 << self.parent.ppoly_deg()) - 2;

        match x {
            0 => self.value = 1,
            1 => {}
            _ => {
                let mut base = *self;
                self.value = 1;
                while x > 0 {
                    if (x & 1) == 1 {
                        *self *= base;
                    }
                    base *= base;
                    x >>= 1;
                }
            }
        }
    }

    fn div(&mut self, rhs: Self) {
        assert_ne!(rhs.value, 0);
        if rhs.value == 1 {
            return;
        }
        let mut rhs = rhs;
        rhs.inv();
        *self *= rhs;
    }
}

impl AddAssign for GaloisField2mElement<'_> {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.parent, rhs.parent);
        self.add(rhs)
    }
}

impl Add for GaloisField2mElement<'_> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.parent, rhs.parent);
        let mut res = self;
        res += rhs;
        res
    }
}

impl MulAssign for GaloisField2mElement<'_> {
    fn mul_assign(&mut self, rhs: Self) {
        assert_eq!(self.parent, rhs.parent);
        self.mul(rhs)
    }
}

impl Mul for GaloisField2mElement<'_> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.parent, rhs.parent);
        let mut res = self;
        res *= rhs;
        res
    }
}

impl DivAssign for GaloisField2mElement<'_> {
    fn div_assign(&mut self, rhs: Self) {
        assert_eq!(self.parent, rhs.parent);
        self.div(rhs);
    }
}

impl Div for GaloisField2mElement<'_> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        assert_eq!(self.parent, rhs.parent);
        let mut res = self;
        res /= rhs;
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::util::galois_field_2m::GaloisField2m;

    #[test]
    fn add() {
        let field = GaloisField2m::new(0b100011101).unwrap();
        let tests = [
            (field.elem(0b11).unwrap(), field.elem(0b111).unwrap()),
            (field.elem(0b11001).unwrap(), field.elem(0b10111).unwrap()),
            (field.elem(0b10000).unwrap(), field.elem(0b100000).unwrap()),
        ];
        let res = [
            field.elem(0b100).unwrap(),
            field.elem(0b1110).unwrap(),
            field.elem(0b110000).unwrap(),
        ];

        for ((x, y), r) in tests.into_iter().zip(res) {
            assert_eq!(x + y, r);
        }
    }

    #[test]
    fn xtime() {
        let field = GaloisField2m::new(0b100011101).unwrap();
        let tests = [
            (field.elem(0b1000110).unwrap(), field.elem(0b10).unwrap()),
            (field.elem(0b1111).unwrap(), field.elem(0b10).unwrap()),
            (field.elem(0b110011).unwrap(), field.elem(0b10).unwrap()),
        ];
        let res = [
            field.elem(0b10001100).unwrap(),
            field.elem(0b11110).unwrap(),
            field.elem(0b1100110).unwrap(),
        ];

        for ((x, y), r) in tests.into_iter().zip(res) {
            assert_eq!(x * y, r);
        }
    }

    #[test]
    fn mul() {
        let field = GaloisField2m::new(0b100011101).unwrap();
        let tests = [
            (field.elem(0b111111).unwrap(), field.elem(0b110101).unwrap()),
            (
                field.elem(0b11101000).unwrap(),
                field.elem(0b10010110).unwrap(),
            ),
            (field.elem(0b10110).unwrap(), field.elem(0b11110).unwrap()),
        ];
        let res = [
            field.elem(0b10100111).unwrap(),
            field.elem(0b1000001).unwrap(),
            field.elem(0b10111001).unwrap(),
        ];

        for ((x, y), r) in tests.into_iter().zip(res) {
            assert_eq!(x * y, r);
        }
    }

    #[test]
    fn div() {
        let field = GaloisField2m::new(0b100011101).unwrap();
        let tests = [
            (field.elem(0b0).unwrap(), field.elem(0b110101).unwrap()),
            (field.elem(0b1).unwrap(), field.elem(0b110101).unwrap()),
            (field.elem(0b111111).unwrap(), field.elem(0b110101).unwrap()),
            (
                field.elem(0b11101000).unwrap(),
                field.elem(0b10010110).unwrap(),
            ),
            (field.elem(0b10110).unwrap(), field.elem(0b11110).unwrap()),
        ];
        let res = [
            field.elem(0b0).unwrap(),
            field.elem(0b11000011).unwrap(),
            field.elem(0b11001100).unwrap(),
            field.elem(0b10110001).unwrap(),
            field.elem(0b1100011).unwrap(),
        ];

        for ((x, y), r) in tests.into_iter().zip(res) {
            assert_eq!(x / y, r);
        }
    }
}
