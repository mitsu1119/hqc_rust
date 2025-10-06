use crate::util::galois_field_2m::GaloisField2m;

pub struct HQCRSParam<'a> {
    pub n: usize,
    pub k: usize,
    pub rs_symbol_field: GaloisField2m<'a>,
    pub rs_genpoly: Vec<u8>,
}

impl<'a> HQCRSParam<'a> {
    pub fn new(n: usize, k: usize, rs_genpoly: Vec<u8>) -> Self {
        Self {
            n,
            k,
            rs_symbol_field: GaloisField2m::new(0b100011101).unwrap(),
            rs_genpoly,
        }
    }

    pub fn new_rss1() -> Self {
        let coeffs = [
            89, 69, 153, 116, 176, 117, 111, 75, 73, 233, 242, 233, 65, 210, 21, 139, 103, 173, 67,
            118, 105, 210, 174, 110, 74, 69, 228, 82, 255, 181, 1,
        ];
        Self {
            n: 46,
            k: 16,
            rs_symbol_field: GaloisField2m::new(0b100011101).unwrap(),
            rs_genpoly: coeffs.to_vec(),
        }
    }

    pub fn new_rss3() -> Self {
        let coeffs = [
            45, 216, 239, 24, 253, 104, 27, 40, 107, 50, 163, 210, 227, 134, 224, 158, 119, 13,
            158, 1, 238, 164, 82, 43, 15, 232, 246, 142, 50, 189, 29, 232, 1,
        ];
        Self {
            n: 56,
            k: 24,
            rs_symbol_field: GaloisField2m::new(0b100011101).unwrap(),
            rs_genpoly: coeffs.to_vec(),
        }
    }

    pub fn new_rss5() -> Self {
        let coeffs = [
            49, 167, 49, 39, 200, 121, 124, 91, 240, 63, 148, 71, 150, 123, 87, 101, 32, 215, 159,
            71, 201, 115, 97, 210, 186, 183, 141, 217, 123, 12, 31, 243, 180, 219, 152, 239, 99,
            141, 4, 246, 191, 144, 8, 232, 47, 27, 141, 178, 130, 64, 124, 47, 39, 188, 216, 48,
            199, 187, 1,
        ];
        Self {
            n: 90,
            k: 32,
            rs_symbol_field: GaloisField2m::new(0b100011101).unwrap(),
            rs_genpoly: coeffs.to_vec(),
        }
    }
}

pub struct HQCParam<'a> {
    pub rs_param: HQCRSParam<'a>,
    pub hadamard_multiplicity: u8,
    pub omega_re: u8,
    pub n: usize,
}

impl<'a> HQCParam<'a> {
    pub fn hqc1() -> Self {
        Self {
            rs_param: HQCRSParam::new_rss1(),
            hadamard_multiplicity: 3,
            omega_re: 75,
            n: 17669,
        }
    }

    pub fn hqc3() -> Self {
        Self {
            rs_param: HQCRSParam::new_rss3(),
            hadamard_multiplicity: 5,
            omega_re: 114,
            n: 35851,
        }
    }

    pub fn hqc5() -> Self {
        Self {
            rs_param: HQCRSParam::new_rss1(),
            hadamard_multiplicity: 5,
            omega_re: 149,
            n: 57637,
        }
    }
}
