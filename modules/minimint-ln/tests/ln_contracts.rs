use bitcoin_hashes::Hash as BitcoinHash;
use minimint_api::config::GenerateConfig;
use minimint_api::db::batch::{BatchItem, DbBatch};
use minimint_api::db::mem_impl::MemDatabase;
use minimint_api::db::{Database, RawDatabase};
use minimint_api::{Amount, FederationModule, InputMeta, OutPoint, PeerId};
use minimint_ln::config::LightningModuleClientConfig;
use minimint_ln::contracts::account::AccountContract;
use minimint_ln::contracts::incoming::{
    DecryptedPreimage, EncryptedPreimage, IncomingContract, IncomingContractOffer,
};
use minimint_ln::contracts::outgoing::{OutgoingContract, Preimage};
use minimint_ln::contracts::{Contract, ContractOutcome, IdentifyableContract};
use minimint_ln::{
    ContractAccount, ContractInput, ContractOrOfferOutput, ContractOutput, LightningModule,
    LightningModuleError, OutputOutcome,
};
use std::fmt::Debug;

#[tokio::test]
async fn test_account() {
    let mut rng = secp256k1::rand::rngs::OsRng::new().unwrap();

    let mut fed = FakeFed::<LightningModule, LightningModuleClientConfig>::new(
        4,
        1,
        LightningModule::new,
        &(),
    );

    let (_sk, pk) = secp256k1::SECP256K1.generate_schnorrsig_keypair(&mut rng);
    let contract = Contract::Account(AccountContract { key: pk });

    let account_output = ContractOrOfferOutput::Contract(ContractOutput {
        amount: Amount::from_sat(42),
        contract: contract.clone(),
    });
    let account_out_point = OutPoint {
        txid: Default::default(),
        out_idx: 0,
    };
    let outputs = [(account_out_point, account_output)];

    fed.consensus_round(&[], &outputs).await;
    match fed.output_outcome(account_out_point).unwrap() {
        OutputOutcome::Contract { outcome, .. } => {
            assert_eq!(outcome, ContractOutcome::Account);
        }
        _ => panic!(),
    };

    let account_input = ContractInput {
        crontract_id: contract.contract_id(),
        amount: Amount::from_sat(42),
        witness: None,
    };
    let meta = fed.verify_input(&account_input).unwrap();
    assert_eq!(meta.keys, vec![pk]);

    fed.consensus_round(&[account_input], &[]).await;

    assert!(fed.verify_input(&account_input).is_err());
}

#[tokio::test]
async fn test_outgoing() {
    let mut rng = secp256k1::rand::rngs::OsRng::new().unwrap();

    let mut fed = FakeFed::<LightningModule, LightningModuleClientConfig>::new(
        4,
        1,
        LightningModule::new,
        &(),
    );

    let (_, gw_pk) = secp256k1::SECP256K1.generate_schnorrsig_keypair(&mut rng);
    let (_, user_pk) = secp256k1::SECP256K1.generate_schnorrsig_keypair(&mut rng);
    let preimage = [42u8; 32];
    let hash = secp256k1::hashes::sha256::Hash::hash(&preimage);

    let contract = Contract::Outgoing(OutgoingContract {
        hash,
        gateway_key: gw_pk,
        timelock: 42,
        user_key: user_pk,
        invoice: "not enforced yet".to_string(),
    });

    let outgoing_output = ContractOrOfferOutput::Contract(ContractOutput {
        amount: Amount::from_sat(42),
        contract: contract.clone(),
    });
    let outgoing_out_point = OutPoint {
        txid: Default::default(),
        out_idx: 0,
    };
    let outputs = [(outgoing_out_point, outgoing_output)];

    fed.consensus_round(&[], &outputs).await;
    match fed.output_outcome(outgoing_out_point).unwrap() {
        OutputOutcome::Contract { outcome, .. } => {
            assert_eq!(outcome, ContractOutcome::Outgoing);
        }
        _ => panic!(),
    };

    // Test case 1: before timeout
    fed.patch_dbs(|db| set_block_height(db, 0));

    // Error: Missing preimage
    let account_input_no_witness = ContractInput {
        crontract_id: contract.contract_id(),
        amount: Amount::from_sat(42),
        witness: None,
    };
    let err = fed.verify_input(&account_input_no_witness).unwrap_err();
    assert_eq!(err, LightningModuleError::MissingPreimage);

    // Ok
    let account_input_witness = ContractInput {
        crontract_id: contract.contract_id(),
        amount: Amount::from_sat(42),
        witness: Some(Preimage(preimage)),
    };
    let meta = fed.verify_input(&account_input_witness).unwrap();
    assert_eq!(meta.keys, vec![gw_pk]);

    // Test case 2: after timeout
    fed.patch_dbs(|db| set_block_height(db, 42));
    let meta = fed.verify_input(&account_input_no_witness).unwrap();
    assert_eq!(meta.keys, vec![user_pk]);

    fed.consensus_round(&[account_input_no_witness], &[]).await;
}

