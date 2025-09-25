use crate::util::GaloisField;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GaloisField2m<const PPOLY: u16> {
    value: u16,
}

impl<const PPOLY: u16> GaloisField for GaloisField2m<PPOLY> {}

impl<const PPOLY: u16> Default for GaloisField2m<PPOLY> {
    fn default() -> Self {
        Self { value: 0 }
    }
}

impl<const PPOLY: u16> GaloisField2m<PPOLY> {
    pub fn new<'a>(value: u16) -> Result<Self, &'a str> {
        if value.leading_zeros() <= PPOLY.leading_zeros() {
            Err("degree of value must be smaller than degree of primitive polynomial")
        } else {
            Ok(Self { value })
        }
    }

    fn add(&mut self, rhs: Self) {
        self.value ^= rhs.value;
    }

    fn xtime(&mut self) {
        match PPOLY {
            0 | 0b10 => {
                self.value = 0;
            }
            1 | 0b11 => {}
            _ => {
                self.value <<= 1;
                if ((self.value >> (16 - PPOLY.leading_zeros() - 1)) & 1) == 1 {
                    self.value ^= PPOLY;
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
}

impl<const PPOLY: u16> AddAssign for GaloisField2m<PPOLY> {
    fn add_assign(&mut self, rhs: Self) {
        self.add(rhs)
    }
}

impl<const PPOLY: u16> Add for GaloisField2m<PPOLY> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res += rhs;
        res
    }
}

impl<const PPOLY: u16> MulAssign for GaloisField2m<PPOLY> {
    fn mul_assign(&mut self, rhs: Self) {
        self.mul(rhs)
    }
}

impl<const PPOLY: u16> Mul for GaloisField2m<PPOLY> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut res = self;
        res *= rhs;
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::util::galois_field_2m::GaloisField2m;

    #[test]
    fn add() {
        let tests = [
            (
                GaloisField2m::<0b100011101>::new(0b11).unwrap(),
                GaloisField2m::<0b100011101>::new(0b111).unwrap(),
            ),
            (
                GaloisField2m::<0b100011101>::new(0b11001).unwrap(),
                GaloisField2m::<0b100011101>::new(0b10111).unwrap(),
            ),
            (
                GaloisField2m::<0b100011101>::new(0b10000).unwrap(),
                GaloisField2m::<0b100011101>::new(0b100000).unwrap(),
            ),
        ];
        let res = [
            GaloisField2m::<0b100011101>::new(0b100).unwrap(),
            GaloisField2m::<0b100011101>::new(0b1110).unwrap(),
            GaloisField2m::<0b100011101>::new(0b110000).unwrap(),
        ];

        for ((x, y), r) in tests.into_iter().zip(res) {
            assert_eq!(x + y, r);
        }
    }

    #[test]
    fn xtime() {
        let tests = [
            (
                GaloisField2m::<0b100011101>::new(0b1000110).unwrap(),
                GaloisField2m::<0b100011101>::new(0b10).unwrap(),
            ),
            (
                GaloisField2m::<0b100011101>::new(0b1111).unwrap(),
                GaloisField2m::<0b100011101>::new(0b10).unwrap(),
            ),
            (
                GaloisField2m::<0b100011101>::new(0b110011).unwrap(),
                GaloisField2m::<0b100011101>::new(0b10).unwrap(),
            ),
        ];
        let res = [
            GaloisField2m::<0b100011101>::new(0b10001100).unwrap(),
            GaloisField2m::<0b100011101>::new(0b11110).unwrap(),
            GaloisField2m::<0b100011101>::new(0b1100110).unwrap(),
        ];

        for ((x, y), r) in tests.into_iter().zip(res) {
            assert_eq!(x * y, r);
        }
    }

    #[test]
    fn mul() {
        let tests = [
            (
                GaloisField2m::<0b100011101>::new(0b111111).unwrap(),
                GaloisField2m::<0b100011101>::new(0b110101).unwrap(),
            ),
            (
                GaloisField2m::<0b100011101>::new(0b11101000).unwrap(),
                GaloisField2m::<0b100011101>::new(0b10010110).unwrap(),
            ),
            (
                GaloisField2m::<0b100011101>::new(0b10110).unwrap(),
                GaloisField2m::<0b100011101>::new(0b11110).unwrap(),
            ),
        ];
        let res = [
            GaloisField2m::<0b100011101>::new(0b10100111).unwrap(),
            GaloisField2m::<0b100011101>::new(0b1000001).unwrap(),
            GaloisField2m::<0b100011101>::new(0b10111001).unwrap(),
        ];

        for ((x, y), r) in tests.into_iter().zip(res) {
            assert_eq!(x * y, r);
        }
    }
}
