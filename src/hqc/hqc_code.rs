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
                .map(|x| param.rs_symbol_field.elem(x).unwrap())
                .collect::<Vec<<<ReedSolomon as Code>::SymbolType as ParentSet>::ElementType<'a>>>(
                ),
        );

        Self { had, rs }
    }
}

/*
impl<'a> Code for HQCCode<'a> {
    type SymbolType = <ReedSolomon<'a> as Code>::SymbolType;
    type MessageType = <ReedSolomon<'a> as Code>::MessageType;
    type CodeType = <DuplicatedHadamard7 as Code>::CodeType;

    fn message_len(&self) -> usize {
        self.rs.message_len()
    }

    fn code_len(&self) -> usize {
        self.had.message_len()
    }

    fn encode(&self, message: Self::MessageType) -> Self::CodeType {
        let rs_code = self.rs.encode(message);
    }

    fn decode(&self, code: Self::CodeType) -> Self::MessageType {
        self.had.decode(code)
    }
}
*/
