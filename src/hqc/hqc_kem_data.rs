#[derive(Debug)]
pub struct EncryptionKeyKEM {
    pub data: Vec<u8>,
    seed_len: usize,
}

impl EncryptionKeyKEM {
    pub fn new(data: Vec<u8>, seed_len: usize) -> Self {
        Self { data, seed_len }
    }
    pub fn seed_pke(&self) -> &[u8] {
        &self.data[..self.seed_len]
    }
    pub fn s(&self) -> &[u8] {
        &self.data[self.seed_len..]
    }
}

#[derive(Debug)]
pub struct DecryptionKeyKEM {
    pub data: Vec<u8>,
    ek_size: usize,
    dk_size: usize,
    k: usize,
}

impl DecryptionKeyKEM {
    pub fn new(data: Vec<u8>, ek_size: usize, dk_size: usize, k: usize) -> Self {
        Self {
            data,
            ek_size,
            dk_size,
            k,
        }
    }
    pub fn ek_kem(&self) -> &[u8] {
        &self.data[..self.ek_size]
    }
    pub fn dk_pke(&self) -> &[u8] {
        &self.data[self.ek_size..self.ek_size + self.dk_size]
    }
    pub fn sigma(&self) -> &[u8] {
        &self.data[self.ek_size + self.dk_size..self.ek_size + self.dk_size + self.k]
    }
    pub fn seed_kem(&self) -> &[u8] {
        &self.data[self.ek_size + self.dk_size + self.k..]
    }
}

#[derive(Debug)]
pub struct CiphertextKEM {
    pub data: Vec<u8>,
    cipher_len: usize,
}

impl CiphertextKEM {
    pub fn new(data: Vec<u8>, cipher_len: usize) -> Self {
        Self { data, cipher_len }
    }
    pub fn c_pke(&self) -> &[u8] {
        &self.data[..self.cipher_len]
    }
    pub fn salt(&self) -> &[u8] {
        &self.data[self.cipher_len..]
    }
}
