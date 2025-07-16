use std::time::Duration;
use tonic::transport::Channel;

use crate::cosmos::bank_client::BankClient;
use crate::error::{RegenError, Result};
use crate::regen::data_client::DataClient;
use crate::regen::eco_credit_client::EcoCreditClient;
use crate::signer::Signer;

pub struct Client {
    pub config: ClientConfig,
    pub channel: Channel,
    pub signer: Option<Signer>,
}

impl Client {
    pub async fn new(config: ClientConfig, signer: Option<Signer>) -> Result<Self> {
        let channel = Channel::from_shared(config.grpc_endpoint.clone())
            .map_err(|e| RegenError::Config(e.to_string()))?
            .connect()
            .await?;
        Ok(Self {
            config,
            channel,
            signer,
        })
    }

    // regen clients
    pub fn data(&self) -> DataClient {
        DataClient::new(self.channel.clone())
    }

    pub fn eco_credit(&self) -> EcoCreditClient {
        EcoCreditClient::new(self.channel.clone())
    }

    // cosmos clients

    pub fn bank(&self) -> BankClient {
        BankClient::new(self.channel.clone())
    }
}

#[derive(Default)]
pub struct ClientBuilder {
    config: ClientConfig,
    signer: Option<Signer>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn config(mut self, config: ClientConfig) -> Self {
        self.config = config;
        self
    }

    pub fn signer(mut self, signer: Signer) -> Self {
        self.signer = Some(signer);
        self
    }

    pub async fn build(self) -> Result<Client> {
        let client = Client::new(self.config, self.signer).await?;
        Ok(client)
    }
}

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub grpc_endpoint: String,
    pub chain_id: String,
    pub timeout: Duration,
    pub connect_timeout: Duration,
    pub gas_price: Option<f64>,
    pub gas_limit: u64,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            grpc_endpoint: "http://localhost:9090".to_string(),
            chain_id: "regen".to_string(),
            timeout: Duration::from_secs(30),
            connect_timeout: Duration::from_secs(10),
            gas_price: None,
            gas_limit: 200_000,
        }
    }
}

#[derive(Default)]
pub struct ClientConfigBuilder {
    grpc_endpoint: Option<String>,
    chain_id: Option<String>,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    gas_price: Option<f64>,
    gas_limit: Option<u64>,
}

impl ClientConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn grpc_endpoint(mut self, grpc_endpoint: &str) -> Self {
        self.grpc_endpoint = Some(grpc_endpoint.to_string());
        self
    }

    pub fn chain_id(mut self, chain_id: &str) -> Self {
        self.chain_id = Some(chain_id.to_string());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn connect_timeout(mut self, connect_timeout: Duration) -> Self {
        self.connect_timeout = Some(connect_timeout);
        self
    }

    pub fn gas_price(mut self, gas_price: f64) -> Self {
        self.gas_price = Some(gas_price);
        self
    }

    pub fn gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = Some(gas_limit);
        self
    }

    pub fn build(self) -> ClientConfig {
        // Default configuration values
        let default_config = ClientConfig::default();

        // Merge provided values with defaults
        ClientConfig {
            grpc_endpoint: self.grpc_endpoint.unwrap_or(default_config.grpc_endpoint),
            chain_id: self.chain_id.unwrap_or(default_config.chain_id),
            timeout: self.timeout.unwrap_or(default_config.timeout),
            connect_timeout: self
                .connect_timeout
                .unwrap_or(default_config.connect_timeout),
            gas_price: self.gas_price.or(default_config.gas_price),
            gas_limit: self.gas_limit.unwrap_or(default_config.gas_limit),
        }
    }
}
