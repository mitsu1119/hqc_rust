use crate::util::ParentSet;

pub mod hadamard;
pub mod reed_solomon;

pub trait Code {
    type SymbolType: ParentSet;
    type CodeType;
    type MessageType;

    fn code_len(&self) -> usize;
    fn message_len(&self) -> usize;
    fn encode(&self, message: Self::MessageType) -> Self::CodeType;
    fn decode(&self, code: Self::CodeType) -> Self::MessageType;
}
