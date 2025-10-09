use crate::{
    code::{Code, duplicated_hadamard::DuplicatedHadamard7, reed_solomon::ReedSolomon},
    hqc::hqc_param::HQCRSParam,
    util::ParentSet,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HQCCode<'a> {
    had: DuplicatedHadamard7,
    rs: ReedSolomon<'a>,
}

impl<'a> HQCCode<'a> {
    pub fn new(param: &'a HQCRSParam<'a>, hadamard_multiplicity: u8) -> Self {
        let had = DuplicatedHadamard7::new(hadamard_multiplicity);
        let rs = ReedSolomon::new(
            param.n,
            param.k,
            &param.rs_symbol_field,
            param
                .rs_genpoly
                .clone()
                .into_iter()
                .map(|x| param.rs_symbol_field.elem(x.into()).unwrap())
                .collect::<Vec<<<ReedSolomon as Code>::SymbolType as ParentSet>::ElementType<'a>>>(
                ),
        );

        Self { had, rs }
    }
}

impl<'a> Code for HQCCode<'a> {
    type SymbolType = u8;
    type MessageType = Vec<Self::SymbolType>;
    type CodeType = Vec<Self::SymbolType>;

    fn message_len(&self) -> usize {
        self.rs.message_len()
    }

    fn code_len(&self) -> usize {
        self.had.code_len()
    }

    fn encode(&self, message: Self::MessageType) -> Self::CodeType {
        let msg_poly: Vec<_> = message
            .into_iter()
            .map(|x| self.rs.symbol_field().elem(x as u16).unwrap())
            .collect();
        let rs_code = self.rs.encode(msg_poly);
        let rs_u8s: Vec<_> = rs_code.into_iter().map(|x| x.value()).collect();

        let had_code = {
            let mut res = vec![];
            for c in rs_u8s {
                res.push(self.had.encode(c.try_into().unwrap()))
            }
            res
        };

        let res_u128: Vec<_> = had_code.into_iter().flatten().collect();
        let mut res = vec![];
        for c in res_u128 {
            let mut mask = 0xff << 120;
            for i in 0..16 {
                res.push(((c & mask) >> (120 - i * 8)) as u8);
                mask >>= 8;
            }
        }

        res
    }

    fn decode(&self, code: Self::CodeType) -> Self::MessageType {
        assert!(code.len() % 16 == 0);

        let mut code_u128s = vec![];
        for i in (0..code.len()).step_by(16) {
            let mut c = 0u128;
            for j in 0..16 {
                c = (c << 8) | code[i + j] as u128;
            }
            code_u128s.push(c);
        }

        assert!(code_u128s.len() % self.had.multiplicity as usize == 0);

        let mut had_msg = vec![];
        for i in (0..code_u128s.len()).step_by(self.had.multiplicity as usize) {
            let m = self
                .had
                .decode(code_u128s[i..i + self.had.multiplicity as usize].to_vec());
            had_msg.push(self.rs.symbol_field().elem(m as u16).unwrap());
        }

        let res: Vec<_> = self
            .rs
            .decode(had_msg)
            .into_iter()
            .map(|x| x.value() as u8)
            .collect();

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        code::Code,
        hqc::{hqc_code::HQCCode, hqc_param::HQCRSParam},
        util::kat_parser::KATParser,
    };

    #[test]
    fn encode_hqc1() {
        let mut parser = KATParser::new("kats/hqc-1/intermediates_values").expect("");

        let _ = parser.line_after("### ENCAPS");
        let kat_ccw = parser
            .bytes_after("Concatenated code word: ")
            .expect("")
            .unwrap();
        let _ = parser.bytes_after("ek_kem: ");
        let kat_m = parser.bytes_after("m: ").expect("").unwrap();

        const MULTIPLICITY: u8 = 3;
        let param = HQCRSParam::new_rss1();
        let hqc_code = HQCCode::new(&param, MULTIPLICITY);
        let enc = hqc_code.encode(kat_m);

        assert_eq!(enc, kat_ccw);
    }

    #[test]
    fn encode_hqc3() {
        let mut parser = KATParser::new("kats/hqc-3/intermediates_values").expect("");

        let _ = parser.line_after("### ENCAPS");
        let kat_ccw = parser
            .bytes_after("Concatenated code word: ")
            .expect("")
            .unwrap();
        let _ = parser.bytes_after("ek_kem: ");
        let kat_m = parser.bytes_after("m: ").expect("").unwrap();

        const MULTIPLICITY: u8 = 5;
        let param = HQCRSParam::new_rss3();
        let hqc_code = HQCCode::new(&param, MULTIPLICITY);
        let enc = hqc_code.encode(kat_m);

        assert_eq!(enc, kat_ccw);
    }

    #[test]
    fn encode_hqc5() {
        let mut parser = KATParser::new("kats/hqc-5/intermediates_values").expect("");

        let _ = parser.line_after("### ENCAPS");
        let kat_ccw = parser
            .bytes_after("Concatenated code word: ")
            .expect("")
            .unwrap();
        let _ = parser.bytes_after("ek_kem: ");
        let kat_m = parser.bytes_after("m: ").expect("").unwrap();

        const MULTIPLICITY: u8 = 5;
        let param = HQCRSParam::new_rss5();
        let hqc_code = HQCCode::new(&param, MULTIPLICITY);
        let enc = hqc_code.encode(kat_m);

        assert_eq!(enc, kat_ccw);
    }

    #[test]
    fn decode_hqc1() {
        let mut parser = KATParser::new("kats/hqc-1/intermediates_values").expect("");

        let _ = parser.line_after("### DECAPS");
        let kat_ccw = parser
            .bytes_after("Concatenated code word: ")
            .expect("")
            .unwrap();
        let _ = parser.bytes_after("c_kem: ").expect("").unwrap();
        let kat_m_prime = parser.bytes_after("m_prime: ").expect("").unwrap();

        const MULTIPLICITY: u8 = 3;
        let param = HQCRSParam::new_rss1();
        let hqc_code = HQCCode::new(&param, MULTIPLICITY);
        let dec = hqc_code.decode(kat_ccw);

        assert_eq!(dec, kat_m_prime);
    }

    #[test]
    fn decode_hqc3() {
        let mut parser = KATParser::new("kats/hqc-3/intermediates_values").expect("");

        let _ = parser.line_after("### DECAPS");
        let kat_ccw = parser
            .bytes_after("Concatenated code word: ")
            .expect("")
            .unwrap();
        let _ = parser.bytes_after("c_kem: ").expect("").unwrap();
        let kat_m_prime = parser.bytes_after("m_prime: ").expect("").unwrap();

        const MULTIPLICITY: u8 = 5;
        let param = HQCRSParam::new_rss3();
        let hqc_code = HQCCode::new(&param, MULTIPLICITY);
        let dec = hqc_code.decode(kat_ccw);

        assert_eq!(dec, kat_m_prime);
    }

    #[test]
    fn decode_hqc5() {
        let mut parser = KATParser::new("kats/hqc-5/intermediates_values").expect("");

        let _ = parser.line_after("### DECAPS");
        let kat_ccw = parser
            .bytes_after("Concatenated code word: ")
            .expect("")
            .unwrap();
        let _ = parser.bytes_after("c_kem: ").expect("").unwrap();
        let kat_m_prime = parser.bytes_after("m_prime: ").expect("").unwrap();

        const MULTIPLICITY: u8 = 5;
        let param = HQCRSParam::new_rss5();
        let hqc_code = HQCCode::new(&param, MULTIPLICITY);
        let dec = hqc_code.decode(kat_ccw);

        assert_eq!(dec, kat_m_prime);
    }
}
