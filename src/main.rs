fn xorstrings(str1: &str, str2: &str) -> String {
    let mut result = String::new();
    for (byte1, byte2) in str1.bytes().zip(str2.bytes()) {
        result.push((byte1 ^ byte2).into());
    }
    result
}

fn dragging(cipher: &str, word: &str) {
    for cons in cipher.chars().collect::<Vec<char>>().windows(word.len()) {
        let cons_str: String = cons.iter().collect();
        let result = xorstrings(word, &cons_str);
        // println!("{} => {}", result, possible_randomness(&result));
        println!("{}", result);
    }
}

// fn possible_randomness(s: &str) -> f64 {
//     let mut total_difference = 0;
//     let mut count = 0;
//     let cleaned_str = s.replace(" ", "");
//     for (a, b) in cleaned_str.chars().zip(cleaned_str.chars().skip(1)) {
//         total_difference += (a as i32 - b as i32).abs();
//         count += 1;
//     }
//     total_difference as f64 / count as f64
// }

fn main() {
    let content = std::fs::read_to_string("encrypted_text");

    match content {
        Ok(content) => {
            let lines = content
                .lines()
                .map(hex::decode)
                .map(Result::unwrap)
                .map(|s| String::from_utf8_lossy(&s).to_string())
                .collect::<Vec<String>>();

            let cipher = xorstrings(&lines[0], &lines[1]);
            for word in [
                " the ",
                " brink ",
                " the heroes "
            ] {
                println!("Checking with {}", word);
                dragging(&cipher, word);
            }
        }
        Err(e) => {
            println!("An error occurred: {}", e);
        }
    }
}
