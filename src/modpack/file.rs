use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct ModpackFileHashes {
    pub sha1: String,
    pub sha512: String,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
pub struct ModpackFileEnv {
    pub client: String,
    pub server: String,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModpackFile {
    pub path: String,
    pub hashes: ModpackFileHashes,
    pub env: ModpackFileEnv,
    // pub download_url: Option<String>,
    pub downloads: Vec<String>,
    pub file_size: usize,
}

impl ModpackFileHashes {
    pub fn new(sha1: String, sha512: String) -> ModpackFileHashes {
        ModpackFileHashes { sha1, sha512 }
    }

    pub fn set_sha1(&mut self, sha1: String) {
        self.sha1 = sha1;
    }

    pub fn set_sha512(&mut self, sha512: String) {
        self.sha512 = sha512;
    }
}

impl ModpackFileEnv {
    pub fn new(client: String, server: String) -> ModpackFileEnv {
        ModpackFileEnv { client, server }
    }

    pub fn set_client(&mut self, client: String) {
        self.client = client;
    }

    pub fn set_server(&mut self, server: String) {
        self.server = server;
    }
}

impl ModpackFile {
    pub fn new(path: String, file_size: usize) -> ModpackFile {
        ModpackFile {
            path,
            hashes: ModpackFileHashes::new("".to_string(), "".to_string()),
            env: ModpackFileEnv::new("required".to_string(), "required".to_string()),
            downloads: Vec::new(),
            file_size,
        }
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn set_hashes(&mut self, hashes: ModpackFileHashes) {
        self.hashes = hashes;
    }

    pub fn set_env(&mut self, env: ModpackFileEnv) {
        self.env = env;
    }

    pub fn set_download_url(&mut self, download_url: String) {
        let mut list: Vec<String> = Vec::new();
        list.push(download_url);
        self.downloads = list;
    }

    pub fn set_file_size(&mut self, file_size: usize) {
        self.file_size = file_size;
    }
}
