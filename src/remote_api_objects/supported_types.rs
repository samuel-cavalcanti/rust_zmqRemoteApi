pub enum CborArgConvert<T: serde::ser::Serialize> {
    VecU8(Vec<u8>),
    Other(T),
}

impl<T: serde::ser::Serialize> CborArgConvert<T> {
    pub fn to_cbor(self) -> ciborium::value::Value {
        match self {
            CborArgConvert::VecU8(vec) => ciborium::value::Value::Bytes(vec),
            CborArgConvert::Other(value) => ciborium::cbor!(value).unwrap(),
        }
    }
}

impl From<Vec<u8>> for CborArgConvert<Vec<u8>> {
    fn from(v: Vec<u8>) -> Self {
        Self::VecU8(v)
    }
}

macro_rules! impl_from_other {
    ($($type_arg:ty),+ $(,)?) => {

        $(
        impl From<$type_arg> for  CborArgConvert<$type_arg> {
            fn from(v: $type_arg) -> Self {
                Self::Other(v)
            }
        }

        )*

    };
}

impl_from_other!(
    f64,
    i64,
    Vec<f64>,
    Vec<i64>,
    Vec<String>,
    Vec<bool>,
    Vec<serde_json::Value>,
    bool,
    serde_json::Value,
    String,
);
