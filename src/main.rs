use std::{env, process, fs};

mod mod_swap;
mod config;

fn main() {
    let argv: Vec<String> = env::args().collect();

    /* Check if the file exists, and then tell the user to update it */
    if !fs::metadata("./modswap.ini").is_ok() {
        config::create_modswap_ini();
        println!("Created modswap.ini file, set the ready or not directory in the ron_dir key in the empty quotes");
        process::exit(1);
    }

    /* Need the path to the mods the user wants to install */
    if argv.len() != 2 {
        println!("Usage: {} /path/to/mod/paks", argv[0]);
        process::exit(1);
    }

    let ron_dir: String = config::get_ini_section_key_value("directories", "ron_dir");
    mod_swap::do_mod_swap(&ron_dir.to_string(), &argv[1]);

    println!("Mod swap completed successfully");
}
