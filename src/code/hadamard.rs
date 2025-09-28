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

    fn fht(a: &mut [i32]) {
        let n = a.len();
        assert!(n.is_power_of_two());
        let mut len = 1usize;
        while len < n {
            let step = len << 1;
            for i in (0..n).step_by(step) {
                for j in 0..len {
                    let u = a[i + j];
                    let v = a[i + j + len];
                    a[i + j] = u + v;
                    a[i + j + len] = u - v;
                }
            }
            len <<= 1;
        }
    }

    pub fn decode_from_u128(&self, code: u128) -> u8 {
        let mut s = [0i32; 128];
        for i in 0..16 {
            let shift = (15 - i) * 8;
            let byte = ((code >> shift) & 0xFF) as u8;
            for bitpos in (0..8).rev() {
                let x = i * 8 + bitpos;
                let bit = (byte >> bitpos) & 1;
                s[x] = if bit == 0 { 1 } else { -1 }; // 0→+1, 1→-1
            }
        }

        Self::fht(&mut s);

        let mut idx = 0usize;
        let mut best = i32::MIN;
        for (i, &v) in s.iter().enumerate() {
            let abs = v.abs();
            if abs > best {
                best = abs;
                idx = i;
            }
        }
        let mut a_bits = [0u8; 7];
        for k in 0..7 {
            a_bits[k] = ((idx >> k) & 1) as u8;
        }
        let b_bit: u8 = if s[idx] >= 0 { 0 } else { 1 };

        let mut msg: u8 = b_bit << 7;
        for k in 0..7 {
            msg |= a_bits[k] << k;
        }

        msg
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
                let inner = ((xi & a).count_ones() & 1) as u8;
                let ci = inner ^ b;
                byte = (byte << 1) | ci;
            }
            c = (c << 8) | byte as u128;
        }
        c
    }

    fn decode(&self, code: Self::CodeType) -> Self::MessageType {
        self.decode_from_u128(code)
    }
}

#[cfg(test)]
mod tests {
    use crate::code::{Code, hadamard::Hadamard7};

    #[test]
    fn encode() {
        let had = Hadamard7::new();

        let msg = [0xed, 0xa2, 0xf5, 0x05];
        let res: [u128; 4] = [
            0xa55aa55a5aa55aa55aa55aa5a55aa55a,
            0x33333333cccccccc33333333cccccccc,
            0xa5a55a5a5a5aa5a55a5aa5a5a5a55a5a,
            0x5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a,
        ];

        for (m, r) in msg.into_iter().zip(res.into_iter()) {
            assert_eq!(had.encode(m), r);
        }
    }

    #[test]
    fn decode() {
        let had = Hadamard7::new();

        let code: [u128; 4] = [
            0xa55aa55a5aa55aa55aa55aa5a55aa55a,
            0x33333333cccccccc33333333cccccccc,
            0xa5a55a5a5a5aa5a55a5aa5a5a5a55a5a,
            0x5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a,
        ];
        let res = [0xed, 0xa2, 0xf5, 0x05];

        for (c, r) in code.into_iter().zip(res.into_iter()) {
            assert_eq!(had.decode(c), r);
        }
    }
}
