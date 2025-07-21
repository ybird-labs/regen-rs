use tonic::transport::Channel;

use cosmos_sdk_proto::cosmos::authz::v1beta1::msg_client::MsgClient;
use cosmos_sdk_proto::cosmos::authz::v1beta1::query_client::QueryClient;

pub struct AuthZClient {
    channel: Channel,
}

impl AuthZClient {
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
