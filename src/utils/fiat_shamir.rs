use blake2::{Blake2s256, Digest};
use ark_serialize::CanonicalSerialize;
use ark_ff::{PrimeField, BigInteger};

use crate::types::*;

pub struct FiatShamir {
    transcript: Vec<u8>
}

impl FiatShamir {
    pub fn new() -> Self {
        Self { transcript: Vec::new() }
    }

    fn update(&mut self, bytes: &[u8]){
        self.transcript.extend_from_slice(bytes);
    }

    pub fn append_g1(&mut self, p: &G1Point) {
        let mut bytes = Vec::new();
        p.serialize_compressed(&mut bytes).unwrap();
        self.update(&bytes);
    }

    pub fn append_field(&mut self, x: &Field) {
        let bytes = x.into_bigint().to_bytes_le();
        self.update(&bytes);
    }

    pub fn challenge(&mut self) -> Field {
        let hash = Blake2s256::digest(&self.transcript);
        let chal = Field::from_le_bytes_mod_order(&hash);
        self.append_field(&chal);
        chal
    }
}