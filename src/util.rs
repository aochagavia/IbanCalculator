pub fn m_proef(test: u32, modulo: u32) -> bool {
    let mut rest: u32 = test;
    let mut counter: u32 = 0;
    let mut index: u32 = 1;
    while rest != 0 {
        counter += (rest % 10) * index;
        rest /= 10;
        index += 1;
    }
    (counter % modulo) == 0
}

pub fn sha1_hex_to_bytes(hex: &str) -> Box<[u8; 20]> {
    // Since we want to extract 20 bytes, the original string needs
    // to provide 40 characters
    assert_eq!(hex.len(), 40);

    let mut buffer = [0; 20];
    let bytes = hex.as_bytes();

    let mut dummy_str = String::with_capacity(2);
    for i in 0..20 {
        // Read each pair of UTF8 hex characters as a single byte
        let first = i * 2;
        let second = first + 1;

        // Push them into a string and parse them as a hex number
        dummy_str.push(bytes[first] as char);
        dummy_str.push(bytes[second] as char);
        let byte = u8::from_str_radix(&dummy_str, 16).unwrap();
        dummy_str.clear();

        // Write the resulting byte in the buffer
        buffer[i] = byte;
    }

    Box::new(buffer)
}
