pub mod contracts;

use crate::contracts::incoming::OfferId;
use crate::contracts::{ContractId, ContractOutcome};
use async_trait::async_trait;
use minimint_api::db::batch::BatchTx;
use minimint_api::db::RawDatabase;
use minimint_api::transaction::OutPoint;
use minimint_api::{Amount, FederationModule, PeerId};
use secp256k1::rand::{CryptoRng, RngCore};
use std::sync::Arc;

pub struct LightningModule {
    cfg: LightningModuleConfig,
    db: Arc<dyn RawDatabase>,
}

pub struct LightningModuleConfig {
    threshold_pub_key: threshold_crypto::PublicKeySet,
    threshold_sec_key: threshold_crypto::SecretKey,
}

pub struct ContractInput {
    pub crontract_id: contracts::ContractId,
    /// Of the three contract types only the outgoing one needs any other witness data than a
    /// signature. The signature is aggregated on the transaction level, so only the optional
    /// preimage remains.
    pub witness: Option<contracts::outgoing::Preimage>,
}

/// Represents an output of the Lightning module.
///
/// There are two sub-types:
///   * Normal contracts users may lock funds in
///   * Offers to buy preimages (see `contracts::incoming` docs)
///
/// The offer type exists to register `IncomingContractOffer`s. Instead of patching in a second way
/// of letting clients submit consensus items outside of transactions we let offers be a 0-amount
/// output. We need to take care to allow 0-input, 1-output transactions for that to allow users
/// to receive their fist tokens via LN without already having tokens.
pub enum ContractOrOfferOutput {
    Contract(ContractOutput),
    Offer(contracts::incoming::IncomingContractOffer),
}

pub struct ContractOutput {
    pub amount: minimint_api::Amount,
    pub contract: contracts::Contract,
}

pub enum OutputOutcome {
    Contract {
        id: ContractId,
        amount: minimint_api::Amount,
        outcome: ContractOutcome,
    },
    Offer {
        id: OfferId,
    },
}

pub enum LightningModuleConsensusItem {
    DecryptionShare(),
}

#[async_trait(?Send)]
impl FederationModule for LightningModule {
    type Error = LightningModuleError;
    type TxInput = ContractInput;
    type TxOutput = ContractOutput;
    type TxOutputOutcome = OutputOutcome;
    type ConsensusItem = LightningModuleConsensusItem;

    async fn consensus_proposal<'a>(
        &'a self,
        rng: impl RngCore + CryptoRng,
    ) -> Vec<Self::ConsensusItem> {
        todo!()
    }

    async fn begin_consensus_epoch<'a>(
        &'a self,
        batch: BatchTx<'a>,
        consensus_items: Vec<(PeerId, Self::ConsensusItem)>,
        rng: impl RngCore + CryptoRng,
    ) {
        todo!()
    }

    fn validate_input(&self, input: &Self::TxInput) -> Result<Amount, Self::Error> {
        todo!()
    }

    fn apply_input<'a>(
        &'a self,
        batch: BatchTx<'a>,
        input: &'a Self::TxInput,
    ) -> Result<Amount, Self::Error> {
        todo!()
    }

    fn validate_output(&self, output: &Self::TxOutput) -> Result<Amount, Self::Error> {
        todo!()
    }

    fn apply_output<'a>(
        &'a self,
        batch: BatchTx<'a>,
        output: &'a Self::TxOutput,
        out_point: OutPoint,
    ) -> Result<Amount, Self::Error> {
        todo!()
    }

    async fn end_consensus_epoch<'a>(&'a self, batch: BatchTx<'a>, rng: impl RngCore + CryptoRng) {
        todo!()
    }

    fn output_status(&self, out_point: OutPoint) -> Option<Self::TxOutputOutcome> {
        todo!()
    }
}

pub enum LightningModuleError {}
