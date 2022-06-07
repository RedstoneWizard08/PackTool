use std::{
    collections::HashMap,
    io::{Read, Write},
};

use crate::{
    logging::Logger,
    modpack::pack::{InternalModpackFileFormat, InternalModpackFormat},
};

use data_encoding::HEXLOWER;
use sha1::{Digest, Sha1};
use sha2::Sha512;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with(".") || s.starts_with("target") || s.starts_with("build") || s.ends_with(".toml"))
        .unwrap_or(false)
}

fn is_not_mod(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| !s.starts_with("mods") && !s.ends_with(".toml"))
        .unwrap_or(false)
}

pub fn refresh(args: Vec<&str>, _options: HashMap<String, &str>) {
    let logger = Logger::new();

    let mut dir: String;

    if args.len() > 1 {
        dir = args.get(1).unwrap().to_string();

        if dir == ".".to_string() {
            dir = std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
        }
    } else {
        dir = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
    }

    logger.info("Refreshing files...");

    let dir2 = dir.clone();

    let mut config_file = std::fs::File::open(format!("{}/modpack.toml", &dir)).unwrap();

    let mut raw_config = "".to_string();
    config_file.read_to_string(&mut raw_config).unwrap();

    let contents = raw_config.as_str();
    let mut config: InternalModpackFormat = toml::from_str(&contents).unwrap();

    config.files = Vec::new();

    let walker = WalkDir::new(&dir).follow_links(true).into_iter();

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let _path = entry.unwrap();
        let path = _path.path();

        if path.is_dir() {
            continue;
        } else {
            let mut sha1hasher: Sha1 = Sha1::new();
            let mut sha512hasher: Sha512 = Sha512::new();

            let bytes = std::fs::read(path).unwrap();

            sha1hasher.update(&bytes);
            sha512hasher.update(&bytes);

            let sha1_ = sha1hasher.finalize();
            let sha1 = HEXLOWER.encode(sha1_.as_slice());

            let sha512_ = sha512hasher.finalize();
            let sha512 = HEXLOWER.encode(sha512_.as_slice());

            let size = bytes.len();

            config.add_file(InternalModpackFileFormat {
                path: path.to_str().unwrap().replace(&format!("{}/", dir2), ""),
                sha1: sha1.to_string(),
                sha512: sha512.to_string(),
                url: "".to_string(),
                size: size,
            });
        }
    }

    let walker2 = WalkDir::new(&dir).follow_links(true).into_iter();

    for entry in walker2.filter_entry(|e| !is_not_mod(e)) {
        let _path = entry.unwrap();
        let path = _path.path();

        if path.is_dir() {
            continue;
        } else {
            let mut sha1hasher: Sha1 = Sha1::new();
            let mut sha512hasher: Sha512 = Sha512::new();

            let bytes = std::fs::read(path).unwrap();

            sha1hasher.update(&bytes);
            sha512hasher.update(&bytes);

            let sha1_ = sha1hasher.finalize();
            let sha1 = HEXLOWER.encode(sha1_.as_slice());

            let sha512_ = sha512hasher.finalize();
            let sha512 = HEXLOWER.encode(sha512_.as_slice());

            let size = bytes.len();

            config.add_mod(InternalModpackFileFormat {
                path: path.to_str().unwrap().replace(&format!("{}/", dir2), ""),
                sha1: sha1.to_string(),
                sha512: sha512.to_string(),
                url: "".to_string(),
                size: size,
            });
        }
    }

    let mut config_file = std::fs::File::create(format!("{}/modpack.toml", dir2)).unwrap();
    let config_string = toml::to_string(&config).unwrap();
    config_file.write_all(config_string.as_bytes()).unwrap();
}
