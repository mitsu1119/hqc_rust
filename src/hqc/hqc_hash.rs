use sha3::{Digest, Sha3_512};

pub struct HQCHash {}

impl HQCHash {
    const HQC_G_DOMAIN: [u8; 1] = [0];
    const HQC_H_DOMAIN: [u8; 1] = [1];
    const HQC_I_DOMAIN: [u8; 1] = [2];
    const HQC_J_DOMAIN: [u8; 1] = [3];

    pub fn G(str: &[u8]) -> [u8; 64] {
        let mut h = Sha3_512::new();
        h.update(str);
        h.update(&Self::HQC_G_DOMAIN);
        let out = h.finalize();
        let mut res = [0u8; 64];
        res.copy_from_slice(&out);
        res
    }
}
