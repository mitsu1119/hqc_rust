use crate::{
    code::Code,
    hqc::{hqc_hash::HQCHash, hqc_param::HQCParam, xof::XOF},
};

#[allow(non_camel_case_types)]
pub struct HQC_PKE<'a> {
    param: HQCParam<'a>,
}

impl<'a> HQC_PKE<'a> {
    pub fn new(param: HQCParam<'a>) -> Self {
        Self { param }
    }

    pub fn hqc1() -> Self {
        Self::new(HQCParam::hqc1())
    }

    pub fn hqc3() -> Self {
        Self::new(HQCParam::hqc3())
    }

    pub fn hqc5() -> Self {
        Self::new(HQCParam::hqc5())
    }

    fn get_bit_from_bitvec(bits: &Vec<u8>, bit_position: usize) -> u8 {
        let index = bit_position >> 3;
        let position_in_u8 = bit_position % 8;
        let mask = 1 << position_in_u8;
        (bits[index] & mask) >> position_in_u8
    }

    fn bit_flip_in_bitvec(bits: &mut Vec<u8>, bit_position: usize) {
        let index = bit_position >> 3;
        let position_in_u8 = bit_position % 8;
        let mask = 1 << position_in_u8;
        bits[index] ^= mask;
    }

    fn vec_mul(&self, u: Vec<u8>, v: Vec<u8>) -> Vec<u8> {
        let mut res = vec![0u8; u.len()];
        for i in 0..self.param.n {
            for j in 0..self.param.n {
                let k = (i + j) % self.param.n;
                let ui = Self::get_bit_from_bitvec(&u, i);
                let vj = Self::get_bit_from_bitvec(&v, j);
                let uv = ui & vj;
                if uv == 0 {
                    continue;
                }
                Self::bit_flip_in_bitvec(&mut res, k);
            }
        }
        res
    }

    fn truncate(&self, v: &mut Vec<u8>) {
        let new_size = (self.param.n1n2 + 7) >> 3;
        let bit_pos = self.param.n1n2 % 8;

        if bit_pos != 0 {
            let mask = (1u8 << bit_pos) - 1;
            v[self.param.n1n2 >> 3] &= mask;
        }

        v.resize(new_size, 0);
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
            let bit_pos = index % 8;
            let bit = 1 << bit_pos;
            res[res_ind] |= bit;
        }
        res
    }

    fn sample_vec(&self, ctx: &mut XOF) -> Vec<u8> {
        let mut res = vec![0u8; self.param.n.div_ceil(8)];
        for i in 0..res.len() {
            res[i] = ctx.get_bytes(1)[0];
        }

        let n_ind = self.param.n >> 3;
        let n_bit_pos = self.param.n - (n_ind << 3);
        let bit = 1usize << (n_bit_pos + 1);
        let mask = bit - 1;

        let len = res.len();
        res[len - 1] &= mask as u8;

        res
    }

    fn generate_dk(&self, seed: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut ctx = XOF::new(seed);
        let y = self.sample_fixed_weight_vect(&mut ctx, self.param.omega_re);
        let x = self.sample_fixed_weight_vect(&mut ctx, self.param.omega_re);

        (x, y)
    }

    fn generate_ek(&self, seed: &[u8], x: Vec<u8>, y: Vec<u8>) -> Vec<u8> {
        let mut ctx = XOF::new(seed);

        let h = self.sample_vec(&mut ctx);
        let mut s = self.vec_mul(h, y);
        for i in 0..s.len() {
            s[i] ^= x[i];
        }

        s
    }

    pub fn keygen(&self, seed: &[u8]) -> (([u8; 32], Vec<u8>), [u8; 32]) {
        let (seed_pkedk, seed_pkeek) = Self::generate_seeds(seed);
        let (x, y) = self.generate_dk(&seed_pkedk);
        let s = self.generate_ek(&seed_pkeek, x, y);

        let ek = (seed_pkeek, s);
        let dk = seed_pkedk;

        (ek, dk)
    }

    fn calc_trunc(&self, s: Vec<u8>, r2: Vec<u8>, e: Vec<u8>) -> Vec<u8> {
        let mut tr = self.vec_mul(s, r2);
        for i in 0..tr.len() {
            tr[i] ^= e[i];
        }
        self.truncate(&mut tr);
        tr
    }

    pub fn encrypt(&self, ek: ([u8; 32], Vec<u8>), m: Vec<u8>, theta: &[u8]) -> (Vec<u8>, Vec<u8>) {
        // regenerate encryption keys
        let seed = ek.0;
        let s = ek.1;
        let mut ctx = XOF::new(&seed);
        let h = self.sample_vec(&mut ctx);

        // compute cipher text
        let mut ctx = XOF::new(&theta);
        let r2 = self.sample_fixed_weight_vect(&mut ctx, self.param.omega_re);
        let e = self.sample_fixed_weight_vect(&mut ctx, self.param.omega_re);
        let r1 = self.sample_fixed_weight_vect(&mut ctx, self.param.omega_re);

        let mut u = self.vec_mul(h, r2.clone());
        for i in 0..u.len() {
            u[i] ^= r1[i];
        }

        let encoder = self.param.gen_hqc_code();
        let code = encoder.encode(m);
        let trunc = self.calc_trunc(s, r2, e);
        assert_eq!(code.len(), trunc.len());
        let mut v = code;
        for i in 0..v.len() {
            v[i] ^= trunc[i];
        }

        (u, v)
    }

    pub fn decrypt(&self, dk: [u8; 32], c: (Vec<u8>, Vec<u8>)) -> Vec<u8> {
        let mut ctx = XOF::new(&dk);
        let y = self.sample_fixed_weight_vect(&mut ctx, self.param.omega_re);
        let (u, v) = c;

        let code = {
            let mut tmp = self.vec_mul(u, y);
            self.truncate(&mut tmp);
            assert_eq!(v.len(), tmp.len());
            for i in 0..tmp.len() {
                tmp[i] ^= v[i];
            }
            tmp
        };
        let decoder = self.param.gen_hqc_code();
        let m = decoder.decode(code);
        m
    }
}

