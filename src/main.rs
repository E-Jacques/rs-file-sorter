use rs_file_sorter::handle;
use std::env;

fn main() {
    let input = env::args().collect::<Vec<String>>().join(" ");
    handle(input);
}
