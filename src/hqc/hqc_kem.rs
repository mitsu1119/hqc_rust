use crate::hqc::{
    hqc_hash::HQCHash,
    hqc_kem_data::{CiphertextKEM, DecryptionKeyKEM, EncryptionKeyKEM},
    hqc_param::HQCParam,
    hqc_pke::HQC_PKE,
    xof::XOF,
};

#[allow(non_camel_case_types)]
pub struct HQC_KEM<'a> {
    pke: HQC_PKE<'a>,
}

impl<'a> HQC_KEM<'a> {
    const SEED_BYTES: usize = 32;
    const SHARED_SECRET_BYTES: usize = 32;

    pub fn new(param: HQCParam<'a>) -> Self {
        Self {
            pke: HQC_PKE::new(param),
        }
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

    fn k(&self) -> usize {
        self.pke.param.k
    }
    fn salt_bytes(&self) -> usize {
        16
    }

    pub fn keygen_from_seed(&self, seed_kem: &[u8]) -> (EncryptionKeyKEM, DecryptionKeyKEM) {
        // generate randomness
        let mut ctx_kem = XOF::new(seed_kem);
        let seed_pke = ctx_kem.get_bytes(Self::SEED_BYTES);
        let sigma = ctx_kem.get_bytes(self.pke.param.k);

        // keygen
        let (ek_pke, dk_pke) = self.pke.keygen(&seed_pke);

        // [*ek_pke.0 *ek_pke.1]
        let ek_kem = {
            let mut ek_kem = ek_pke.1;
            let len = ek_kem.len();
            ek_kem.reserve(32);
            ek_kem.resize(len + 32, 0);
            ek_kem.copy_within(0..len, 32);
            ek_kem[..32].copy_from_slice(&ek_pke.0);
            ek_kem
        };

        // [*ek_kem *dk_pke *sigma *seed_kem]
        let dk_kem = {
            let mut dk_kem = ek_kem.clone();
            let len = dk_kem.len();
            dk_kem.reserve(32 + self.pke.param.k + seed_kem.len());
            dk_kem.resize(len + 32 + self.pke.param.k + seed_kem.len(), 0);
            dk_kem[len..len + 32].copy_from_slice(&dk_pke);
            dk_kem[len + 32..len + 32 + self.pke.param.k].copy_from_slice(&sigma);
            dk_kem[len + 32 + self.pke.param.k..].copy_from_slice(&seed_kem);
            (dk_kem, ek_kem.len(), 32, self.pke.param.k)
        };

        return (
            EncryptionKeyKEM::new(ek_kem, Self::SEED_BYTES),
            DecryptionKeyKEM::new(dk_kem.0, dk_kem.1, dk_kem.2, dk_kem.3),
        );
    }

    pub fn encaps_from_m_salt(
        &self,
        ek_kem: &EncryptionKeyKEM,
        m: &[u8],
        salt: &[u8],
    ) -> (Vec<u8>, CiphertextKEM) {
        let k_theta = {
            let mut h = HQCHash::H(&ek_kem.data).to_vec();
            let len = h.len();
            h.reserve(m.len() + salt.len());
            h.resize(len + m.len() + salt.len(), 0);
            h[len..len + m.len()].copy_from_slice(&m);
            h[len + m.len()..].copy_from_slice(&salt);
            HQCHash::G(&h)
        };

        let (shared_secret, theta) = {
            let mut shared_secret = vec![0; Self::SHARED_SECRET_BYTES];
            let mut theta = vec![0; k_theta.len() - Self::SHARED_SECRET_BYTES];

            shared_secret.copy_from_slice(&k_theta[..Self::SHARED_SECRET_BYTES]);
            theta.copy_from_slice(&k_theta[Self::SHARED_SECRET_BYTES..]);

            (shared_secret, theta)
        };

        let seed_pke: [u8; 32] = ek_kem.seed_pke().clone().try_into().unwrap();
        let s = ek_kem.s().clone().to_vec();
        let c_pke = self.pke.encrypt((seed_pke, s), m.to_vec(), &theta);

        let c_kem = {
            let mut c_kem = c_pke.0;
            let len = c_kem.len();
            let c_len = len + c_pke.1.len();
            c_kem.reserve(c_pke.1.len() + salt.len());
            c_kem.resize(len + c_pke.1.len() + salt.len(), 0);
            c_kem[len..len + c_pke.1.len()].copy_from_slice(&c_pke.1);
            c_kem[len + c_pke.1.len()..].copy_from_slice(salt);
            CiphertextKEM::new(c_kem, c_len)
        };

        (shared_secret, c_kem)
    }

    pub fn keygen(&self) -> (EncryptionKeyKEM, DecryptionKeyKEM) {
        let seed_kem = {
            let mut res = [0u8; Self::SEED_BYTES];
            getrandom::fill(&mut res).expect("OS RNG");
            res
        };

        self.keygen_from_seed(&seed_kem)
    }

    pub fn encaps(&self, ek_kem: &EncryptionKeyKEM) -> (Vec<u8>, CiphertextKEM) {
        let m = {
            let mut res = vec![0u8; self.k()];
            getrandom::fill(&mut res).expect("OS RNG");
            res
        };
        let salt = {
            let mut res = vec![0u8; self.salt_bytes()];
            getrandom::fill(&mut res).expect("OS RNG");
            res
        };
        self.encaps_from_m_salt(ek_kem, &m, &salt)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        hqc::{hqc_kem::HQC_KEM, xof::XOF},
        util::kat_parser::KATParser,
    };

    #[test]
    fn hqc1_kat1() {
        println!("HQC-1 KATs test");

        let mut parser = KATParser::new("kats/hqc-1/PQCkemKAT_2321.rsp").expect("");

        let count = parser.line_after("count = ").unwrap().expect("");
        println!("count: {}", count);

        let seed_bytes = parser.line_after("seed = ").unwrap().expect("");
        println!("seed: {}", seed_bytes);
        let seed = KATParser::hex_to_bytes(&seed_bytes);

        let mut test_xof = XOF::new_prng(&seed);
        let seed_kem = test_xof.get_bytes(HQC_KEM::SEED_BYTES);

        let hqc = HQC_KEM::hqc1();
        let (ek, dk) = hqc.keygen_from_seed(&seed_kem);

        let kat_pk = parser.bytes_after("pk = ").unwrap().expect("");
        let kat_sk = parser.bytes_after("sk = ").unwrap().expect("");

        assert_eq!(ek.data, kat_pk);
        assert_eq!(dk.data, kat_sk);

        let kat_ct = parser.bytes_after("ct = ").unwrap().expect("");
        let kat_ss = parser.bytes_after("ss = ").unwrap().expect("");

        let m = test_xof.get_bytes(hqc.k());
        let salt = test_xof.get_bytes(hqc.salt_bytes());

        let (shared_secret, c_kem) = hqc.encaps_from_m_salt(&ek, &m, &salt);

        assert_eq!(c_kem.data, kat_ct);
        assert_eq!(shared_secret, kat_ss);
    }

    #[test]
    #[ignore]
    fn hqc1_kats() {
        println!("HQC-1 KATs test");

        let mut parser = KATParser::new("kats/hqc-1/PQCkemKAT_2321.rsp").expect("");

        loop {
            let count = parser.line_after("count = ").unwrap().expect("");
            println!("count: {}", count);

            let seed_bytes = parser.line_after("seed = ").unwrap().expect("");
            println!("seed: {}", seed_bytes);
            let seed = KATParser::hex_to_bytes(&seed_bytes);

            let mut test_xof = XOF::new_prng(&seed);
            let seed_kem = test_xof.get_bytes(HQC_KEM::SEED_BYTES);

            let hqc = HQC_KEM::hqc1();
            let (ek, dk) = hqc.keygen_from_seed(&seed_kem);

            let kat_pk = parser.bytes_after("pk = ").unwrap().expect("");
            let kat_sk = parser.bytes_after("sk = ").unwrap().expect("");

            assert_eq!(ek.data, kat_pk);
            assert_eq!(dk.data, kat_sk);

            let kat_ct = parser.bytes_after("ct = ").unwrap().expect("");
            let kat_ss = parser.bytes_after("ss = ").unwrap().expect("");

            let m = test_xof.get_bytes(hqc.k());
            let salt = test_xof.get_bytes(hqc.salt_bytes());

            let (shared_secret, c_kem) = hqc.encaps_from_m_salt(&ek, &m, &salt);

            assert_eq!(c_kem.data, kat_ct);
            assert_eq!(shared_secret, kat_ss);

            if count == "99" {
                break;
            }
        }
    }

    #[test]
    fn hqc3_kat1() {
        println!("HQC-3 KATs test");

        let mut parser = KATParser::new("kats/hqc-3/PQCkemKAT_4602.rsp").expect("");

        let count = parser.line_after("count = ").unwrap().expect("");
        println!("count: {}", count);

        let seed_bytes = parser.line_after("seed = ").unwrap().expect("");
        println!("seed: {}", seed_bytes);
        let seed = KATParser::hex_to_bytes(&seed_bytes);

        let mut test_xof = XOF::new_prng(&seed);
        let seed_kem = test_xof.get_bytes(HQC_KEM::SEED_BYTES);

        let hqc = HQC_KEM::hqc3();
        let (ek, dk) = hqc.keygen_from_seed(&seed_kem);

        let kat_pk = parser.bytes_after("pk = ").unwrap().expect("");
        let kat_sk = parser.bytes_after("sk = ").unwrap().expect("");

        assert_eq!(ek.data, kat_pk);
        assert_eq!(dk.data, kat_sk);

        let kat_ct = parser.bytes_after("ct = ").unwrap().expect("");
        let kat_ss = parser.bytes_after("ss = ").unwrap().expect("");

        let m = test_xof.get_bytes(hqc.k());
        let salt = test_xof.get_bytes(hqc.salt_bytes());

        let (shared_secret, c_kem) = hqc.encaps_from_m_salt(&ek, &m, &salt);

        assert_eq!(c_kem.data, kat_ct);
        assert_eq!(shared_secret, kat_ss);
    }

    #[test]
    #[ignore]
    fn hqc3_kats() {
        println!("HQC-3 KATs test");

        let mut parser = KATParser::new("kats/hqc-3/PQCkemKAT_4602.rsp").expect("");

        loop {
            let count = parser.line_after("count = ").unwrap().expect("");
            println!("count: {}", count);

            let seed_bytes = parser.line_after("seed = ").unwrap().expect("");
            println!("seed: {}", seed_bytes);
            let seed = KATParser::hex_to_bytes(&seed_bytes);

            let mut test_xof = XOF::new_prng(&seed);
            let seed_kem = test_xof.get_bytes(HQC_KEM::SEED_BYTES);

            let hqc = HQC_KEM::hqc3();
            let (ek, dk) = hqc.keygen_from_seed(&seed_kem);

            let kat_pk = parser.bytes_after("pk = ").unwrap().expect("");
            let kat_sk = parser.bytes_after("sk = ").unwrap().expect("");

            assert_eq!(ek.data, kat_pk);
            assert_eq!(dk.data, kat_sk);

            let kat_ct = parser.bytes_after("ct = ").unwrap().expect("");
            let kat_ss = parser.bytes_after("ss = ").unwrap().expect("");

            let m = test_xof.get_bytes(hqc.k());
            let salt = test_xof.get_bytes(hqc.salt_bytes());

            let (shared_secret, c_kem) = hqc.encaps_from_m_salt(&ek, &m, &salt);

            assert_eq!(c_kem.data, kat_ct);
            assert_eq!(shared_secret, kat_ss);

            if count == "99" {
                break;
            }
        }
    }

    #[test]
    fn hqc5_kat1() {
        println!("HQC-5 KATs test");

        let mut parser = KATParser::new("kats/hqc-5/PQCkemKAT_7333.rsp").expect("");

        let count = parser.line_after("count = ").unwrap().expect("");
        println!("count: {}", count);

        let seed_bytes = parser.line_after("seed = ").unwrap().expect("");
        println!("seed: {}", seed_bytes);
        let seed = KATParser::hex_to_bytes(&seed_bytes);

        let mut test_xof = XOF::new_prng(&seed);
        let seed_kem = test_xof.get_bytes(HQC_KEM::SEED_BYTES);

        let hqc = HQC_KEM::hqc5();
        let (ek, dk) = hqc.keygen_from_seed(&seed_kem);

        let kat_pk = parser.bytes_after("pk = ").unwrap().expect("");
        let kat_sk = parser.bytes_after("sk = ").unwrap().expect("");

        assert_eq!(ek.data, kat_pk);
        assert_eq!(dk.data, kat_sk);

        let kat_ct = parser.bytes_after("ct = ").unwrap().expect("");
        let kat_ss = parser.bytes_after("ss = ").unwrap().expect("");

        let m = test_xof.get_bytes(hqc.k());
        let salt = test_xof.get_bytes(hqc.salt_bytes());

        let (shared_secret, c_kem) = hqc.encaps_from_m_salt(&ek, &m, &salt);

        assert_eq!(c_kem.data, kat_ct);
        assert_eq!(shared_secret, kat_ss);
    }

    #[test]
    #[ignore]
    fn hqc5_kats() {
        println!("HQC-5 KATs test");

        let mut parser = KATParser::new("kats/hqc-5/PQCkemKAT_7333.rsp").expect("");

        loop {
            let count = parser.line_after("count = ").unwrap().expect("");
            println!("count: {}", count);

            let seed_bytes = parser.line_after("seed = ").unwrap().expect("");
            println!("seed: {}", seed_bytes);
            let seed = KATParser::hex_to_bytes(&seed_bytes);

            let mut test_xof = XOF::new_prng(&seed);
            let seed_kem = test_xof.get_bytes(HQC_KEM::SEED_BYTES);

            let hqc = HQC_KEM::hqc5();
            let (ek, dk) = hqc.keygen_from_seed(&seed_kem);

            let kat_pk = parser.bytes_after("pk = ").unwrap().expect("");
            let kat_sk = parser.bytes_after("sk = ").unwrap().expect("");

            assert_eq!(ek.data, kat_pk);
            assert_eq!(dk.data, kat_sk);

            let kat_ct = parser.bytes_after("ct = ").unwrap().expect("");
            let kat_ss = parser.bytes_after("ss = ").unwrap().expect("");

            let m = test_xof.get_bytes(hqc.k());
            let salt = test_xof.get_bytes(hqc.salt_bytes());

            let (shared_secret, c_kem) = hqc.encaps_from_m_salt(&ek, &m, &salt);

            assert_eq!(c_kem.data, kat_ct);
            assert_eq!(shared_secret, kat_ss);

            if count == "99" {
                break;
            }
        }
    }
}
