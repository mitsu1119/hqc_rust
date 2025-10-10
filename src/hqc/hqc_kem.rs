use crate::hqc::{hqc_param::HQCParam, hqc_pke::HQC_PKE, xof::XOF};

#[derive(Debug)]
pub struct EncryptionKeyKEM {
    data: Vec<u8>,
    seed_len: usize,
}

impl EncryptionKeyKEM {
    pub fn new(data: Vec<u8>, seed_len: usize) -> Self {
        Self { data, seed_len }
    }
    pub fn seed_pke(&self) -> &[u8] {
        &self.data[..self.seed_len]
    }
    pub fn s(&self) -> &[u8] {
        &self.data[self.seed_len..]
    }
}

#[derive(Debug)]
pub struct DecryptionKeyKEM {
    data: Vec<u8>,
    ek_size: usize,
    dk_size: usize,
    k: usize,
}

impl DecryptionKeyKEM {
    pub fn new(data: Vec<u8>, ek_size: usize, dk_size: usize, k: usize) -> Self {
        Self {
            data,
            ek_size,
            dk_size,
            k,
        }
    }
    pub fn ek_kem(&self) -> &[u8] {
        &self.data[..self.ek_size]
    }
    pub fn dk_pke(&self) -> &[u8] {
        &self.data[self.ek_size..self.ek_size + self.dk_size]
    }
    pub fn sigma(&self) -> &[u8] {
        &self.data[self.ek_size + self.dk_size..self.ek_size + self.dk_size + self.k]
    }
    pub fn seed_kem(&self) -> &[u8] {
        &self.data[self.ek_size + self.dk_size + self.k..]
    }
}

#[allow(non_camel_case_types)]
pub struct HQC_KEM<'a> {
    pke: HQC_PKE<'a>,
}

impl<'a> HQC_KEM<'a> {
    const SEED_BYTES: usize = 32;

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

    pub fn keygen_from_seed(&self, seed_kem: [u8; 32]) -> (EncryptionKeyKEM, DecryptionKeyKEM) {
        // generate randomness
        let mut ctx_kem = XOF::new(&seed_kem);
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
            dk_kem.reserve(64 + self.pke.param.k);
            dk_kem.resize(len + 64 + self.pke.param.k, 0);
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

    pub fn keygen(&self) -> (EncryptionKeyKEM, DecryptionKeyKEM) {
        let seed_kem = {
            let mut res = [0u8; Self::SEED_BYTES];
            getrandom::fill(&mut res).expect("OS RNG");
            res
        };

        self.keygen_from_seed(seed_kem)
    }
}

#[cfg(test)]
mod tests {
    use crate::{hqc::hqc_kem::HQC_KEM, util::kat_parser::KATParser};

    #[test]
    fn hqc1_kats() {
        println!("HQC-1 KATs test");

        let mut parser = KATParser::new("kats/hqc-1/PQCkemKAT_2321.rsp").expect("");

        let count = parser.line_after("count = ").unwrap().expect("");
        println!("count: {}", count);

        let seed_bytes = parser.line_after("seed = ").unwrap().expect("");
        println!("seed: {}", seed_bytes);
        let seed = KATParser::hex_to_bytes(&seed_bytes);

        let hqc = HQC_KEM::hqc1();
        // let (ek, dk) = hqc.keygen_from_seed(seed);
    }
}
