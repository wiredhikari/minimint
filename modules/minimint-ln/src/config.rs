use minimint_api::config::GenerateConfig;
use minimint_api::PeerId;
use secp256k1::rand::{CryptoRng, RngCore};
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct LightningModuleConfig {
    pub threshold_pub_keys: threshold_crypto::PublicKeySet,
    pub threshold_sec_key: threshold_crypto::SecretKeyShare,
    pub threshold: usize,
}

#[derive(Debug)]
pub struct LightningModuleClientConfig {
    pub threshold_pub_key: threshold_crypto::PublicKey,
}

impl GenerateConfig for LightningModuleConfig {
    type Params = ();
    type ClientConfig = LightningModuleClientConfig;

    fn trusted_dealer_gen(
        peers: &[PeerId],
        max_evil: usize,
        _params: &Self::Params,
        mut rng: impl RngCore + CryptoRng,
    ) -> (BTreeMap<PeerId, Self>, Self::ClientConfig) {
        let threshold = peers.len() - max_evil;
        let sks = threshold_crypto::SecretKeySet::random(threshold, &mut rng);
        let pks = sks.public_keys();

        let server_cfg = peers
            .iter()
            .map(|&peer| {
                let sk = sks.secret_key_share(peer.to_usize());

                (
                    peer,
                    LightningModuleConfig {
                        threshold_pub_keys: pks.clone(),
                        threshold_sec_key: sk,
                        threshold,
                    },
                )
            })
            .collect();

        let client_cfg = LightningModuleClientConfig {
            threshold_pub_key: pks.public_key(),
        };

        (server_cfg, client_cfg)
    }
}
