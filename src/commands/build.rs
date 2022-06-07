use std::{
    collections::HashMap,
    io::{Read, Write},
};

use crate::{
    logging::Logger,
    modpack::{
        file::{ModpackFile, ModpackFileHashes},
        pack::{InternalModpackFormat, Modpack},
    },
};

pub fn build(args: Vec<&str>, _options: HashMap<String, &str>) {
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

    logger.info(format!("Building modpack in {} for Modrinth...", dir).as_str());

    let mut config_file = std::fs::File::open(format!("{}/modpack.toml", dir)).unwrap();

    let mut buf = "".to_string();
    config_file.read_to_string(&mut buf).unwrap();

    let contents = buf.as_str();
    let config: InternalModpackFormat = toml::from_str(contents).unwrap();

    let mut modpack = Modpack::new();
    modpack.set_name(config.name);
    modpack.set_version_id(config.version);
    modpack.set_summary(config.summary);

    for file in config.files {
        let mut pack_file = ModpackFile::new(file.path, file.size);
        pack_file.set_hashes(ModpackFileHashes::new(file.sha1, file.sha512));

        if file.url != "" {
            pack_file.set_download_url(file.url);
        }

        modpack.add_file(pack_file);
    }

    let mut manifest = std::fs::File::create(format!("{}/modrinth.index.json", dir)).unwrap();
    let manifest_string = serde_json::to_string(&modpack).unwrap();
    manifest.write_all(manifest_string.as_bytes()).unwrap();
}
