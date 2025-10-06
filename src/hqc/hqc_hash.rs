use sha3::{Digest, Sha3_512};

#[allow(non_camel_case_types)]
pub struct HQCHash {}

impl HQCHash {
    const HQC_I_DOMAIN: u8 = 2;

    #[allow(non_snake_case)]
    pub fn I(data: &[u8]) -> [u8; 64] {
        let mut h = Sha3_512::new();
        h.update(data);
        h.update(&[Self::HQC_I_DOMAIN]);
        let out = h.finalize();
        let mut res = [0u8; 64];
        res.copy_from_slice(&out);
        res
    }
}
