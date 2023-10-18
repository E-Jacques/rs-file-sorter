pub fn add_0_to_single_number (n: u32) -> String {
    let mut n_str = n.to_string();
    if n < 10 {
        let mut zero_str = String::from("0");
        zero_str.push_str(n_str.as_str());
        n_str = zero_str.to_owned();
    }

    n_str
}

#[test]
fn add_0_to_single_number_test () {
    let value = add_0_to_single_number(0);
    assert_eq!(value, String::from("00"));

    let value = add_0_to_single_number(9);
    assert_eq!(value, String::from("09"));

    let value = add_0_to_single_number(10);
    assert_eq!(value, String::from("10"));
    
    let value = add_0_to_single_number(100);
    assert_eq!(value, String::from("100"));
}