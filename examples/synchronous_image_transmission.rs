use zmq_remote_api::{sim::Sim, RemoteApiClient, RemoteApiClientParams};

/* Based on synchronousImageTransmission.cpp example
 *
 * Make sure to have the add-on "ZMQ remote API" running in
 * CoppeliaSim and have following scene loaded:
 *
 * scenes/messaging/synchronousImageTransmissionViaRemoteApi.ttt
 *
 * Do not launch simulation, but run this script
 */

fn main() -> Result<(), zmq::Error> {
    println!("Program started");
    env_logger::init();

    let client = RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    let sim = Sim::new(&client);

    let vison_sensor_handle = sim.get_object("/VisionSensor".to_string(), None)?;

    let passive_vision_sensor_handle = sim.get_object("/PassiveVisionSensor".to_string(), None)?;

    client.set_stepping(true)?;

    sim.start_simulation()?;

    let start_time = sim.get_simulation_time()?;

    while sim.get_simulation_time()? - start_time < 5.0 {
        let (img, _res) = sim.get_vision_sensor_img(vison_sensor_handle, None, None, None, None)?;

        println!("size: {}", img.len());
        sim.set_vision_sensor_img(passive_vision_sensor_handle, img, None, None, None)?;
        client.step(true)?;
    }

    sim.stop_simulation()?;

    println!("Program ended");

    Ok(())
}
