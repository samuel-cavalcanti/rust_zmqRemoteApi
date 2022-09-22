use std::ffi::c_void;
use zmq_remote_api::{sim::Sim, RemoteApiClient, RemoteApiClientParams,RemoteAPIError};
use opencv;
/* Based on opencv.py example
 *
 * Make sure to have the add-on "ZMQ remote API" running in
 * CoppeliaSim and have following scene loaded:
 *
 * scenes/messaging/synchronousImageTransmissionViaRemoteApi.ttt
 *
 * Do not launch simulation, but run this script
 */

fn main() -> Result<(),RemoteAPIError> {
    println!("Program started");
 

    let client = RemoteApiClient::new(RemoteApiClientParams {
        host: "localhost".to_string(),
        ..RemoteApiClientParams::default()
    })?;

    let sim = Sim::new(&client);

    let vison_sensor_handle = sim.get_object("/VisionSensor".to_string(), None)?;


    client.set_stepping(true)?;

    sim.start_simulation()?;

    let start_time = sim.get_simulation_time()?;
    let mut time = start_time;
    while time - start_time < 5.0 {
        let (img, res) = sim.get_vision_sensor_img(vison_sensor_handle, None, None, None, None)?;

        opencv_show_image(img,res);
       
        println!("time: {:.2}",time);
       
        client.step(true)?;
        time = sim.get_simulation_time()?;
    }

    sim.stop_simulation()?;

    println!("Program ended");

    Ok(())
}

fn opencv_show_image(image:Vec<u8>,resolution:Vec<i64>){

    let image_size = opencv::core::Size::new(resolution[0] as i32, resolution[1] as i32);
        
    let image = unsafe {// zerocopy, is unsafe in rust.
        let mut img = image;
        opencv::core::Mat::new_size_with_data(image_size,  opencv::core::CV_8UC3, img.as_mut_ptr()  as *mut c_void , opencv::core::Mat_AUTO_STEP).unwrap()
    };

    let mut dest = opencv::core::Mat::default();
    let mut dest2 = opencv::core::Mat::default();
    opencv::imgproc::cvt_color(&image, &mut dest, opencv::imgproc::COLOR_BGR2RGB, 0).unwrap();
    opencv::core::flip(&dest, &mut dest2,  0).unwrap();

    /*
        In CoppeliaSim images are left to right (x-axis), and bottom to top (y-axis)
        (consistent with the axes of vision sensors, pointing Z outwards, Y up)
        and color format is RGB triplets, whereas OpenCV uses BGR:
    */
     

    opencv::highgui::imshow("Opencv CoppeliaSim", &dest2).unwrap();
    opencv::highgui::wait_key(1).unwrap();
}