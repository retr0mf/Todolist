use std::fs::File;

pub fn get_config() -> File {

    let config_file_path_dir = String::from("/home/retr0/.config/rtdl");
    let config_file_path_file = String::from("config.json");
    let config_file_path = config_file_path_dir.clone() + "/" + &config_file_path_file;

    match File::open(&config_file_path) {
        Ok(file) => file,
        Err(err) => {
            std::println!(
                "Unable to read default config file. {:?}.\nCreating a new one...",
                err.kind().to_string()
            );

            std::fs::create_dir_all(&config_file_path_dir).unwrap();
            File::create_new(config_file_path).unwrap()
        }
    }
}