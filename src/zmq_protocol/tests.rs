use super::{RawRequest, ZmqRequest};

use super::{LANG, VERSION};

#[test]
fn test_get_sim() {
    let uuid = "a9bb7126-0d1f-4474-8801-078c094dcee9".to_string();
    let request = ZmqRequest::remote_api_info("sim".to_string(), uuid);
    let bytes =
        b"\xa6dfuncqzmqRemoteApi.infodargs\x81csimduuidx$a9bb7126-0d1f-4474-8801-078c094dcee9cver\x02dlangdrusteargsL\x01"
            .to_vec();
    let zmq_bytes = request.to_raw_request();
    assert_eq!(
        bytes,
        zmq_bytes,
        "\nzmq request:{}\n      bytes:{}",
        crate::log_utils::to_byte_array_string(&zmq_bytes),
        crate::log_utils::to_byte_array_string(&bytes)
    );
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
            args_l: 0,
        },
        ZmqRequest {
            func: "sim.stopSimulation".to_string(),
            args: vec![],
            ver: VERSION,
            lang: LANG.into(),
            uuid: uuid.clone(),

            args_l: 0,
        },
    ];

    let expected_bytes = vec![
        b"\xa6dfuncssim.startSimulationdargs\x80duuidx$a9bb7126-0d1f-4474-8801-078c094dcee9cver\x02dlangdrusteargsL\x00".to_vec(),
        b"\xa6dfuncrsim.stopSimulationdargs\x80duuidx$a9bb7126-0d1f-4474-8801-078c094dcee9cver\x02dlangdrusteargsL\x00".to_vec(),
    ];

    for (request, bytes) in requests.iter().zip(expected_bytes) {
        let zmq_bytes = request.to_raw_request();
        assert_eq!(
            zmq_bytes,
            bytes,
            "\nzmq request:{}\n      bytes:{}",
            crate::log_utils::to_byte_array_string(&zmq_bytes),
            crate::log_utils::to_byte_array_string(&bytes)
        );
    }
}
