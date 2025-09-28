use crate::util::galois_field_2m::GaloisField2m;

pub struct HQCParam<'a> {
    pub rs_n: usize,
    pub rs_k: usize,
    pub rs_symbol_field: GaloisField2m<'a>,
    pub rs_genpoly: Vec<u16>,
    pub hadamard_multiplicity: u8,
}

impl<'a> HQCParam<'a> {
    pub fn new(rs_n: usize, rs_k: usize, rs_genpoly: Vec<u16>, hadamard_multiplicity: u8) -> Self {
        Self {
            rs_n,
            rs_k,
            rs_symbol_field: GaloisField2m::new(0b100011101).unwrap(),
            rs_genpoly,
            hadamard_multiplicity,
        }
    }
}
