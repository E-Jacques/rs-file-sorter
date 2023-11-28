use rs_file_sorter::handle;
use std::env;

fn main() {
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    handle(input, None);
}
