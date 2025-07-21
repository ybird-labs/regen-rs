use tonic::transport::Channel;

use cosmos_sdk_proto::cosmos::crisis::v1beta1::msg_client::MsgClient;

pub struct CrisisClient {
    channel: Channel,
}

impl CrisisClient {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }
    pub fn tx(&self) -> MsgClient<Channel> {
        MsgClient::new(self.channel.clone())
    }
}