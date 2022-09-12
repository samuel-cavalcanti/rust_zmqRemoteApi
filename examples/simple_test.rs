use zmq_remote_api::{RemoteAPIObjects, RemoteApiClientParams};

/*
    Example based on SimpleTest.py
*/

fn main() -> Result<(), zmq::Error> {
    println!("Program started");

    let client = zmq_remote_api::RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    let _sim = RemoteAPIObjects::new(&client);

    Ok(())
}
