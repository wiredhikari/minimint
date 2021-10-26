pub mod account;
pub mod incoming;
pub mod outgoing;

use bitcoin_hashes::sha256::Hash as Sha256;
use bitcoin_hashes::Hash as BitcoinHash;
use bitcoin_hashes::{borrow_slice_impl, hash_newtype, hex_fmt_impl, index_impl, serde_impl};
use minimint_api::encoding::{Decodable, Encodable};

pub trait IdentifyableContract: Encodable {
    fn contract_id(&self) -> ContractId;
}

hash_newtype!(
    ContractId,
    Sha256,
    32,
    doc = "The hash of a LN incoming contract offer"
);

#[derive(Debug, Encodable, Decodable)]
pub enum Contract {
    Account(account::AccountContract),
    Incoming(incoming::IncomingContract),
    Outgoing(outgoing::OutgoingContract),
}

#[derive(Debug, Encodable, Decodable)]
pub enum ContractOutcome {
    Account,
    Incoming(incoming::Preimage),
    Outgoing,
}

impl IdentifyableContract for Contract {
    fn contract_id(&self) -> ContractId {
        match self {
            Contract::Account(c) => c.contract_id(),
            Contract::Incoming(c) => c.contract_id(),
            Contract::Outgoing(c) => c.contract_id(),
        }
    }
}