#[tokio::test]
async fn test_incoming() {
    tracing_subscriber::fmt()
        .with_test_writer()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("trace")),
        )
        .init();

    let mut rng = secp256k1::rand::rngs::OsRng::new().unwrap();

    let mut fed = FakeFed::<LightningModule, LightningModuleClientConfig>::new(
        4,
        1,
        LightningModule::new,
        &(),
    );

    let (_, gw_pk) = secp256k1::SECP256K1.generate_schnorrsig_keypair(&mut rng);
    let (_, user_pk) = secp256k1::SECP256K1.generate_schnorrsig_keypair(&mut rng);

    let preimage = user_pk.serialize();
    let hash = secp256k1::hashes::sha256::Hash::hash(&preimage);

    let offer = IncomingContractOffer {
        amount: Amount::from_sat(42),
        hash,
        encrypted_preimage: EncryptedPreimage::new(preimage, &fed.client_cfg().threshold_pub_key),
    };
    let offer_output = ContractOrOfferOutput::Offer(offer.clone());
    let offer_out_point = OutPoint {
        txid: Default::default(),
        out_idx: 0,
    };

    fed.consensus_round(&[], &[(offer_out_point, offer_output)])
        .await;
    let offers = fed.fetch_from_all(|m| m.get_offers());
    assert_eq!(offers, vec![offer.clone()]);

    let contract = Contract::Incoming(IncomingContract {
        hash, // TODO: check unknown hash
        encrypted_preimage: offer.encrypted_preimage,
        decrypted_preimage: DecryptedPreimage::Pending, // TODO: check what happens if this is not pending
        gateway_key: gw_pk,
    });
    let incoming_output = ContractOrOfferOutput::Contract(ContractOutput {
        amount: Amount::from_sat(42),
        contract: contract.clone(),
    });
    let incoming_out_point = OutPoint {
        txid: Default::default(),
        out_idx: 1,
    };
    let outputs = [(incoming_out_point, incoming_output)];

    fed.consensus_round(&[], &outputs).await;
    match fed.output_outcome(incoming_out_point).unwrap() {
        OutputOutcome::Contract { outcome, .. } => {
            assert_eq!(
                outcome,
                ContractOutcome::Incoming(DecryptedPreimage::Pending)
            );
        }
        _ => panic!(),
    };

    let incoming_input = ContractInput {
        crontract_id: contract.contract_id(),
        amount: Amount::from_sat(42),
        witness: None,
    };
    let error = fed.verify_input(&incoming_input).unwrap_err();
    assert_eq!(error, LightningModuleError::ContractNotReady);

    fed.consensus_round(&[], &[]).await;
    match fed.output_outcome(incoming_out_point).unwrap() {
        OutputOutcome::Contract { outcome, .. } => {
            assert_eq!(
                outcome,
                ContractOutcome::Incoming(DecryptedPreimage::Some(
                    minimint_ln::contracts::incoming::Preimage(user_pk)
                ))
            );
        }
        _ => panic!(),
    };

    let meta = fed.verify_input(&incoming_input).unwrap();
    assert_eq!(meta.keys, vec![user_pk]);

    // TODO: test faulty encrypted preimage
}

pub struct FakeFed<M, CC> {
    members: Vec<(PeerId, M, MemDatabase)>,
    client_cfg: CC,
    max_evil: usize,
}

#[derive(Debug, PartialEq, Eq)]
struct TestInputMeta {
    amount: Amount,
    keys: Vec<secp256k1::schnorrsig::PublicKey>,
}

