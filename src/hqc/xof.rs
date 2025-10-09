use sha3::{
    Shake256, Shake256Reader,
    digest::{ExtendableOutput, Update, XofReader},
};

pub struct XOF {
    reader: Shake256Reader,
}

impl XOF {
    const HQC_XOF_DOMAIN: u8 = 1;

    pub fn new(seed: &[u8]) -> Self {
        let mut ctx = Shake256::default();
        ctx.update(seed);
        ctx.update(&[Self::HQC_XOF_DOMAIN]);
        Self {
            reader: ctx.finalize_xof(),
        }
    }

    pub fn init(&mut self, seed: &[u8]) {
        let mut ctx = Shake256::default();
        ctx.update(&seed);
        self.reader = ctx.finalize_xof();
    }

    fn squeeze_into(&mut self, out: &mut [u8]) {
        self.reader.read(out);
    }

    pub fn get_bytes(&mut self, len: usize) -> Vec<u8> {
        let mut out = vec![0u8; len];
        self.squeeze_into(&mut out);
        out
    }
}
