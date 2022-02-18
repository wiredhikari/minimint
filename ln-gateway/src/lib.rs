mod ln;

use crate::ln::{LightningError, LnRpc};
use minimint::modules::ln::contracts::ContractId;
use mint_client::clients::gateway::{GatewayClient, GatewayClientError};
use rand::thread_rng;
use std::sync::Arc;
use thiserror::Error;
use tracing::{error, warn};

pub struct LnGateway {
    federation_client: Arc<GatewayClient>,
    ln_client: Arc<dyn LnRpc>,
    fetcher: tokio::task::JoinHandle<()>,
}

impl LnGateway {
    pub async fn new(
        mint_client: GatewayClient,
        ln_client: Box<dyn LnRpc + Sync + Send>,
    ) -> LnGateway {
        let ln_client: Arc<dyn LnRpc + Sync + Send> = ln_client.into();
        let mint_client = Arc::new(mint_client);
        let fetcher = tokio::spawn(background_fetch(mint_client.clone(), ln_client.clone()));

        LnGateway {
            federation_client: mint_client,
            ln_client,
            fetcher,
        }
    }

    pub async fn pay_invoice(&self, contract_id: ContractId) -> Result<(), LnGatewayError> {
        let contract_account = self
            .federation_client
            .fetch_outgoing_contract(contract_id)
            .await?;

        let payment_params = self
            .federation_client
            .validate_outgoing_account(&contract_account)
            .await?;

        self.federation_client
            .save_outgoing_payment(contract_account.clone());

        let preimage = match self
            .ln_client
            .pay(
                &contract_account.contract.invoice,
                payment_params.max_delay,
                payment_params.max_fee_percent,
            )
            .await
        {
            Ok(preimage) => preimage,
            Err(e) => {
                warn!("LN payment failed, aborting");
                self.federation_client.abort_outgoing_payment(contract_id);
                return Err(LnGatewayError::CouldNotRoute(e));
            }
        };

        // FIXME: figure out how to treat RNGs (maybe include in context?)
        self.federation_client
            .claim_outgoing_contract(contract_id, preimage, thread_rng())
            .await?;
        self.federation_client
            .await_claimed_outgoing_accepted(contract_id)
            .await;

        Ok(())
    }
}

/// This function runs as a background process fetching issued token signatures and driving forward
/// payments which were interrupted during execution.
async fn background_fetch(
    federation_client: Arc<GatewayClient>,
    _ln_client: Arc<dyn LnRpc + Send + Sync>,
) {
    // TODO: also try to drive forward payments that were interrupted
    loop {
        let pending_fetches = federation_client
            .list_fetchable_coins()
            .into_iter()
            .map(|out_point| {
                // TODO: get rid of cloning
                let federation_client = federation_client.clone();
                async move {
                    if let Err(e) = federation_client.fetch_coins(out_point).await {
                        error!("Fetching coins failed: {:?}", e);
                    }
                }
            })
            .collect::<Vec<_>>();
        futures::future::join_all(pending_fetches).await;
    }
}

impl Drop for LnGateway {
    fn drop(&mut self) {
        self.fetcher.abort();
        assert!(futures::executor::block_on(&mut self.fetcher).is_err());
    }
}

#[derive(Debug, Error)]
pub enum LnGatewayError {
    #[error("Federation operation error: {0:?}")]
    FederationError(GatewayClientError),
    #[error("Our LN node could not route the payment: {0:?}")]
    CouldNotRoute(LightningError),
}

impl From<GatewayClientError> for LnGatewayError {
    fn from(e: GatewayClientError) -> Self {
        LnGatewayError::FederationError(e)
    }
}
