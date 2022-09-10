use crate::{RemoteApiClient, remote_api_client::RemoteApiClientParams};

pub struct RemoteAPIObjects {
    client: RemoteApiClient,
}

impl RemoteAPIObjects {
    pub fn new(client: RemoteApiClient) -> RemoteAPIObjects {
        RemoteAPIObjects { client: client }
    }
}

impl Default for RemoteAPIObjects {
    fn default() -> Self {
        Self {
            client: RemoteApiClient::new(RemoteApiClientParams::default()).unwrap(),
        }
    }
}
