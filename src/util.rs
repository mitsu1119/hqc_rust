use std::{
    fmt::{Debug, Display},
    ops::{Add, Mul},
};

pub mod galois_field_2m;

pub trait GaloisField:
    Debug + Display + Copy + Default + Add<Output = Self> + Mul<Output = Self>
{
    const SIZE: u16;
    fn zero() -> Self;
    fn one() -> Self;
}
