use crate::regen::ecocredit::v1::msg_client::MsgClient;
use crate::regen::ecocredit::v1::query_client::QueryClient;
use tonic::transport::Channel;

pub struct EcoCreditClient {
    channel: Channel,
}

impl EcoCreditClient {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    pub fn query(&self) -> QueryClient<tonic::transport::Channel> {
        QueryClient::new(self.channel.clone())
    }

    pub fn tx(&self) -> MsgClient<tonic::transport::Channel> {
        MsgClient::new(self.channel.clone())
    }
}
