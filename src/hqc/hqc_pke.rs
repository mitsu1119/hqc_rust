use crate::{
    code::Code,
    hqc::{hqc_hash::HQCHash, hqc_param::HQCParam, xof::XOF},
};

#[allow(non_camel_case_types)]
pub struct HQC_PKE<'a> {
    pub param: HQCParam<'a>,
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

    fn n_bits_align(x: &mut Vec<u8>, n: usize) {
        x.resize(n.div_ceil(8), 0u8);
        if (n & 0b111) != 0 {
            if let Some(last) = x.last_mut() {
                *last &= (1u8 << (n & 0b111)) - 1;
            }
        }
    }

    // x[base..base+len)
    fn bits_slice(x: &Vec<u8>, base: usize, len: usize) -> Vec<u8> {
        let mut res = vec![0u8; len.div_ceil(8)];
        for i in 0..len {
            if Self::get_bit_from_bitvec(x, base + i) == 1 {
                Self::bit_flip_in_bitvec(&mut res, i);
            }
        }
        res
    }

    // nbits a ^ b
    fn bits_xor(n: usize, a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
        let mut res = vec![0u8; n.div_ceil(8)];
        for i in 0..n {
            let left = if i < a.len() * 8 {
                Self::get_bit_from_bitvec(a, i)
            } else {
                0
            };
            let right = if i < b.len() * 8 {
                Self::get_bit_from_bitvec(b, i)
            } else {
                0
            };
            if left ^ right == 1 {
                Self::bit_flip_in_bitvec(&mut res, i);
            }
        }
        res
    }

    // a ^= (b << shift)
    fn bits_shift_xor(a: &mut Vec<u8>, b: &Vec<u8>, shift: usize) {
        let n = b.len() * 8;
        for i in 0..n {
            if Self::get_bit_from_bitvec(b, i) == 1 {
                Self::bit_flip_in_bitvec(a, i + shift);
            }
        }
    }

    fn vec_mul_schoolbook(n: usize, a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
        let mut res = vec![0u8; (n * 2).div_ceil(8)];
        for i in 0..n {
            if Self::get_bit_from_bitvec(a, i) == 0 {
                continue;
            }
            for j in 0..n {
                if Self::get_bit_from_bitvec(b, j) == 1 {
                    Self::bit_flip_in_bitvec(&mut res, i + j);
                }
            }
        }
        res
    }

    const SCHOOLBOOK_BITS: usize = 256;
    fn vec_mul_karatsuba(n: usize, a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
        if n == 0 {
            return vec![];
        }
        if n <= Self::SCHOOLBOOK_BITS {
            return Self::vec_mul_schoolbook(n, a, b);
        }

        let n0 = n >> 1;
        let n1 = n - n0;
        let a0 = Self::bits_slice(a, 0, n0);
        let a1 = Self::bits_slice(a, n >> 1, n1);
        let b0 = Self::bits_slice(b, 0, n0);
        let b1 = Self::bits_slice(b, n >> 1, n1);

        let z0 = Self::vec_mul_karatsuba(n0, &a0, &b0);
        let z2 = Self::vec_mul_karatsuba(n1, &a1, &b1);

        // (a0 + a1) * (b0 + b1) + z0 + z2
        let l = n0.max(n1);
        let x = Self::bits_xor(l, &a0, &a1);
        let y = Self::bits_xor(l, &b0, &b1);
        let mut z1 = Self::vec_mul_karatsuba(l, &x, &y);
        Self::bits_shift_xor(&mut z1, &z0, 0);
        Self::bits_shift_xor(&mut z1, &z2, 0);

        // z0 + (z1 << n0) + (z2 << n0)
        let mut res = vec![0u8; (2 * n).div_ceil(8)];
        Self::bits_shift_xor(&mut res, &z0, 0);
        Self::bits_shift_xor(&mut res, &z1, n0);
        Self::bits_shift_xor(&mut res, &z2, 2 * n0);

        res
    }

    fn vec_mul(&self, u: Vec<u8>, v: Vec<u8>) -> Vec<u8> {
        let mut u = u;
        let mut v = v;
        Self::n_bits_align(&mut u, self.param.n);
        Self::n_bits_align(&mut v, self.param.n);

        let mulmul = Self::vec_mul_karatsuba(self.param.n, &u, &v);
        // mod x^n - 1
        let mut res = vec![0u8; self.param.n.div_ceil(8)];
        for i in 0..self.param.n {
            if Self::get_bit_from_bitvec(&mulmul, i) == 1 {
                Self::bit_flip_in_bitvec(&mut res, i);
            }
        }
        for i in self.param.n..(2 * self.param.n).min(mulmul.len() * 8) {
            if Self::get_bit_from_bitvec(&mulmul, i) == 1 {
                Self::bit_flip_in_bitvec(&mut res, i - self.param.n);
            }
        }
        Self::n_bits_align(&mut res, self.param.n);

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

    // SampleFixedWeightVect$
    // unbias but rejection sampling
    fn sample_fixed_weight_vect_indices_dollar(&self, ctx: &mut XOF, weight: u8) -> Vec<usize> {
        let threshold = ((1 << 24) / self.param.n) * self.param.n;
        let block_size = 3 * weight as usize;

        let mut rand = vec![];
        let mut cnt = block_size;
        let mut res = vec![];
        while res.len() < weight as usize {
            let index_chunk = loop {
                if cnt == block_size {
                    rand = ctx.get_bytes(block_size);
                    cnt = 0;
                }

                let res = ((rand[cnt] as usize) << 16)
                    | ((rand[cnt + 1] as usize) << 8)
                    | rand[cnt + 2] as usize;
                cnt += 3;
                if res < threshold {
                    break res;
                }
            };

            let index = index_chunk % self.param.n;
            if !res.contains(&index) {
                res.push(index);
            }
        }

        res
    }

    // SampleFixedWeightVect
    // non-rejection sampling but small bias
    fn sample_fixed_weight_vect_indices(&self, ctx: &mut XOF, weight: u8) -> Vec<usize> {
        let rand = ctx.get_bytes(4 * weight as usize);
        let mut res = vec![0; weight as usize];

        for i in 0..weight {
            let rand_index = (i as usize) << 2;
            let index_chunk = ((rand[rand_index + 3] as usize) << 24)
                | ((rand[rand_index + 2] as usize) << 16)
                | ((rand[rand_index + 1] as usize) << 8)
                | (rand[rand_index] as usize);
            res[i as usize] = i as usize + ((index_chunk * (self.param.n - i as usize)) >> 32);
        }

        for i in (0..(weight - 1)).rev() {
            let x = res[i as usize];
            let found = res[(i as usize) + 1..].iter().any(|&y| y == x);
            if found {
                res[i as usize] = i as usize;
            }
        }

        res
    }

    fn generate_vec_from_weight_indices(&self, indices: &[usize]) -> Vec<u8> {
        let mut res = vec![0u8; self.param.n.div_ceil(8)];
        for index in indices.iter() {
            let res_ind = index >> 3;
            let bit_pos = index % 8;
            let bit = 1 << bit_pos;
            res[res_ind] |= bit;
        }

        res
    }

    fn sample_fixed_weight_vect_dollar(&self, ctx: &mut XOF, weight: u8) -> Vec<u8> {
        let indices = self.sample_fixed_weight_vect_indices_dollar(ctx, weight);
        let res = self.generate_vec_from_weight_indices(&indices);
        res
    }

    fn sample_fixed_weight_vect(&self, ctx: &mut XOF, weight: u8) -> Vec<u8> {
        let indices = self.sample_fixed_weight_vect_indices(ctx, weight);
        let res = self.generate_vec_from_weight_indices(&indices);
        res
    }

    fn sample_vec(&self, ctx: &mut XOF) -> Vec<u8> {
        let mut res = ctx.get_bytes(self.param.n.div_ceil(8));

        let r = self.param.n % 8;
        if r != 0 {
            let mask = (1usize << r) - 1;
            let len = res.len();
            res[len - 1] &= mask as u8;
        }

        res
    }

    fn generate_dk(&self, seed: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut ctx = XOF::new(seed);
        let y = self.sample_fixed_weight_vect_dollar(&mut ctx, self.param.omega);
        let x = self.sample_fixed_weight_vect_dollar(&mut ctx, self.param.omega);

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
        let y = self.sample_fixed_weight_vect_dollar(&mut ctx, self.param.omega);
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
    use crate::{hqc::hqc_pke::HQC_PKE, util::kat_parser::KATParser};

    #[test]
    fn generate_seeds_hqc1() {
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

    #[test]
    fn generate_seeds_hqc3() {
        let mut parser = KATParser::new("kats/hqc-3/intermediates_values").expect("");

        let kat_seed_dk = parser.bytes_after("seed_dk: ").expect("").unwrap();
        let kat_seed_ek = parser.bytes_after("seed_ek: ").expect("").unwrap();
        let kat_seed_pke = parser.bytes_after("seed_pke: ").expect("").unwrap();

        let (seed_dk, seed_ek) = HQC_PKE::generate_seeds(&kat_seed_pke);
        assert_eq!(
            (seed_dk.to_vec(), seed_ek.to_vec()),
            (kat_seed_dk, kat_seed_ek)
        );
    }

    #[test]
    fn generate_seeds_hqc5() {
        let mut parser = KATParser::new("kats/hqc-5/intermediates_values").expect("");

        let kat_seed_dk = parser.bytes_after("seed_dk: ").expect("").unwrap();
        let kat_seed_ek = parser.bytes_after("seed_ek: ").expect("").unwrap();
        let kat_seed_pke = parser.bytes_after("seed_pke: ").expect("").unwrap();

        let (seed_dk, seed_ek) = HQC_PKE::generate_seeds(&kat_seed_pke);
        assert_eq!(
            (seed_dk.to_vec(), seed_ek.to_vec()),
            (kat_seed_dk, kat_seed_ek)
        );
    }

    #[test]
    fn encrypt_hqc1() {
        let mut parser = KATParser::new("kats/hqc-1/intermediates_values").expect("");

        let kat_seed_pke = parser.bytes_after("seed_pke: ").expect("").unwrap();
        let kat_c_u = parser.bytes_after("c_pke->u: ").expect("").unwrap();
        let kat_c_v = parser.bytes_after("c_pke->v: ").expect("").unwrap();
        let _ = parser.bytes_after("ek_kem: ").expect("").unwrap();
        let kat_m = parser.bytes_after("m: ").expect("").unwrap();
        let kat_theta = parser.bytes_after("theta: ").expect("").unwrap();

        let hqc = HQC_PKE::hqc1();
        let (ek, dk) = hqc.keygen(&kat_seed_pke);

        let c = hqc.encrypt(ek, kat_m, &kat_theta);
        let (u, v) = c;

        assert_eq!(u, kat_c_u);
        assert_eq!(v, kat_c_v);
    }

    #[test]
    fn encrypt_hqc3() {
        let mut parser = KATParser::new("kats/hqc-3/intermediates_values").expect("");

        let kat_seed_pke = parser.bytes_after("seed_pke: ").expect("").unwrap();
        let kat_c_u = parser.bytes_after("c_pke->u: ").expect("").unwrap();
        let kat_c_v = parser.bytes_after("c_pke->v: ").expect("").unwrap();
        let _ = parser.bytes_after("ek_kem: ").expect("").unwrap();
        let kat_m = parser.bytes_after("m: ").expect("").unwrap();
        let kat_theta = parser.bytes_after("theta: ").expect("").unwrap();

        let hqc = HQC_PKE::hqc3();
        let (ek, dk) = hqc.keygen(&kat_seed_pke);

        let c = hqc.encrypt(ek, kat_m, &kat_theta);
        let (u, v) = c;
    }

    #[test]
    fn encrypt_hqc5() {
        let mut parser = KATParser::new("kats/hqc-5/intermediates_values").expect("");

        let kat_seed_pke = parser.bytes_after("seed_pke: ").expect("").unwrap();
        let kat_c_u = parser.bytes_after("c_pke->u: ").expect("").unwrap();
        let kat_c_v = parser.bytes_after("c_pke->v: ").expect("").unwrap();
        let _ = parser.bytes_after("ek_kem: ").expect("").unwrap();
        let kat_m = parser.bytes_after("m: ").expect("").unwrap();
        let kat_theta = parser.bytes_after("theta: ").expect("").unwrap();

        let hqc = HQC_PKE::hqc5();
        let (ek, dk) = hqc.keygen(&kat_seed_pke);

        let c = hqc.encrypt(ek, kat_m, &kat_theta);
        let (u, v) = c;

        assert_eq!(u, kat_c_u);
        assert_eq!(v, kat_c_v);
    }

    #[test]
    fn decrypt_hqc1() {
        let mut parser = KATParser::new("kats/hqc-1/intermediates_values").expect("");

        let kat_c_u = parser.bytes_after("c_pke.u: ").expect("").unwrap();
        let kat_c_v = parser.bytes_after("c_pke.v: ").expect("").unwrap();
        let kat_dk_pke = parser.bytes_after("dk_pke: ").expect("").unwrap();
        let kat_m_prime = parser.bytes_after("m_prime: ").expect("").unwrap();

        let hqc = HQC_PKE::hqc1();
        let m_prime = hqc.decrypt(kat_dk_pke.try_into().unwrap(), (kat_c_u, kat_c_v));

        assert_eq!(m_prime, kat_m_prime);
    }

    #[test]
    fn decrypt_hqc3() {
        let mut parser = KATParser::new("kats/hqc-3/intermediates_values").expect("");

        let kat_c_u = parser.bytes_after("c_pke.u: ").expect("").unwrap();
        let kat_c_v = parser.bytes_after("c_pke.v: ").expect("").unwrap();
        let kat_dk_pke = parser.bytes_after("dk_pke: ").expect("").unwrap();
        let kat_m_prime = parser.bytes_after("m_prime: ").expect("").unwrap();

        let hqc = HQC_PKE::hqc3();
        let m_prime = hqc.decrypt(kat_dk_pke.try_into().unwrap(), (kat_c_u, kat_c_v));

        assert_eq!(m_prime, kat_m_prime);
    }

    #[test]
    fn decrypt_hqc5() {
        let mut parser = KATParser::new("kats/hqc-5/intermediates_values").expect("");

        let kat_c_u = parser.bytes_after("c_pke.u: ").expect("").unwrap();
        let kat_c_v = parser.bytes_after("c_pke.v: ").expect("").unwrap();
        let kat_dk_pke = parser.bytes_after("dk_pke: ").expect("").unwrap();
        let kat_m_prime = parser.bytes_after("m_prime: ").expect("").unwrap();

        let hqc = HQC_PKE::hqc5();
        let m_prime = hqc.decrypt(kat_dk_pke.try_into().unwrap(), (kat_c_u, kat_c_v));

        assert_eq!(m_prime, kat_m_prime);
    }

    #[test]
    fn keygen_hqc1() {
        let mut parser = KATParser::new("kats/hqc-1/intermediates_values").expect("");

        let kat_s = parser.bytes_after("s: ").expect("").unwrap();
        let kat_seed_pke = parser.bytes_after("seed_pke: ").expect("").unwrap();

        let hqc = HQC_PKE::hqc1();
        let ((_, s), _) = hqc.keygen(&kat_seed_pke);

        assert_eq!(s, kat_s);
    }

    #[test]
    fn keygen_hqc3() {
        let mut parser = KATParser::new("kats/hqc-3/intermediates_values").expect("");

        let kat_s = parser.bytes_after("s: ").expect("").unwrap();
        let kat_seed_pke = parser.bytes_after("seed_pke: ").expect("").unwrap();

        let hqc = HQC_PKE::hqc3();
        let ((_, s), _) = hqc.keygen(&kat_seed_pke);

        assert_eq!(s, kat_s);
    }

    #[test]
    fn keygen_hqc5() {
        let mut parser = KATParser::new("kats/hqc-5/intermediates_values").expect("");

        let kat_s = parser.bytes_after("s: ").expect("").unwrap();
        let kat_seed_pke = parser.bytes_after("seed_pke: ").expect("").unwrap();

        let hqc = HQC_PKE::hqc5();
        let ((_, s), _) = hqc.keygen(&kat_seed_pke);

        assert_eq!(s, kat_s);
    }
}
