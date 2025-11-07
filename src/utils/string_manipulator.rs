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

pub enum ElispsisDirection {
    Start,
    End,
    Middle,
}

pub fn elipsis(content: String, max_char: usize, direction: ElispsisDirection) -> String {
    if content.len() > max_char {
        match direction {
            ElispsisDirection::Start => {
                let mut elipsed = "...".to_string();
                elipsed.push_str(&content[content.len() - (max_char - 3)..]);
                elipsed
            }
            ElispsisDirection::End => {
                let mut elipsed = content[..max_char - 3].to_string();
                elipsed.push_str("...");
                elipsed
            }
            ElispsisDirection::Middle => {
                let part_length = (max_char - 3) / 2;
                let mut elipsed = content[..part_length].to_string();
                elipsed.push_str("...");
                elipsed.push_str(&content[content.len() - part_length..]);
                elipsed
            }
        }
    } else {
        content
    }
}
