pub fn add_0_to_single_number(n: u32) -> String {
    let mut n_str = n.to_string();
    if n < 10 {
        let mut zero_str = String::from("0");
        zero_str.push_str(n_str.as_str());
        n_str = zero_str.to_owned();
    }

    n_str
}

pub fn random_string(length: usize) -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    let random_string: String = (0..length)
        .map(|_| {
            let rng = rand::random_range(0..chars.len());
            chars[rng]
        })
        .collect();
    random_string
}
