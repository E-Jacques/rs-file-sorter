pub fn add_0_to_single_number (n: u32) -> String {
    let mut n_str = n.to_string();
    if n < 10 {
        let mut zero_str = String::from("0");
        zero_str.push_str(n_str.as_str());
        n_str = zero_str.to_owned();
    }

    n_str
}