use crate::code::Code;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hadamard {
    m: u8,
}

impl Hadamard {
    fn new(m: u8) -> Self {
        assert!(m > 0);
        assert!(m < u8::MAX);

        // assumption: SymbolTYpe = u8 => n % 8 == 0 and k % 8 == 0
        let n = 1 << m;
        let k = m + 1;
        assert_eq!(n & 0b111, 0);
        assert_eq!(k & 0b111, 0);
        Self { m }
    }
}

impl Code for Hadamard {
    type SymbolType = u8;
    type CodeType = Vec<Self::SymbolType>;
    type MessageType = Vec<Self::SymbolType>;

    fn code_len(&self) -> usize {
        (((1 << self.m) >> 3) + 1) as usize
    }

    fn message_len(&self) -> usize {
        (((self.m + 1) >> 3) + 1) as usize
    }

    fn encode(&self, message: Self::MessageType) -> Self::CodeType {
        vec![0]
    }

    fn decode(&self, code: Self::CodeType) -> Self::MessageType {
        vec![0]
    }
}
