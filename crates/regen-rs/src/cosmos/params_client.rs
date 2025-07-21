use tonic::transport::Channel;


use cosmos_sdk_proto::cosmos::params::v1beta1::query_client::QueryClient;

pub struct ParamsClient {
    channel: Channel,
}

impl ParamsClient {
    pub fn new(channel: Channel) -> Self {
        Self { channel }
    }

    pub fn query(&self) -> QueryClient<tonic::transport::Channel> {
        QueryClient::new(self.channel.clone())
    }

}
