use zmq_remote_api::{sim::Sim, RemoteAPIError, RemoteApiClient, RemoteApiClientParams};

/* Based on synchronousImageTransmission.cpp example
 *
 * Make sure to have the add-on "ZMQ remote API" running in
 * CoppeliaSim and have following scene loaded:
 *
 * scenes/messaging/synchronousImageTransmissionViaRemoteApi.ttt
 *
 * Do not launch simulation, but run this script
 */

fn main() -> Result<(), RemoteAPIError> {
    println!("Program started");
    env_logger::init();

    let client = RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    let vision_sensor_handle = client.sim_get_object("/VisionSensor".to_string(), None)?;

    let passive_vision_sensor_handle =
        client.sim_get_object("/PassiveVisionSensor".to_string(), None)?;

    client.sim_set_stepping(true)?;

    client.sim_start_simulation()?;

    let start_time = client.sim_get_simulation_time()?;

    while client.sim_get_simulation_time()? - start_time < 5.0 {
        let (img, _res) =
            client.sim_get_vision_sensor_img(vision_sensor_handle, None, None, None, None)?;

        client.sim_set_vision_sensor_img(passive_vision_sensor_handle, img, None, None, None)?;
        client.sim_step()?;
    }

    client.sim_stop_simulation()?;

    println!("Program ended");

    Ok(())
}
