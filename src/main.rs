use std::{env, process};
use ini::Ini;

mod mod_swap;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() != 2 {
        println!("Usage: {} /path/to/mod/paks", argv[0]);
        process::exit(1);
    }

    let config = Ini::load_from_file("modswap.ini").unwrap(); 
    let directories = config.section(Some("directories")).unwrap();
    let ron_dir = directories.get("ron_dir").unwrap();

    mod_swap::do_mod_swap(&ron_dir.to_string(), &argv[1]);
}
