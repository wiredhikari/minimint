use crate::contracts::{ContractId, IdentifyableContract};
use bitcoin_hashes::Hash as BitcoinHash;
use minimint_api::encoding::{Decodable, Encodable};

/// A generic contract to hold money in a pub key locked account
#[derive(Debug, Encodable, Decodable)]
pub struct AccountContract {
    pub key: secp256k1::schnorrsig::PublicKey,
}

impl IdentifyableContract for AccountContract {
    fn contract_id(&self) -> ContractId {
        let mut engine = ContractId::engine();
        Encodable::consensus_encode(self, &mut engine).expect("Hashing never fails");
        ContractId::from_engine(engine)
    }
}
