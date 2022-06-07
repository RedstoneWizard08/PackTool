use std::{collections::HashMap, io::Write};

use crate::{logging::Logger, modpack::pack::InternalModpackFormat};

pub fn init(args: Vec<&str>, options: HashMap<String, &str>) {
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

    let mut name = dir.split("/").last().unwrap().to_string();

    if options.contains_key("name") {
        name = options.get("name").unwrap().to_string();
    } else if options.contains_key("n") {
        name = options.get("n").unwrap().to_string();
    }

    logger.info(format!("Creating a new project in {}...", dir).as_str());

    let mut config_file = std::fs::File::create(format!("{}/modpack.toml", dir)).unwrap();

    let config = InternalModpackFormat {
        name: name,
        version: "0.0.0".to_string(),
        summary: "A modpack created with PackTool.".to_string(),
        files: Vec::new(),
        mods: Vec::new(),
    };

    let config_string = toml::to_string(&config).unwrap();
    config_file.write_all(config_string.as_bytes()).unwrap();
}
