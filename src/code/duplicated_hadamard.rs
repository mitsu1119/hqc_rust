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

    fn decode_from_u128(&self, codes: &[u128]) -> u8 {
        assert!(!codes.is_empty());

        let mut s = [0i32; 128];
        for &code in codes {
            for i in 0..16 {
                let shift = (15 - i) * 8;
                let byte = ((code >> shift) & 0xFF) as u8;
                for bitpos in (0..8).rev() {
                    // MSB → LSB
                    let x = i * 8 + bitpos;
                    let bit = (byte >> bitpos) & 1;
                    s[x] += if bit == 0 { 1 } else { -1 }; // 0→+1, 1→-1
                }
            }
        }

        Self::fht(&mut s);

        let mut idx = 0usize;
        let mut best = i32::MIN;
        for (i, &v) in s.iter().enumerate() {
            let abs = v.abs();
            if abs > best || (abs == best && i < idx) {
                best = abs;
                idx = i;
            }
        }

        let b_bit: u8 = if s[idx] >= 0 { 0 } else { 1 };
        let mut msg: u8 = b_bit << 7;
        for k in 0..7 {
            msg |= (((idx >> k) & 1) as u8) << k;
        }

        msg
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
        self.decode_from_u128(&code)
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

    #[test]
    fn decode() {
        let had = DuplicatedHadamard7::new(3);

        let code: [Vec<u128>; 4] = [
            vec![0xa55aa55a5aa55aa55aa55aa5a55aa55a; 3],
            vec![0x33333333cccccccc33333333cccccccc; 3],
            vec![0xa5a55a5a5a5aa5a55a5aa5a5a5a55a5a; 3],
            vec![0x5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a; 3],
        ];
        let error = [
            vec![
                0x00000000012000000000034000000000,
                0x00000000000000000000560000000000,
                0x07800009f00000000000000000000000,
            ],
            vec![
                0x00000000000127000000000000000000,
                0x9a0000000000000000000000032a0000,
                0x00000460000000000400000000000060,
            ],
            vec![
                0x00000000057000000000000000000000,
                0x01100000000000000000000008900000,
                0x000000aa000000de0000001200000000,
            ],
            vec![
                0x00000000057000000000000000000000,
                0x01100000000000000000000008900000,
                0x000000aa000000de0000001200000000,
            ],
        ];
        let errored_code = {
            let mut res = vec![];
            for i in 0..4 {
                let mut r = vec![];
                for j in 0..3 {
                    r.push(code[i][j] ^ error[i][j]);
                }
                res.push(r);
            }
            res
        };

        let res = [0xed, 0xa2, 0xf5, 0x05];

        for (c, r) in errored_code.into_iter().zip(res.into_iter()) {
            assert_eq!(had.decode(c), r);
        }
    }
}
