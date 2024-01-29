use std::fs;
use std::path::{Path, PathBuf};
use glob::glob;

fn find_mod_files(dir: &String) -> Vec<String> {
    let pattern = format!("{}/*Mods_*_P.pak", dir);

    let files: Vec<String> = glob(&pattern)
        .expect("Failed to read glob pattern")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.is_file())
        .flat_map(|entry|entry.into_os_string().into_string())
        .collect();

    return files;
}

fn remove_current_mods(files: Vec<String>) {
    for file in files { 
        println!("Removing file {}", file);

        if let Err(err) = fs::remove_file(&file) {
            panic!("Error removing file {}: {}", file, err);
        }
    }
}

fn install_new_mods(game_dir: &String, swap_dir: &String) {
    let ron_mod_folder: PathBuf = Path::new(game_dir).join("ReadyOrNot/Content/Paks/");

    let mods: Vec<String> = find_mod_files(swap_dir);

    for mod_file in mods {
        let dest = Path::new(&ron_mod_folder).join(Path::new(&mod_file).file_name().unwrap());

        match fs::copy(&mod_file, &dest) {
            Ok(_) => println!("Installed {} into {}", &mod_file, &ron_mod_folder.to_string_lossy()),
            Err(err) => panic!("Failed to install {}: {}", &mod_file, err),
        }
    }
}

pub fn do_mod_swap(game_dir: &String, swap_dir: &String) {
    let ron_mod_folder = format!("{}/ReadyOrNot/Content/Paks", game_dir);

    let installed_mods: Vec<String> = find_mod_files(&ron_mod_folder);
    remove_current_mods(installed_mods);
    install_new_mods(game_dir, swap_dir);
}
