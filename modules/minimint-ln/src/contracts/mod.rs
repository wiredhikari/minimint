pub mod account;
pub mod incoming;
pub mod outgoing;

use crate::contracts::incoming::{DecryptedPreimage, FundedIncomingContract};
use bitcoin_hashes::sha256::Hash as Sha256;
use bitcoin_hashes::Hash as BitcoinHash;
use bitcoin_hashes::{borrow_slice_impl, hash_newtype, hex_fmt_impl, index_impl, serde_impl};
use minimint_api::encoding::{Decodable, DecodeError, Encodable};
use minimint_api::OutPoint;
use std::io::Error;

pub trait IdentifyableContract: Encodable {
    fn contract_id(&self) -> ContractId;
}

hash_newtype!(
    ContractId,
    Sha256,
    32,
    doc = "The hash of a LN incoming contract"
);

#[derive(Debug, Clone, Encodable, Decodable)]
pub enum Contract {
    Account(account::AccountContract),
    Incoming(incoming::IncomingContract),
    Outgoing(outgoing::OutgoingContract),
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub enum FundedContract {
    Account(account::AccountContract),
    Incoming(incoming::FundedIncomingContract),
    Outgoing(outgoing::OutgoingContract),
}

#[derive(Debug, PartialEq, Eq, Encodable, Decodable)]
pub enum ContractOutcome {
    Account,
    Incoming(incoming::DecryptedPreimage),
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

impl IdentifyableContract for FundedContract {
    fn contract_id(&self) -> ContractId {
        match self {
            FundedContract::Account(c) => c.contract_id(),
            FundedContract::Incoming(c) => c.contract.contract_id(),
            FundedContract::Outgoing(c) => c.contract_id(),
        }
    }
}

impl Contract {
    pub fn to_outcome(&self) -> ContractOutcome {
        match self {
            Contract::Account(_) => ContractOutcome::Account,
            Contract::Incoming(_) => ContractOutcome::Incoming(DecryptedPreimage::Pending),
            Contract::Outgoing(_) => ContractOutcome::Outgoing,
        }
    }

    pub fn to_funded(self, out_point: OutPoint) -> FundedContract {
        match self {
            Contract::Account(account) => FundedContract::Account(account),
            Contract::Incoming(incoming) => FundedContract::Incoming(FundedIncomingContract {
                contract: incoming,
                out_point,
            }),
            Contract::Outgoing(outgoing) => FundedContract::Outgoing(outgoing),
        }
    }
}

impl Encodable for ContractId {
    fn consensus_encode<W: std::io::Write>(&self, writer: W) -> Result<usize, Error> {
        self.as_inner().consensus_encode(writer)
    }
}

impl Decodable for ContractId {
    fn consensus_decode<D: std::io::Read>(d: D) -> Result<Self, DecodeError> {
        Ok(ContractId::from_inner(Decodable::consensus_decode(d)?))
    }
}
