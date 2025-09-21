use super::config::get_config;
use super::textconstants;
use super::types::Folder;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::process::exit;

pub struct TodoApplication {
    is_running: bool,
    folders: Option<Vec<Folder>>,
    catalog_path: String,
    current_opened_folder: Option<Folder>,
}

impl TodoApplication {
    pub fn new() -> TodoApplication {
        TodoApplication {
            is_running: false,
            folders: None,
            catalog_path: String::new(),
            current_opened_folder: None,
        }
    }

    pub fn run(&mut self) {
        std::println!("{}", textconstants::greeting());

        self.load_from_config();

        std::println!("Catalog path: {}\n", self.catalog_path);

        self.is_running = true;
        while self.is_running {
            std::io::stdout()
                .write(self.create_prompt().as_bytes())
                .unwrap();
            std::io::stdout().flush().unwrap();
            let mut command = String::new();

            std::io::stdin().read_line(&mut command).unwrap();

            match command.strip_suffix("\n").unwrap().to_string().as_str() {
                "exit" | "q" | "quit" => {
                    std::println!("Quitting... Bye!");
                    self.quit();
                }
                "lf" => {
                    match &self.folders {
                        Some(folders) => {
                            std::println!("Available folders:");
                            for (index, folder) in folders.iter().enumerate() {
                                std::println!("\t{}. {}", index + 1, folder.title);
                            }
                        }

                        None => {
                            std::println!("No folders.");
                        }
                    }

                }
                "o" | "open"  => {}
                "e" | "edit" => {}
                "ln" => {}
                "c" | "create" => {}
                "cf" => {}
                "clear" => {
                    print!("\x1B[2J\x1B[1;1H");
                }
                "" => {}
                _ => {
                    std::println!("Unknown command!");
                }
            }
        }
    }

    fn set_new_catalog_path(&mut self) {
        std::println!("Enter new path to the notes catalog: ");
        std::io::stdin()
            .read_to_string(&mut self.catalog_path)
            .unwrap();
    }

    fn load_from_config(&mut self) {
        let mut config_file_str: String = String::new();
        let mut config_file = get_config();
        config_file.read_to_string(&mut config_file_str).unwrap();

        let mut json_config: serde_json::Value = serde_json::from_str(&config_file_str).unwrap();

        let catalog_path = json_config["catalog-path"]
            .as_str()
            .unwrap_or_else(|| {
                std::println!("Unable to read catalog path");
                exit(-1);
            })
            .to_string();

        if catalog_path.is_empty() {
            self.set_new_catalog_path();
            json_config["catalog_path"] = serde_json::Value::String(self.catalog_path.clone());
            match config_file.write(json_config.to_string().as_bytes()) {
                Ok(_) => {}

                Err(_) => {}
            }
        }
        match OpenOptions::new()
            .write(true)
            .read(true)
            .open(&catalog_path)
        {
            Ok(mut file) => {
                let mut str_from_cat: String = String::new();
                file.read_to_string(&mut str_from_cat).unwrap();
                let catalog_json: Result<serde_json::Value, serde_json::Error> =
                    serde_json::from_str(&str_from_cat);
                match catalog_json {
                    Ok(json) => {
                        self.catalog_path = catalog_path;
                        let mut folders_objs: Vec<serde_json::Value> = vec![];
                        let folders_objs_result = json["folders"].as_array();
                        match folders_objs_result {
                            Some(v) => {
                                folders_objs = v.to_vec();
                            }
                            None => {}
                        }
                        let mut folders: Vec<Folder> = vec![];
                        for obj in folders_objs {
                            folders.push(Folder::from_json(obj));
                        }

                        if folders.is_empty() {
                            self.folders = None;
                        } else {
                            self.folders = Some(folders);
                        }
                    }
                    Err(_) => loop {
                        print!("Unable to read new catalog. Recreate it from scratch (y/n) ?: ");
                        std::io::stdout().flush().unwrap();
                        let mut buf: String = String::new();
                        std::io::stdin().read_line(&mut buf).unwrap();

                        match buf.as_str().trim() {
                            "y" | "Y" => {
                                std::io::stdout()
                                    .write(b"Recreating JSON structure of catalog...\n")
                                    .unwrap();
                                std::io::stdout().flush().unwrap();
                                file.write(b"{\n\t\"folders\":[]\n}\n").unwrap();
                                self.catalog_path = catalog_path;
                                std::io::stdout()
                                    .write(b"Catalog JSON Structure recreated.\n\n")
                                    .unwrap();
                                std::io::stdout().flush().unwrap();
                                break;
                            }
                            "n" | "N" => {
                                std::println!("Unable to continue without catalog. Exiting...");
                                std::io::stdout().flush().unwrap();
                                self.quit();
                            }
                            _ => {}
                        }
                    },
                }
            }

            Err(_) => {
                panic!("Unable to read catalog file!");
            }
        }
    }

    fn create_prompt(&self) -> String {
        match &self.current_opened_folder {
            Some(folder) => format!("[{}]: ", folder.title).to_string(),

            None => String::from("[-]: "),
        }
    }

    fn quit(&mut self) {
        self.is_running = false;
    }
}
