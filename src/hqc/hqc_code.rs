use crate::{
    code::{Code, duplicated_hadamard::DuplicatedHadamard7, reed_solomon::ReedSolomon},
    hqc::hqc_param::HQCParam,
    util::ParentSet,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HQCCode<'a> {
    had: DuplicatedHadamard7,
    rs: ReedSolomon<'a>,
}

impl<'a> HQCCode<'a> {
    fn new(param: &'a HQCParam<'a>) -> Self {
        let had = DuplicatedHadamard7::new(param.hadamard_multiplicity);
        let rs = ReedSolomon::new(
            param.rs_n,
            param.rs_k,
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
    type CodeType = Vec<u128>;

    fn message_len(&self) -> usize {
        self.rs.message_len()
    }

    fn code_len(&self) -> usize {
        self.had.message_len()
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

        let res: Vec<_> = had_code.into_iter().flatten().collect();

        res
    }

    fn decode(&self, code: Self::CodeType) -> Self::MessageType {
        // self.had.decode(code);

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        code::Code,
        hqc::{hqc_code::HQCCode, hqc_param::HQCParam},
    };

    #[test]
    fn encode() {
        const N: usize = 46;
        const K: usize = 16;
        const MULTIPLICITY: u8 = 3;

        let coeffs = [
            89, 69, 153, 116, 176, 117, 111, 75, 73, 233, 242, 233, 65, 210, 21, 139, 103, 173, 67,
            118, 105, 210, 174, 110, 74, 69, 228, 82, 255, 181, 1,
        ];
        let param = HQCParam::new(N, K, coeffs.to_vec(), MULTIPLICITY);

        let hqc_code = HQCCode::new(&param);

        let m = [
            116, 178, 211, 82, 207, 116, 201, 52, 6, 156, 157, 231, 71, 87, 245, 5,
        ];

        let enc = hqc_code.encode(m.to_vec());

        let hex_str = {
            let mut s = String::new();
            for i in 0..enc.len() {
                s += &format!("{:032x}", enc[i]);
            }
            s
        };

        let res = "a55aa55a5aa55aa55aa55aa5a55aa55aa55aa55a5aa55aa55aa55aa5a55aa55aa55aa55a5aa55aa55aa55aa5a55aa55a33333333cccccccc33333333cccccccc33333333cccccccc33333333cccccccc33333333cccccccc33333333cccccccccc3333cc33cccc3333cccc33cc3333cccc3333cc33cccc3333cccc33cc3333cccc3333cc33cccc3333cccc33cc3333ccf0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0f696996966969969669699696696996966969969669699696696996966969969669699696696996966969969669699696aaaa55555555aaaaaaaa55555555aaaaaaaa55555555aaaaaaaa55555555aaaaaaaa55555555aaaaaaaa55555555aaaaff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00ff00c33c3cc33cc3c33c3cc3c33cc33c3cc3c33c3cc33cc3c33c3cc3c33cc33c3cc3c33c3cc33cc3c33c3cc3c33cc33c3cc3aaaaaaaa5555555555555555aaaaaaaaaaaaaaaa5555555555555555aaaaaaaaaaaaaaaa5555555555555555aaaaaaaacc3333cc33cccc33cc3333cc33cccc33cc3333cc33cccc33cc3333cc33cccc33cc3333cc33cccc33cc3333cc33cccc3355aa55aa55aa55aaaa55aa55aa55aa5555aa55aa55aa55aaaa55aa55aa55aa5555aa55aa55aa55aaaa55aa55aa55aa55966996699669966969966996699669969669966996699669699669966996699696699669966996696996699669966996c33c3cc33cc3c33cc33c3cc33cc3c33cc33c3cc33cc3c33cc33c3cc33cc3c33cc33c3cc33cc3c33cc33c3cc33cc3c33c699696696996966996696996966969966996966969969669966969969669699669969669699696699669699696696996aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaa55553c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3caaaaaaaaaaaaaaaa5555555555555555aaaaaaaaaaaaaaaa5555555555555555aaaaaaaaaaaaaaaa55555555555555559669966969966996699669969669966996699669699669966996699696699669966996696996699669966996966996699999666666669999999966666666999999996666666699999999666666669999999966666666999999996666666699995555aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaa5555aaaaffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff669999666699996666999966669999666699996666999966669999666699996666999966669999666699996666999966c33c3cc33cc3c33cc33c3cc33cc3c33cc33c3cc33cc3c33cc33c3cc33cc3c33cc33c3cc33cc3c33cc33c3cc33cc3c33c55555555aaaaaaaaaaaaaaaa5555555555555555aaaaaaaaaaaaaaaa5555555555555555aaaaaaaaaaaaaaaa5555555533333333cccccccccccccccc3333333333333333cccccccccccccccc3333333333333333cccccccccccccccc33333333696969699696969696969696696969696969696996969696969696966969696969696969969696969696969669696969969696966969696969696969969696969696969669696969696969699696969696969696696969696969696996969696f0f00f0f0f0ff0f00f0ff0f0f0f00f0ff0f00f0f0f0ff0f00f0ff0f0f0f00f0ff0f00f0f0f0ff0f00f0ff0f0f0f00f0f0000ffffffff00000000ffffffff00000000ffffffff00000000ffffffff00000000ffffffff00000000ffffffff0000699696699669699669969669966969966996966996696996699696699669699669969669966969966996966996696996f0f00f0f0f0ff0f00f0ff0f0f0f00f0ff0f00f0f0f0ff0f00f0ff0f0f0f00f0ff0f00f0f0f0ff0f00f0ff0f0f0f00f0f3333cccccccc33333333cccccccc33333333cccccccc33333333cccccccc33333333cccccccc33333333cccccccc3333999966669999666666669999666699999999666699996666666699996666999999996666999966666666999966669999cccc3333cccc33333333cccc3333cccccccc3333cccc33333333cccc3333cccccccc3333cccc33333333cccc3333cccc699669966996699696699669966996696996699669966996966996699669966969966996699669969669966996699669f0f00f0f0f0ff0f00f0ff0f0f0f00f0ff0f00f0f0f0ff0f00f0ff0f0f0f00f0ff0f00f0f0f0ff0f00f0ff0f0f0f00f0f55aa55aa55aa55aaaa55aa55aa55aa5555aa55aa55aa55aaaa55aa55aa55aa5555aa55aa55aa55aaaa55aa55aa55aa55f0f00f0f0f0ff0f0f0f00f0f0f0ff0f0f0f00f0f0f0ff0f0f0f00f0f0f0ff0f0f0f00f0f0f0ff0f0f0f00f0f0f0ff0f03c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c3c0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00f0ff0f00fa55a5aa5a55a5aa5a55a5aa5a55a5aa5a55a5aa5a55a5aa5a55a5aa5a55a5aa5a55a5aa5a55a5aa5a55a5aa5a55a5aa5696969699696969696969696696969696969696996969696969696966969696969696969969696969696969669696969969696969696969669696969696969699696969696969696696969696969696996969696969696966969696969696969969669699696696969699696696996969696696996966969696996966969969696966969969669696969969669699696a5a55a5a5a5aa5a55a5aa5a5a5a55a5aa5a55a5a5a5aa5a55a5aa5a5a5a55a5aa5a55a5a5a5aa5a55a5aa5a5a5a55a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a";

        assert_eq!(hex_str, res);
    }
}