#[cfg(test)]
mod tests {
    use crate::{hqc::hqc::HQC_PKE, util::kat_parser::KATParser};

    #[test]
    fn enc_dec() {
        let mut parser = KATParser::new("kats/hqc-1/intermediates_values").expect("");

        let kat_seed_pke = parser.bytes_after("seed_pke: ").expect("").unwrap();
        let _ = parser.bytes_after("ek_kem: ").expect("").unwrap();
        let kat_m = parser.bytes_after("m: ").expect("").unwrap();
        let kat_theta = parser.bytes_after("theta: ").expect("").unwrap();

        let hqc1 = HQC_PKE::hqc1();
        let (ek, dk) = hqc1.keygen(&kat_seed_pke);
        let c = hqc1.encrypt(ek, kat_m.clone(), &kat_theta);

        let m_prime = hqc1.decrypt(dk, c);
        assert_eq!(kat_m, m_prime);
    }

    #[test]
    fn calc_trunc() {
        let mut parser = KATParser::new("kats/hqc-1/intermediates_values").expect("");

        let _ = parser.line_after("### ENCAPS");
        let kat_s = parser.bytes_after("s: ").expect("").unwrap();
        let kat_r1 = parser.bytes_after("r1: ").expect("").unwrap();
        let kat_r2 = parser.bytes_after("r2: ").expect("").unwrap();
        let kat_e = parser.bytes_after("e: ").expect("").unwrap();
        let kat_trunc = parser
            .bytes_after("Truncate(s.r2 + e): ")
            .expect("")
            .unwrap();

        let hqc1 = HQC_PKE::hqc1();
        let trunc = hqc1.calc_trunc(kat_s, kat_r2, kat_e);
        assert_eq!(trunc, kat_trunc);
    }

    #[test]
    fn vec_mul() {
        let mut parser = KATParser::new("kats/hqc-1/intermediates_values").expect("");

        let kat_y = parser.bytes_after("y: ").expect("").unwrap();
        let kat_x = parser.bytes_after("x: ").expect("").unwrap();
        let kat_h = parser.bytes_after("h: ").expect("").unwrap();
        let kat_s = parser.bytes_after("s: ").expect("").unwrap();

        let hqc1 = HQC_PKE::hqc1();
        let mut w = hqc1.vec_mul(kat_h, kat_y);
        assert_eq!(w.len(), kat_x.len());
        for i in 0..w.len() {
            w[i] ^= kat_x[i];
        }

        assert_eq!(w, kat_s);
    }

    #[test]
    fn generate_seeds() {
        let mut parser = KATParser::new("kats/hqc-1/intermediates_values").expect("");

        let kat_seed_dk = parser.bytes_after("seed_dk: ").expect("").unwrap();
        let kat_seed_ek = parser.bytes_after("seed_ek: ").expect("").unwrap();
        let kat_seed_pke = parser.bytes_after("seed_pke: ").expect("").unwrap();

        let (seed_dk, seed_ek) = HQC_PKE::generate_seeds(&kat_seed_pke);
        assert_eq!(
            (seed_dk.to_vec(), seed_ek.to_vec()),
            (kat_seed_dk, kat_seed_ek)
        );
    }
}
