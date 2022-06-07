use crate::modpack::file::ModpackFile;
use serde::{Deserialize, Serialize};

use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Modpack {
    pub format_version: i32,
    pub game: String,
    pub version_id: String,
    pub name: String,
    pub summary: String,
    pub files: HashSet<ModpackFile>,
    pub dependencies: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InternalModpackFileFormat {
    pub path: String,
    pub sha1: String,
    pub sha512: String,
    pub url: String,
    pub size: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InternalModpackFormat {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub files: Vec<InternalModpackFileFormat>,
    pub mods: Vec<InternalModpackFileFormat>,
}

impl InternalModpackFormat {
    pub fn add_file(&mut self, file: InternalModpackFileFormat) {
        self.files.push(file);
    }

    pub fn add_mod(&mut self, mod_file: InternalModpackFileFormat) {
        self.mods.push(mod_file);
    }
}

trait StrExt {
    fn remove_last(&self) -> &str;
}

impl StrExt for str {
    fn remove_last(&self) -> &str {
        match self.char_indices().next_back() {
            Some((i, _)) => &self[..i],
            None => self,
        }
    }
}

impl Modpack {
    pub fn new() -> Modpack {
        Modpack {
            format_version: 1,
            game: "minecraft".to_string(),
            version_id: "".to_string(),
            name: "".to_string(),
            summary: "".to_string(),
            files: HashSet::new(),
            dependencies: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, file: ModpackFile) {
        self.files.insert(file);
    }

    pub fn remove_file(&mut self, file: ModpackFile) {
        self.files.remove(&file);
    }

    pub fn add_dependency(&mut self, key: String, value: String) {
        self.dependencies.insert(key, value);
    }

    pub fn remove_dependency(&mut self, key: String) {
        self.dependencies.remove(&key);
    }

    pub fn set_version_id(&mut self, version_id: String) {
        self.version_id = version_id;
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_summary(&mut self, summary: String) {
        self.summary = summary;
    }
}
