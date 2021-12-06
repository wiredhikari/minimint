use crate::contracts::{ContractId, IdentifyableContract};
use bitcoin_hashes::sha256::Hash as Sha256;
use bitcoin_hashes::Hash as BitcoinHash;
use bitcoin_hashes::{borrow_slice_impl, hash_newtype, hex_fmt_impl, index_impl, serde_impl};
use minimint_api::encoding::{Decodable, DecodeError, Encodable};
use minimint_api::OutPoint;
use std::io::Error;

// FIXME: the protocol currently envisions the use of a pub key as preimage. This is bad for privacy
// though since pub keys are distinguishable from randomness and the payer would learn the recipient
// is using a federated mint. Probably best to just hash the key before.

/// Specialized smart contract for incoming payments
///
/// A user generates a private/public keypair that can later be used to claim the incoming funds.
/// The public key is the defined as the preimage of a payment has and threshold-encrypted to the
/// federation's public key. They then put up the encrypted preimage for sale by creating an offer
/// A lightning gateway wanting to claim an incoming HTLC can now use the offer to buy the preimage
/// by transferring funds into the corresponding contract. This activates the threshold decryption
/// process inside the federation. Since the user could have threshold-encrypted useless data there
/// are now two possible outcomes:
///
///   1. The decryption results in a valid preimage which is given to the lightning gateway. The
///      user can in return claim the funds from the contract. For this they need to be able to sign
///      with the private key corresponding to the public key which they used as preimage.
///   2. The decryption results in an invalid preimage, the gateway can claim back the money. For
///      this to work securely they have to specify a public key when creating the actual contract.
#[derive(Debug, Clone, PartialEq, Eq, Encodable, Decodable)]
pub struct IncomingContractOffer {
    /// Amount for which the user is willing to sell the preimage
    pub amount: minimint_api::Amount,
    pub hash: bitcoin_hashes::sha256::Hash,
    pub encrypted_preimage: EncryptedPreimage,
}

// TODO: don't duplicate offer, include id instead and fetch offer on mint side
#[derive(Debug, Clone, Encodable, Decodable)]
pub struct IncomingContract {
    pub hash: bitcoin_hashes::sha256::Hash,
    pub encrypted_preimage: EncryptedPreimage,
    pub decrypted_preimage: DecryptedPreimage,
    pub gateway_key: secp256k1::schnorrsig::PublicKey,
}

#[derive(Debug, Clone, Encodable, Decodable)]
pub struct FundedIncomingContract {
    pub contract: IncomingContract,
    /// Incoming contracts are funded exactly once, so they have an associated out-point. We use it
    /// to report the outcome of the preimage decryption started by the funding in the output's
    /// outcome (This can already be queried by users, making an additional way of querying contract
    /// states unnecessary for now).
    pub out_point: OutPoint,
}

#[derive(Debug, Clone, PartialEq, Eq, Encodable, Decodable)]
pub enum DecryptedPreimage {
    Pending,
    Some(Preimage),
    Invalid,
}

hash_newtype!(
    OfferId,
    Sha256,
    32,
    doc = "The hash of a LN incoming contract offer"
);

#[derive(Debug, Clone, PartialEq, Eq, Encodable, Decodable)]
pub struct Preimage(pub secp256k1::schnorrsig::PublicKey);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EncryptedPreimage(pub threshold_crypto::Ciphertext);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreimageDecryptionShare(pub threshold_crypto::DecryptionShare);

impl EncryptedPreimage {
    pub fn new(preimage: [u8; 32], key: &threshold_crypto::PublicKey) -> EncryptedPreimage {
        EncryptedPreimage(key.encrypt(preimage))
    }
}

impl IdentifyableContract for IncomingContract {
    fn contract_id(&self) -> ContractId {
        let mut engine = ContractId::engine();
        Encodable::consensus_encode(self, &mut engine).expect("Hashing never fails");
        ContractId::from_engine(engine)
    }
}

impl Encodable for EncryptedPreimage {
    fn consensus_encode<W: std::io::Write>(&self, writer: W) -> Result<usize, Error> {
        // TODO: get rid of bincode
        let bytes = bincode::serialize(&self.0).expect("Serialization shouldn't fail");
        bytes.consensus_encode(writer)
    }
}

impl Decodable for EncryptedPreimage {
    fn consensus_decode<D: std::io::Read>(d: D) -> Result<Self, DecodeError> {
        let bytes = Vec::<u8>::consensus_decode(d)?;
        Ok(EncryptedPreimage(
            bincode::deserialize(&bytes).map_err(DecodeError::from_err)?,
        ))
    }
}

impl Encodable for OfferId {
    fn consensus_encode<W: std::io::Write>(&self, writer: W) -> Result<usize, Error> {
        self.as_inner().consensus_encode(writer)
    }
}

impl Decodable for OfferId {
    fn consensus_decode<D: std::io::Read>(d: D) -> Result<Self, DecodeError> {
        Ok(OfferId::from_inner(Decodable::consensus_decode(d)?))
    }
}

impl Encodable for PreimageDecryptionShare {
    fn consensus_encode<W: std::io::Write>(&self, writer: W) -> Result<usize, Error> {
        // TODO: get rid of bincode
        let bytes = bincode::serialize(&self.0).expect("Serialization shouldn't fail");
        bytes.consensus_encode(writer)
    }
}

impl Decodable for PreimageDecryptionShare {
    fn consensus_decode<D: std::io::Read>(d: D) -> Result<Self, DecodeError> {
        let bytes = Vec::<u8>::consensus_decode(d)?;
        Ok(PreimageDecryptionShare(
            bincode::deserialize(&bytes).map_err(DecodeError::from_err)?,
        ))
    }
}
