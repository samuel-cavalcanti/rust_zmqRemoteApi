pub fn bytes_to_hex_format(bytes: &Vec<u8>) -> Option<String> {
    bytes
        .iter()
        .map(|byte| format!("{:X?}", byte)) // byte to string in heX. :X? means heX format
        .reduce(|accum, item| {
            // // joining hex numbers in single string

            let mut result = accum;
            result.push_str(&item);
            result
        })
}
