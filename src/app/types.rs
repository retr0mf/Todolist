use serde::Serialize;
use serde_json;

#[derive(Serialize, Debug, Clone)]
pub struct Note {
    pub title: String,
    pub contents: String,
}

impl Note {
    pub fn print(&self, index: usize) {
        const FIXED_WIDTH: i64 = 80;

        let mut limited_str_vec: Vec<String> = vec![];
        let mut words_vec: Vec<String> = self
            .contents
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        std::println!("[{}: {}]", index, self.title);

        words_vec.reverse();

        while !words_vec.is_empty() {
            let mut limited_string: Vec<String> = vec![];

            while (limited_string.len() < FIXED_WIDTH as usize) && !words_vec.is_empty() {
                let word = words_vec.pop().unwrap();
                if limited_string.join(" ").len() + word.len() < FIXED_WIDTH as usize {
                    limited_string.push(word);
                } else {
                    break;
                }
            }

            limited_str_vec.push(limited_string.join(" "));
        }

        for limstr in limited_str_vec {
            let mut printable: String = "[ ".to_owned() + limstr.as_str();
            let needed_spaces: i64 = FIXED_WIDTH - printable.len() as i64;
            if(needed_spaces > 0) {
                for _ in 0..needed_spaces {
                        printable += " ";
                }

                printable += " ]";
            }

            std::println!("{}", printable);
        }
    }

    fn from_json(json_obj: serde_json::Value) -> Note {
        Note {
            title: json_obj["title"].as_str().unwrap().to_string(),
            contents: json_obj["contents"].as_str().unwrap().to_string(),
        }
    }

    fn create(title: String, contents: String) -> Note {
        Note { title, contents }
    }
}
#[derive(Debug, Clone)]
pub struct Folder {
    pub title: String,
    pub notes: Vec<Note>,
}

impl Folder {
    pub fn new(new_title: String) -> Folder {
        Folder {
            title: new_title,
            notes: vec![],
        }
    }

    pub fn from_json(obj: serde_json::Value) -> Folder {
        let notes_objs: Vec<serde_json::Value> = obj["notes"].as_array().unwrap().to_vec();
        let mut notes: Vec<Note> = vec![];

        for object in notes_objs {
            notes.push(Note::from_json(object));
        }

        Folder {
            title: obj["title"].as_str().unwrap().to_string(),
            notes: notes,
        }
    }

    pub fn get_note_by_ind(&self, index: usize) -> &Note {
        &self.notes[index]
    }
}