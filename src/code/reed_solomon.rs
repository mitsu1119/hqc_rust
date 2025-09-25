use crate::code::Code;

pub struct ReedSolomon<const CODE_LEN: usize, const MESSAGE_LEN: usize> {}

impl<const CODE_LEN: usize, const MESSAGE_LEN: usize> Code for ReedSolomon<CODE_LEN, MESSAGE_LEN> {
    const CODE_LEN: usize = CODE_LEN;
    const MESSAGE_LEN: usize = MESSAGE_LEN;
    type SymbolType = u16;
    type CodeType = [Self::SymbolType; CODE_LEN];
    type MessageType = [Self::SymbolType; MESSAGE_LEN];

    fn encode(message: Self::MessageType) -> Self::CodeType {
        [0; CODE_LEN]
    }
    fn decode(code: Self::CodeType) -> Self::MessageType {
        [0; MESSAGE_LEN]
    }
}
