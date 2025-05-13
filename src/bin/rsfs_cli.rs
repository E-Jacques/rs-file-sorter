use std::env;

use rs_file_sorter::cli::handle;

fn main() {
    let input = env::args().skip(1).collect::<Vec<String>>().join(" ");
    handle(input, None);
}
