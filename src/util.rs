use std::ops::{Add, Mul};

pub mod galois_field_2m;

pub trait GaloisField: Copy + Default + Add<Output = Self> + Mul<Output = Self> {
    const SIZE: u16;
}
