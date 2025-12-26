use blake2::{Blake2s256, Digest};
use ark_ff::{PrimeField};
use ark_bn254::{Fr};

pub struct FiatShamir {
    transcript: Vec<u8>
}

impl FiatShamir {
    pub fn new() -> Self {
        Self { transcript: Vec::new() }
    }

    pub fn update(&mut self, bytes: &[u8]){
        self.transcript.extend_from_slice(bytes);
    }

    pub fn challenge(&self) -> Fr {
        let hash = Blake2s256::digest(&self.transcript);
        Fr::from_le_bytes_mod_order(&hash)
    }
}