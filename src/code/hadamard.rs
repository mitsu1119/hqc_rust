use crate::code::Code;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hadamard {
    m: u8,
}

impl Hadamard {
    fn new(m: u8) -> Self {
        assert!(m > 0);
        assert!(m < u8::MAX);

        let n = ((1 << m) >> 3) + 1;
        let k = (m + 1) >> 3;
        Self { m }
    }
}

impl Code for Hadamard {
    type SymbolType = u8;
    type CodeType = Vec<Self::SymbolType>;
    type MessageType = Vec<Self::SymbolType>;

    fn code_len(&self) -> usize {
        1 << self.m
    }

    fn message_len(&self) -> usize {
        (self.m + 1) as usize
    }

    fn encode(&self, message: Self::MessageType) -> Self::CodeType {
        vec![0]
    }

    fn decode(&self, code: Self::CodeType) -> Self::MessageType {
        vec![0]
    }
}
