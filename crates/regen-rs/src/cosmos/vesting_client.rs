use tonic::transport::Channel;

use cosmos_sdk_proto::cosmos::vesting::v1beta1::msg_client::MsgClient;


pub struct VestingClient {
    channel: Channel,
}

impl VestingClient {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    pub fn tx(&self) -> MsgClient<Channel> {
        MsgClient::new(self.channel.clone())
    }
}
