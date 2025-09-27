use crate::code::Code;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hadamard {
    n: usize,
    m: u8,
    k: u8,
}

impl Hadamard {
    fn new(m: u8) -> Self {
        assert!(m > 0);
        assert!(m < u8::MAX);

        let n = ((1 << m) >> 3) + 1;
        let k = (m + 1) >> 3;
        Self { n, m, k }
    }
}
