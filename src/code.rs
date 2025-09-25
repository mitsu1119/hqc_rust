pub mod reed_solomon;

trait Code {
    const CODE_LEN: usize;
    const MESSAGE_LEN: usize;
    type SymbolType;
    type CodeType;
    type MessageType;

    fn encode(message: Self::MessageType) -> Self::CodeType;
    fn decode(code: Self::CodeType) -> Self::MessageType;
}
