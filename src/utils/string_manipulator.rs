
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
