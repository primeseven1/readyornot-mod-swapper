use ini::Ini;

pub fn get_ini_section_key_value(section: &str, key: &str) -> String {
    let config = Ini::load_from_file("modswap.ini").unwrap(); 

    let section = config.section(Some(section)).unwrap();
    let val: &str = section.get(key).unwrap();

    return val.to_string();
}

pub fn create_modswap_ini() {
    let mut conf = Ini::new();
    conf.with_section(Some("directories"))
        .set("ron_dir", "\"/path/to/ron/dir/Ready Or Not\"");
    conf.write_to_file("./modswap.ini").unwrap();
}
