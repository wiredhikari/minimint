use crate::encoding::{Decodable, DecodeError, Encodable};
use bitcoin::hashes::Hash;
use std::io::Error;

impl Encodable for bitcoin_hashes::sha256::Hash {
    fn consensus_encode<W: std::io::Write>(&self, writer: W) -> Result<usize, Error> {
        self.into_inner().consensus_encode(writer)
    }
}

impl Decodable for bitcoin_hashes::sha256::Hash {
    fn consensus_decode<D: std::io::Read>(d: D) -> Result<Self, DecodeError> {
        Ok(bitcoin_hashes::sha256::Hash::from_inner(
            <[u8; 32]>::consensus_decode(d)?,
        ))
    }
}
