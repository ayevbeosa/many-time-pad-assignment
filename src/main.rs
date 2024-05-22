fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks(2)
        .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap())
        .collect()
}

fn xorbytes(byte1: &[u8], byte2: &[u8]) -> Vec<u8> {
    byte1
        .iter()
        .zip(byte2.iter())
        .map(|(&x1, &x2)| x1 ^ x2)
        .collect()
}

fn bytes_to_string(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| b as char).collect()
}

fn main() {
    let message = std::fs::read_to_string("encrypted_text");
    let key = "Bitcoin: A purely peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution."
        .as_bytes();

    match message {
        Ok(message) => {
            let bytes = message
                .lines()
                .map(|hex| hex_to_bytes(hex))
                .map(|line| (xorbytes(&key, line.as_slice())))
                .collect::<Vec<Vec<u8>>>();

            bytes
                .iter()
                .for_each(|e| println!("{}", bytes_to_string(e)));
        }
        Err(e) => {
            println!("An error occurred: {}", e);
        }
    }
}
