#[cfg(test)]
use crate::zmq_requests::{RawRequest, ZmqRequest};

#[test]
fn test_get_sim() {
    let request = ZmqRequest::remote_api_info("sim".to_string());
    let bytes = b"\xa2dfuncqzmqRemoteApi.infodargs\x81csim".to_vec();
    assert_eq!(bytes, request.to_raw_request());
}

#[test]
fn test_get_step() {
    let request = ZmqRequest::step("2b2d55e0-576c-4dce-86c5-c3b7a3df0d73".to_string());
    let bytes = b"\xa2dfuncdstepdargs\x81x$2b2d55e0-576c-4dce-86c5-c3b7a3df0d73".to_vec();
    assert_eq!(bytes, request.to_raw_request());
}

#[test]
fn test_set_stepping() {
    let request =
        ZmqRequest::set_stepping(true, "a9bb7126-0d1f-4474-8801-078c094dcee9".to_string());

    let bytes =
        b"\xa2dfuncksetSteppingdargs\x82\xf5x$a9bb7126-0d1f-4474-8801-078c094dcee9".to_vec();

    assert_eq!(bytes, request.to_raw_request());
}

#[test]
fn test_requests_macros() {
    let requests = vec![
        ZmqRequest {
            function_name: format!("sim.startSimulation"),
            args: vec![],
        },
        ZmqRequest {
            function_name: format!("sim.stopSimulation"),
            args: vec![],
        },
    ];

    let expected_bytes = vec![
        b"\xa2dfuncssim.startSimulationdargs\x80".to_vec(),
        b"\xa2dfuncrsim.stopSimulationdargs\x80".to_vec(),
    ];

    for (request, bytes) in requests.iter().zip(expected_bytes) {
        assert_eq!(bytes, request.to_raw_request());
    }
}
