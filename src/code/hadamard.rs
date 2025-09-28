use std::u128;

use crate::code::Code;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hadamard7;

impl Hadamard7 {
    fn new() -> Self {
        Self {}
    }

    fn m(&self) -> u8 {
        7
    }
}

impl Code for Hadamard7 {
    type SymbolType = u8;
    type CodeType = u128;

    // message: (a, b) = (a1 ... a7 b)
    // -> b = message & 0b10000000
    type MessageType = u8;

    fn code_len(&self) -> usize {
        (1 << self.m()) as usize
    }

    fn message_len(&self) -> usize {
        (self.m() + 1) as usize
    }

    fn encode(&self, message: Self::MessageType) -> Self::CodeType {
        let a = message & 0b1111111;
        let b = message >> 7;
        let mut c = 0u128;
        for i in 0..(self.code_len() >> 3) as u8 {
            let mut byte = 0u8;
            for j in (0u8..8).rev() {
                let xi = (i << 3) | j;
                println!("{}", xi);
                let inner = ((xi & a).count_ones() & 1) as u8;
                let ci = inner ^ b;
                byte = (byte << 1) | ci;
            }
            c = (c << 8) | byte as u128;
        }
        c
    }

    fn decode(&self, code: Self::CodeType) -> Self::MessageType {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::code::{Code, hadamard::Hadamard7};

    #[test]
    fn encode() {
        let had = Hadamard7::new();

        let msg = [0xed, 0xa2, 0xf5, 0x05];
        let res = [
            0xa55aa55a5aa55aa55aa55aa5a55aa55a,
            0x33333333cccccccc33333333cccccccc,
            0xa5a55a5a5a5aa5a55a5aa5a5a5a55a5a,
            0x5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a,
        ];

        for (m, r) in msg.into_iter().zip(res.into_iter()) {
            assert_eq!(had.encode(m), r);
        }
    }
}