impl<M, CC> FakeFed<M, CC>
where
    M: FederationModule,
    M::ConsensusItem: Clone,
    M::Error: Debug + Eq,
    M::TxOutputOutcome: Eq + Debug,
{
    pub fn new<C, F>(
        members: usize,
        max_evil: usize,
        constructor: F,
        params: &C::Params,
    ) -> FakeFed<M, C::ClientConfig>
    where
        C: GenerateConfig,
        F: Fn(C, MemDatabase) -> M, // TODO: put constructor into Module trait
    {
        let peers = (1..=members)
            .map(|idx| PeerId::from(idx as u16))
            .collect::<Vec<_>>();
        let (server_cfg, client_cfg) = C::trusted_dealer_gen(
            &peers,
            max_evil,
            params,
            secp256k1::rand::rngs::OsRng::new().unwrap(),
        );

        let members = server_cfg
            .into_iter()
            .map(|(peer, cfg)| {
                let mem_db = MemDatabase::new();
                let member = constructor(cfg, mem_db.clone());
                (peer, member, mem_db)
            })
            .collect();

        FakeFed {
            members,
            client_cfg,
            max_evil,
        }
    }

    fn verify_input(&self, input: &M::TxInput) -> Result<TestInputMeta, M::Error> {
        let results = self.members.iter().map(|(_, member, _)| {
            let InputMeta { amount, puk_keys } = member.validate_input(input)?;
            Ok(TestInputMeta {
                amount,
                keys: puk_keys.collect(),
            })
        });
        assert_all_equal(results)
    }

    fn verify_output(&self, output: &M::TxOutput) -> bool {
        let results = self
            .members
            .iter()
            .map(|(_, member, _)| member.validate_output(output).is_err());
        assert_all_equal(results)
    }

    // TODO: add expected result to inputs/outputs
    async fn consensus_round(
        &mut self,
        inputs: &[M::TxInput],
        outputs: &[(OutPoint, M::TxOutput)],
    ) {
        let mut rng = secp256k1::rand::rngs::OsRng::new().unwrap();

        // TODO: only include some of the proposals for realism
        let mut consensus = vec![];
        for (id, member, _db) in &mut self.members {
            consensus.extend(
                member
                    .consensus_proposal(&mut rng)
                    .await
                    .into_iter()
                    .map(|ci| (*id, ci)),
            );
        }

        for (_peer, member, db) in &mut self.members {
            let mut batch = DbBatch::new();

            member
                .begin_consensus_epoch(batch.transaction(), consensus.clone(), &mut rng)
                .await;

            for input in inputs {
                member
                    .apply_input(batch.transaction(), input)
                    .expect("Faulty input");
            }

            for (out_point, output) in outputs {
                member
                    .apply_output(batch.transaction(), output, *out_point)
                    .expect("Faulty output");
            }

            (db as &mut dyn RawDatabase)
                .apply_batch(batch)
                .expect("DB error");

            let mut batch = DbBatch::new();
            member
                .end_consensus_epoch(batch.transaction(), &mut rng)
                .await;

            (db as &mut dyn RawDatabase)
                .apply_batch(batch)
                .expect("DB error");
        }
    }

    fn output_outcome(&self, out_point: OutPoint) -> Option<M::TxOutputOutcome> {
        // Since every member is in the same epoch they should have the same internal state, even
        // in terms of outcomes. This may change later once end_consensus_epoch is pulled out of the
        // main consensus loop into another thread to optimize latency. This test will probably fail
        // then.
        assert_all_equal(
            self.members
                .iter()
                .map(|(_, member, _)| member.output_status(out_point)),
        )
    }

    fn patch_dbs<U>(&mut self, update: U)
    where
        U: Fn(&mut dyn RawDatabase),
    {
        for (_, _, db) in &mut self.members {
            update(db);
        }
    }

    fn client_cfg(&self) -> &CC {
        &self.client_cfg
    }

    fn fetch_from_all<O, F>(&mut self, fetch: F) -> O
    where
        O: Debug + Eq,
        F: Fn(&mut M) -> O,
    {
        assert_all_equal(self.members.iter_mut().map(|(_, member, _)| fetch(member)))
    }
}

fn assert_all_equal<I>(mut iter: I) -> I::Item
where
    I: Iterator,
    I::Item: Eq + Debug,
{
    let first = iter.next().expect("empty iterator");
    while let Some(item) = iter.next() {
        assert_eq!(first, item);
    }
    first
}

/// Hack to set consensus height of wallet module which is being used by the LN module too for now.
fn set_block_height(db: &mut dyn RawDatabase, block_height: u32) {
    use minimint_api::encoding::{Decodable, Encodable};

    const DB_PREFIX_ROUND_CONSENSUS: u8 = 0x32;

    #[derive(Clone, Debug, Encodable, Decodable)]
    pub struct RoundConsensusKey;

    impl minimint_api::db::DatabaseKeyPrefixConst for RoundConsensusKey {
        const DB_PREFIX: u8 = DB_PREFIX_ROUND_CONSENSUS;
    }

    #[derive(Debug, Encodable, Decodable)]
    pub struct RoundConsensus {
        block_height: u32,
        fee_rate: u64,
        randomness_beacon: [u8; 32],
    }

    db.insert_entry(
        &RoundConsensusKey,
        &RoundConsensus {
            block_height,
            fee_rate: 0,
            randomness_beacon: [0; 32],
        },
    )
    .unwrap();
}
