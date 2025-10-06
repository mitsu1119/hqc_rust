use crate::hqc::{hqc_hash::HQCHash, xof::XOF};

pub struct HQC_PKE;

impl HQC_PKE {
    fn generate_seeds(seed: &[u8]) -> ([u8; 32], [u8; 32]) {
        let pke_seeds = HQCHash::I(seed);

        let mut seed_pkedk = [0u8; 32];
        let mut seed_pkeek = [0u8; 32];
        seed_pkedk.copy_from_slice(&pke_seeds[..32]);
        seed_pkeek.copy_from_slice(&pke_seeds[32..]);

        (seed_pkedk, seed_pkeek)
    }

    fn generate_dk(seed: &[u8]) {
        let ctx = XOF::new(seed);
    }

    pub fn keygen(seed: &[u8]) {
        let (seed_pkedk, seed_pkeek) = Self::generate_seeds(seed);
    }
}

#[cfg(test)]
mod tests {
    use crate::hqc::hqc::HQC_PKE;

    #[test]
    fn generate_seeds() {
        // seed_pke: 81313de32ad36c4779865fe66dda28aa9228818c0f3e2fa0348ef16e377d1049
        let seed = [
            129, 49, 61, 227, 42, 211, 108, 71, 121, 134, 95, 230, 109, 218, 40, 170, 146, 40, 129,
            140, 15, 62, 47, 160, 52, 142, 241, 110, 55, 125, 16, 73,
        ];
        let (seed_dk, seed_ek) = HQC_PKE::generate_seeds(&seed);

        // seed_dk: 12daf031bdc7fc592e0003a21eefa9a1019539abccc8f67075947cbfeaac98c5
        // seed_ek: ef2b80f46f3a6437b4d869bb38bdd6004bff72bcd0ceb139b4b8d47301f4fcb1
        let res = (
            [
                18, 218, 240, 49, 189, 199, 252, 89, 46, 0, 3, 162, 30, 239, 169, 161, 1, 149, 57,
                171, 204, 200, 246, 112, 117, 148, 124, 191, 234, 172, 152, 197,
            ],
            [
                239, 43, 128, 244, 111, 58, 100, 55, 180, 216, 105, 187, 56, 189, 214, 0, 75, 255,
                114, 188, 208, 206, 177, 57, 180, 184, 212, 115, 1, 244, 252, 177,
            ],
        );

        assert_eq!((seed_dk, seed_ek), res);
    }

    #[test]
    fn keygen_hqc1() {
        // seed_pke: 81313de32ad36c4779865fe66dda28aa9228818c0f3e2fa0348ef16e377d1049
        let seed = [
            129, 49, 61, 227, 42, 211, 108, 71, 121, 134, 95, 230, 109, 218, 40, 170, 146, 40, 129,
            140, 15, 62, 47, 160, 52, 142, 241, 110, 55, 125, 16, 73,
        ];
        HQC_PKE::keygen(&seed);

        panic!();
    }
}
