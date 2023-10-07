use crate::zmq_requests::{RawRequest, ZmqRequest};

use super::{LANG, VERSION};

#[test]
fn test_get_sim() {
    let uuid = "a9bb7126-0d1f-4474-8801-078c094dcee9".to_string();
    let request = ZmqRequest::remote_api_info("sim".to_string(), uuid);
    let bytes = b"\xa2dfuncqzmqRemoteApi.infodargs\x81csim".to_vec();
    assert_eq!(bytes, request.to_raw_request());
}


#[test]
fn test_requests_macros() {
    let uuid = "a9bb7126-0d1f-4474-8801-078c094dcee9".to_string();

    let requests = vec![
        ZmqRequest {
            func: "sim.startSimulation".to_string(),
            args: vec![],
            uuid: uuid.clone(),
            ver: VERSION,
            lang: LANG.into(),
        },
        ZmqRequest {
            func: "sim.stopSimulation".to_string(),
            args: vec![],
            ver: VERSION,
            lang: LANG.into(),
            uuid: uuid.clone(),
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
