use crate::{code::Code, util::GaloisField};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReedSolomon<
    SymbolType: GaloisField + Default + Copy,
    const CODE_LEN: usize,
    const MESSAGE_LEN: usize,
    const PARITY_LEN: usize,
> {
    genpoly: [SymbolType; PARITY_LEN],
}

impl<
    SymbolType: GaloisField + Default + Copy,
    const CODE_LEN: usize,
    const MESSAGE_LEN: usize,
    const PARITY_LEN: usize,
> ReedSolomon<SymbolType, CODE_LEN, MESSAGE_LEN, PARITY_LEN>
{
    fn new(genpoly: [SymbolType; PARITY_LEN]) -> Self {
        Self { genpoly }
    }
}

impl<
    SymbolType: GaloisField + Default + Copy,
    const CODE_LEN: usize,
    const MESSAGE_LEN: usize,
    const PARITY_LEN: usize,
> Code for ReedSolomon<SymbolType, CODE_LEN, MESSAGE_LEN, PARITY_LEN>
{
    const CODE_LEN: usize = CODE_LEN;
    const MESSAGE_LEN: usize = MESSAGE_LEN;
    type SymbolType = SymbolType;
    type CodeType = [SymbolType; CODE_LEN];
    type MessageType = [SymbolType; MESSAGE_LEN];

    fn encode(message: Self::MessageType) -> Self::CodeType {
        [Default::default(); CODE_LEN]
    }
    fn decode(code: Self::CodeType) -> Self::MessageType {
        [Default::default(); MESSAGE_LEN]
    }
}
