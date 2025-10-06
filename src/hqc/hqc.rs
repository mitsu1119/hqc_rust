use crate::hqc::{hqc_hash::HQCHash, hqc_param::HQCParam, xof::XOF};

#[allow(non_camel_case_types)]
pub struct HQC_PKE<'a> {
    param: HQCParam<'a>,
}

impl<'a> HQC_PKE<'a> {
    pub fn new(param: HQCParam<'a>) -> Self {
        Self { param }
    }

    pub fn hqc1() -> Self {
        Self {
            param: HQCParam::hqc1(),
        }
    }

    pub fn hqc3() -> Self {
        Self {
            param: HQCParam::hqc3(),
        }
    }

    pub fn hqc5() -> Self {
        Self {
            param: HQCParam::hqc5(),
        }
    }

    fn generate_seeds(seed: &[u8]) -> ([u8; 32], [u8; 32]) {
        let pke_seeds = HQCHash::I(seed);

        let mut seed_pkedk = [0u8; 32];
        let mut seed_pkeek = [0u8; 32];
        seed_pkedk.copy_from_slice(&pke_seeds[..32]);
        seed_pkeek.copy_from_slice(&pke_seeds[32..]);

        (seed_pkedk, seed_pkeek)
    }

    fn sample_fixed_weight_vect_indices(&self, ctx: &mut XOF, weight: u8) -> Vec<usize> {
        let mut res = vec![];
        while res.len() != weight as usize {
            let bytes = ctx.get_bytes(4);
            let index_orig = ((bytes[3] as usize) << 24)
                | ((bytes[2] as usize) << 16)
                | ((bytes[1] as usize) << 8)
                | bytes[0] as usize;
            let index = index_orig % self.param.n;

            if !res.contains(&index) {
                res.push(index);
            }
        }
        res
    }

    fn sample_fixed_weight_vect(&self, ctx: &mut XOF, weight: u8) -> Vec<u8> {
        let indices = self.sample_fixed_weight_vect_indices(ctx, weight);

        let mut res = vec![0u8; self.param.n.div_ceil(8)];
        for index in indices.iter() {
            let res_ind = index >> 3;
            let bit_pos = index - (res_ind << 3);
            let bit = 1 << bit_pos;
            res[res_ind] |= bit;
        }
        res
    }

    fn generate_dk(&self, seed: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut ctx = XOF::new(seed);
        let y = self.sample_fixed_weight_vect(&mut ctx, self.param.omega_re);
        let x = self.sample_fixed_weight_vect(&mut ctx, self.param.omega_re);

        (x, y)
    }

    pub fn keygen(&self, seed: &[u8]) {
        let (seed_pkedk, seed_pkeek) = Self::generate_seeds(seed);

        let (x, y) = self.generate_dk(&seed_pkedk);
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
        let hqc1 = HQC_PKE::hqc1();
        hqc1.keygen(&seed);

        panic!();
    }
}
