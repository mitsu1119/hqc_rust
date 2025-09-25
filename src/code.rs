pub mod reed_solomon;

pub trait Code {
    const CODE_LEN: usize;
    const MESSAGE_LEN: usize;
    type SymbolType;
    type CodeType;
    type MessageType;

    fn encode(&self, message: Self::MessageType) -> Self::CodeType;
    fn decode(&self, code: Self::CodeType) -> Self::MessageType;
}
