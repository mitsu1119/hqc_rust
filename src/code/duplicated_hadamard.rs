use crate::code::{Code, hadamard::Hadamard7};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DuplicatedHadamard7 {
    had: Hadamard7,
    multiplicity: u8,
}

impl DuplicatedHadamard7 {
    pub fn new(multiplicity: u8) -> Self {
        Self {
            had: Hadamard7::new(),
            multiplicity,
        }
    }
}

impl Code for DuplicatedHadamard7 {
    type SymbolType = u8;
    type CodeType = Vec<u128>;
    type MessageType = u8;

    fn code_len(&self) -> usize {
        self.had.code_len() * self.multiplicity as usize
    }

    fn message_len(&self) -> usize {
        self.had.message_len()
    }

    fn encode(&self, message: Self::MessageType) -> Self::CodeType {
        let code = self.had.encode(message);
        vec![code; self.multiplicity as usize]
    }

    fn decode(&self, code: Self::CodeType) -> Self::MessageType {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::code::{Code, duplicated_hadamard::DuplicatedHadamard7};

    #[test]
    fn encode() {
        let had = DuplicatedHadamard7::new(3);

        let msg = [0xed, 0xa2, 0xf5, 0x05];
        let res: [Vec<u128>; 4] = [
            vec![0xa55aa55a5aa55aa55aa55aa5a55aa55a; 3],
            vec![0x33333333cccccccc33333333cccccccc; 3],
            vec![0xa5a55a5a5a5aa5a55a5aa5a5a5a55a5a; 3],
            vec![0x5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a; 3],
        ];

        for (m, r) in msg.into_iter().zip(res.into_iter()) {
            assert_eq!(had.encode(m), r);
        }
    }
}
