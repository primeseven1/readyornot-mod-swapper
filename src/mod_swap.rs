use std::process::Command;

fn find_mod_files(dir: &String) -> Vec<String> {
    let cmd = Command::new("find")
        .arg(dir)
        .arg("-type")
        .arg("f")
        .arg("-name")
        .arg("*Mods_*_P.pak")
        .arg("-print")
        .output();

    let files: String;

    match cmd {
        Ok(output) => {
            if output.status.success() {
                files = String::from_utf8_lossy(&output.stdout).to_string();
            } else {
                let result_stderr = String::from_utf8_lossy(&output.stderr);
                panic!("Error finding files:\n{}", result_stderr);
            }
        }
        Err(e) => panic!("Error finding files: {}", e),
    }

    let mut files_vector: Vec<&str> = files.split("\n").collect();
    /* The last element is empty, so just remove it to avoid errors */
    files_vector.pop();

    let mut files_vector_string: Vec<String> = vec![];

    for file in files_vector {
        files_vector_string.push(file.to_string());
    } 

    return files_vector_string;
}

fn remove_current_mods(files: Vec<String>) {
    for file in files.iter() {
        let cmd = Command::new("rm")
            .arg(&file)
            .output();
        match cmd {
            Ok(output) => {
                if output.status.success() {
                    println!("Removed {}", file)
                } else {
                    let result_stderr = String::from_utf8_lossy(&output.stderr);
                    panic!("Error removing files:\n{}", result_stderr);
                }
            } Err(e) => panic!("Error removing files {}", e),
        }
    }

}

fn install_new_mods(game_dir: &String, swap_dir: &String) {
    let ron_mod_folder: String = format!("{}/ReadyOrNot/Content/Paks/", game_dir);
    
    let mods: Vec<String> = find_mod_files(swap_dir);
    
    for mod_file in mods {
        let cmd = Command::new("cp")
            .arg(&mod_file)
            .arg(&ron_mod_folder)
            .output();
        match cmd {
            Ok(output) => {
                if output.status.success() {
                    println!("Installed {}", mod_file)
                } else {
                    let result_stderr = String::from_utf8_lossy(&output.stderr);
                    panic!("Error installing files:\n{}", result_stderr);
                }
            } Err(e) => panic!("Error installing files {}", e),
        }

    }
}

pub fn do_mod_swap(game_dir: &String, swap_dir: &String) {
    let ron_mod_folder = format!("{}/ReadyOrNot/Content/Paks", game_dir);

    let installed_mods: Vec<String> = find_mod_files(&ron_mod_folder);
    remove_current_mods(installed_mods);
    install_new_mods(game_dir, swap_dir);
}
