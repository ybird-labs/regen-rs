use tonic::transport::Channel;

use cosmos_sdk_proto::cosmos::bank::v1beta1::msg_client::MsgClient;
use cosmos_sdk_proto::cosmos::bank::v1beta1::query_client::QueryClient;

pub struct BankClient {
    channel: Channel,
}

impl BankClient {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    pub fn query(&self) -> QueryClient<tonic::transport::Channel> {
        QueryClient::new(self.channel.clone())
    }

    pub fn tx(&self) -> MsgClient<Channel> {
        MsgClient::new(self.channel.clone())
    }
}
