use crate::regen::data::v2::msg_client::MsgClient;
use crate::regen::data::v2::query_client::QueryClient;
use tonic::transport::Channel;

pub struct DataClient {
    channel: Channel,
}

impl DataClient {
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
