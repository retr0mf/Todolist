use serde::Serialize;
use serde_json;

#[derive(Serialize, Debug)]
pub struct Note {
    title: String,
    contents: String,
}

impl Note {
    fn from_json(json_obj: serde_json::Value) -> Note {
        Note {
            title: json_obj["title"].as_str().unwrap().to_string(),
            contents: json_obj["contents"].as_str().unwrap().to_string()
        }
    }

    fn create(title: String, contents: String) -> Note {
        Note { title, contents }
    }
}
#[derive(Debug)]
pub struct Folder {
    pub title: String,
    pub notes: Vec<Note>,
}

impl Folder {
    pub fn new(new_title: String) -> Folder {
        Folder {
            title: new_title,
            notes: vec![]
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
}
