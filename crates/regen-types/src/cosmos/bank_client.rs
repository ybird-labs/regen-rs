use tonic::transport::Channel;


use cosmrs::bank::v1beta1::query_client::QueryClient;
use cosmrs::bank::v1beta1::msg_client::MsgClient;

pub struct BankClient {
    channel: Channel,
}

impl BankClient {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    pub fn query(&self) -> QueryClient<Channel> {
        QueryClient::new(self.channel.clone())
    }

    pub fn tx(&self) -> MsgClient<Channel> {
        MsgClient::new(self.channel.clone())
    }
}