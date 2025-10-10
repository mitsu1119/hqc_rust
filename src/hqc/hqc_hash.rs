use sha3::{Digest, Sha3_256, Sha3_512};

pub struct HQCHash {}

impl HQCHash {
    const HQC_G_DOMAIN: u8 = 0;
    const HQC_H_DOMAIN: u8 = 1;
    const HQC_I_DOMAIN: u8 = 2;
    const HQC_J_DOMAIN: u8 = 3;

    #[allow(non_snake_case)]
    pub fn G(data: &[u8]) -> [u8; 64] {
        let mut h = Sha3_512::new();
        h.update(data);
        h.update(&[Self::HQC_G_DOMAIN]);
        let out = h.finalize();
        let mut res = [0u8; 64];
        res.copy_from_slice(&out);
        res
    }

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

    #[allow(non_snake_case)]
    pub fn H(data: &[u8]) -> [u8; 32] {
        let mut h = Sha3_256::new();
        h.update(data);
        h.update(&[Self::HQC_H_DOMAIN]);
        let out = h.finalize();
        let mut res = [0u8; 32];
        res.copy_from_slice(&out);
        res
    }

    #[allow(non_snake_case)]
    pub fn J(data: &[u8]) -> [u8; 32] {
        let mut h = Sha3_256::new();
        h.update(data);
        h.update(&[Self::HQC_J_DOMAIN]);
        let out = h.finalize();
        let mut res = [0u8; 32];
        res.copy_from_slice(&out);
        res
    }
}
