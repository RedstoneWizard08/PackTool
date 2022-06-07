use std::collections::HashMap;

use crate::logging::Logger;

pub fn help(_args: Vec<&str>, _options: HashMap<String, &str>) {
    let logger = Logger::new();

    logger.info("Help menu is not yet implemented.");
}
