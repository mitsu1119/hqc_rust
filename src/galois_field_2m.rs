use std::ops::{Add, AddAssign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GaloisField2m<const PPOLY: u16> {
    value: u16,
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

#[cfg(test)]
mod tests {
    use crate::galois_field_2m::GaloisField2m;

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
}
