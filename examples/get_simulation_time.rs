use zmqRemoteApi::{RemoteAPIObjects, RemoteApiClient, RemoteApiClientParams};


fn main()  {
    let client = zmqRemoteApi::RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })
    .unwrap();

    // let sim = RemoteAPIObjects::new(client);

    let result = client.get_object("sim".to_string()).unwrap();

    println!("{}", result);

   
}
