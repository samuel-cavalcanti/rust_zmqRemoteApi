macro_rules! requests {

    ( $sim_type:literal, $(
        ($rust_fn:ident,$function_name:literal $(, ( $($arg_id:ident : $arg_type:ty),+ )  )? $(, opt( $($opt_arg_id:ident : $opt_arg_type:ty),+   ) )?  $(->$return_type:ty)? )
    ),+ $(,)? ) => {
        $(




            #[allow(dead_code,clippy::too_many_arguments)]
             fn $rust_fn(&self, $( $($arg_id:$arg_type,)*  )* $( $($opt_arg_id:Option<$opt_arg_type>,)*  )*   ) -> Result<$($return_type)*, crate::remote_api_objects::connection_error::RemoteAPIError>  {//

                let mut _brk = false;
                /*
                    CborArgConvert::from($arg_id).to_cbor()
                    converting the arg type properly.
                    ciborium::cbor!(value) is transforming Vec<u8> in an array of integers,
                 */
                let mut _args = vec![$($(crate::remote_api_objects::cbor_arg_convert::CborArgConvert::from($arg_id).into_cbor()),*)* ]; //


                $(
                    $(
                        if let Some(option) = $opt_arg_id{
                            let option = ciborium::cbor!(option).unwrap();
                            if _brk{
                                panic!("no gaps allowed");
                            }
                            _args.push(option);

                        }
                        else{
                            _brk = true;
                        }
                    )*
                )*

                let request = crate::zmq_requests::ZmqRequest {
                    function_name: format!("{}.{}",$sim_type,$function_name),
                    args: _args,
                };



                let result = self.send_raw_request(request.to_raw_request());


                if let Err(error) = result {
                    return Err(crate::remote_api_objects::connection_error::RemoteAPIError::new(format!("{:?}",error)));
                }

                let result = result.unwrap();

                let get_error = |json: &serde_json::Value| -> String{
                    if let Some(serde_json::Value::String(error)) = json.get("error") {
                        error.clone()
                    } else {
                        "unknown error".to_string()
                    }
                };

                let is_success = |json: &serde_json::Value| -> Result<(), String>{

                    if let Some(serde_json::Value::Bool(success)) = json.get("success") { if *success {
                            Ok(())
                        } else {
                            Err(get_error(json))
                        }
                    } else {
                        Err(get_error(json))
                    }
                };

                if let Err(error) = is_success(&result) {
                    return Err(crate::remote_api_objects::connection_error::RemoteAPIError::new(error));
                }

                let mut ret  =result["ret"].to_owned();
               $(

                if let Some(vec) = ret.as_array_mut() {
                    if vec.len() == 1{

                        log::trace!("vec: {:?} return type: {}", vec,stringify!($return_type));
                        let json_item = vec.remove(0);
                        let value:Result<$return_type, serde_json::Error> = serde_json::from_value(json_item);

                        match value {
                            Ok(value) => {
                                  // return a single value
                                    return Ok(value);
                            },
                            Err(e) => {
                                log::trace!("Err {}",e);

                                // Expected return ()
                                return Ok(Default::default());
                            },
                        }


                    }

                }

                let value:Result<$return_type, serde_json::Error> = serde_json::from_value(ret);

                match value {
                    Ok(value) => {
                          // Expected return tuple
                            Ok(value)
                    },           // Expected return ()
                    Err(_) => Ok(Default::default()),
                }

               )+

            }
        )*
    };

}
