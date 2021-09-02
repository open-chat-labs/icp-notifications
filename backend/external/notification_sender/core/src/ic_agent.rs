use crate::notification_reader::NotificationReader;
use async_trait::async_trait;
use candid::{Decode, Encode};
use garcon::ThrottleWaiter;
use ic_agent::agent::http_transport::ReqwestHttpReplicaV2Transport;
use ic_agent::identity::BasicIdentity;
use ic_agent::{Agent, Identity};
use notifications_canister_api::{notifications, remove_notifications};
use std::time::Duration;
use types::{CanisterId, Error};

const MAX_NOTIFICATIONS_PER_BATCH: u32 = 200;

pub struct IcAgentConfig {
    pub ic_url: String,
    pub ic_identity_pem: String,
    pub fetch_root_key: bool,
}

pub struct IcAgent {
    agent: Agent,
    canister_id: CanisterId,
}

impl IcAgent {
    pub async fn build(config: &IcAgentConfig, canister_id: CanisterId) -> Result<IcAgent, Error> {
        let transport = ReqwestHttpReplicaV2Transport::create(&config.ic_url)?;
        let timeout = std::time::Duration::from_secs(60 * 5);

        let agent = Agent::builder()
            .with_transport(transport)
            .with_boxed_identity(Self::get_identity(&config.ic_identity_pem))
            .with_ingress_expiry(Some(timeout))
            .build()?;

        if config.fetch_root_key {
            agent.fetch_root_key().await?;
        }

        Ok(IcAgent { agent, canister_id })
    }

    /// Returns an identity derived from the private key.
    fn get_identity(pem: &str) -> Box<dyn Identity + Sync + Send> {
        match BasicIdentity::from_pem(pem.as_bytes()) {
            Ok(identity) => Box::new(identity),
            Err(error) => {
                eprintln!(
                    "Couldn't load identity from PEM file. {:?}. Input: {:?}",
                    error,
                    pem.as_bytes()
                );
                std::process::exit(1);
            }
        }
    }
}

#[async_trait]
impl NotificationReader for IcAgent {
    async fn get(&self, from_index: u64) -> Result<notifications::SuccessResult, Error> {
        let args = notifications::Args {
            from_index,
            max_results: MAX_NOTIFICATIONS_PER_BATCH,
        };

        let response = self
            .agent
            .query(&self.canister_id, "notifications")
            .with_arg(Encode!(&args)?)
            .call()
            .await?;

        match Decode!(&response, notifications::Response)? {
            notifications::Response::Success(result) => Ok(result),
            notifications::Response::NotAuthorized => Err("Not authorized".into()),
        }
    }

    async fn remove(&self, up_to_index: u64) -> Result<(), Error> {
        let args = remove_notifications::Args { up_to_index };

        let request_id = self
            .agent
            .update(&self.canister_id, "remove_notifications")
            .with_arg(Encode!(&args)?)
            .call()
            .await?;

        let waiter = ThrottleWaiter::new(Duration::from_secs(1));
        let response_bytes = self
            .agent
            .wait(request_id, &self.canister_id, waiter)
            .await?;

        match Decode!(&response_bytes, remove_notifications::Response)? {
            remove_notifications::Response::Success => Ok(()),
            remove_notifications::Response::NotAuthorized => Err("Not authorized".into()),
        }
    }
}
