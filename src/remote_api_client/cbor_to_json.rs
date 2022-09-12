use ciborium::value::Value as CborValue;
use serde_json::{value::Number as JsonNumber, Value as JsonValue};

fn cbor_into_string(cbor: CborValue) -> Option<String> {
    match cbor {
        CborValue::Text(string) => Some(string),
        _ => None,
    }
}

pub fn cbor_to_json(cbor: CborValue) -> JsonValue {
    match cbor {
        CborValue::Null => JsonValue::Null,
        CborValue::Bool(boolean) => JsonValue::Bool(boolean),
        CborValue::Text(string) => JsonValue::String(string),
        CborValue::Integer(int) => JsonValue::Number({
            let int: i128 = int.into();
            if let Ok(int) = u64::try_from(int) {
                JsonNumber::from(int)
            } else if let Ok(int) = i64::try_from(int) {
                JsonNumber::from(int)
            } else {
                JsonNumber::from_f64(int as f64).unwrap()
            }
        }),
        CborValue::Float(float) => JsonValue::Number(JsonNumber::from_f64(float).unwrap()),
        CborValue::Array(vec) => JsonValue::Array(vec.into_iter().map(cbor_to_json).collect()),
        CborValue::Map(map) => JsonValue::Object(
            map.into_iter()
                .map(|(k, v)| (cbor_into_string(k).unwrap(), cbor_to_json(v)))
                .collect(),
        ),
        CborValue::Bytes(_) | CborValue::Tag(_, _) => unimplemented!(),
        _ => unimplemented!(),
    }
}
