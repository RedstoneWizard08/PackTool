pub mod cli;
pub mod colors;
pub mod commands;
pub mod logging;
pub mod modpack;

use crate::cli::parse;

fn main() {
    let raw: Vec<String> = std::env::args().collect();
    let joined = raw.join(" ");
    let options = parse::parse_arguments(joined.as_str());
    let args = parse::get_arguments(joined.as_str());

    if args.len() > 0 {
        if args.get(0).unwrap().to_string() == "init".to_string() {
            commands::init::init(args, options);
        } else if args.get(0).unwrap().to_string() == "build".to_string() {
            commands::build::build(args, options);
        } else if args.get(0).unwrap().to_string() == "refresh".to_string() {
            commands::refresh::refresh(args, options);
        } else {
            commands::help::help(args, options);
        }
    } else {
        commands::help::help(args, options);
    }
}
