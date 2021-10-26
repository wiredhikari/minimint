use crate::contracts::{ContractId, IdentifyableContract};
use bitcoin_hashes::sha256::Hash as Sha256;
use bitcoin_hashes::Hash as BitcoinHash;
use bitcoin_hashes::{borrow_slice_impl, hash_newtype, hex_fmt_impl, index_impl, serde_impl};
use minimint_api::encoding::{Decodable, DecodeError, Encodable};
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
#[derive(Debug, Encodable, Decodable)]
pub struct IncomingContractOffer {
    /// Amount for which the user is willing to sell the preimage
    pub amount: minimint_api::Amount,
    pub hash: bitcoin_hashes::sha256::Hash,
    pub encrypted_preimage: EncryptedPreimage,
}

// TODO: don't duplicate offer, include id instead and fetch offer on mint side
#[derive(Debug, Encodable, Decodable)]
pub struct IncomingContract {
    pub hash: bitcoin_hashes::sha256::Hash,
    pub encrypted_preimage: EncryptedPreimage,
    pub gateway_key: secp256k1::schnorrsig::PublicKey,
}

hash_newtype!(
    OfferId,
    Sha256,
    32,
    doc = "The hash of a LN incoming contract offer"
);

#[derive(Debug, Encodable, Decodable)]
pub struct Preimage(secp256k1::schnorrsig::PublicKey);

#[derive(Debug)]
pub struct EncryptedPreimage(threshold_crypto::Ciphertext);

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
