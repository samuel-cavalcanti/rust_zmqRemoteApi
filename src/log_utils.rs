// pub fn bytes_to_hex_format(bytes: &Vec<u8>) -> Option<String> {
//     bytes
//         .iter()
//         .map(|byte| format!("{:X?}", byte)) // byte to string in heX. :X? means heX format
//         .reduce(|accum, item| {
//             // // joining hex numbers in single string
//
//             let mut result = accum;
//             result.push_str(&item);
//             result
//         })
// }

pub fn to_byte_array_string(bytes: &Vec<u8>) -> Result<String, std::string::FromUtf8Error> {
    let ascii_format = bytes
        .iter()
        .map(|b| std::ascii::escape_default(*b))
        .flatten()
        .collect();
    let string = String::from_utf8(ascii_format)?;

    Ok(format!("b'{}'", string))
}
