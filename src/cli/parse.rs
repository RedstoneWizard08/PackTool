use std::{
    collections::{HashMap, HashSet},
    env::Args,
};

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

pub fn join_args(args: Args) -> String {
    let mut joined = String::new();
    for arg in args {
        joined.push_str(arg.as_str());
        joined.push(' ');
    }
    joined.remove_last();
    joined
}

pub fn parse_arguments(raw_args: &str) -> HashMap<String, &str> {
    let args: HashSet<&str> = raw_args.split(" ").collect();
    let mut options: HashMap<String, &str> = HashMap::new();

    let mut i = 0;

    while i < args.len() {
        let arg = args.iter().nth(i).unwrap();

        if arg.starts_with("--") {
            if arg.contains("=") {
                let mut split = arg.split("=");
                let key = split.next().unwrap();
                let value = split.next().unwrap();
                let rkey = key.clone().replace("--", "");
                options.insert(rkey, value);
            } else {
                if args.iter().len() < i + 1 {
                    panic!("Missing value for option: {}", arg);
                }

                let key = arg;
                let value = args.iter().nth(i + 1).unwrap();
                let rkey = key.clone().replace("--", "");
                options.insert(rkey, value);
                i += 1;
            }
        } else if arg.starts_with("-") {
            if arg.contains("=") {
                let mut split = arg.split("=");
                let key = split.next().unwrap();
                let value = split.next().unwrap();
                let rkey = key.clone().replace("-", "");
                options.insert(rkey, value);
            } else {
                if args.iter().len() < i + 1 {
                    panic!("Missing value for option: {}", arg);
                }

                let key = arg;
                let value = args.iter().nth(i + 1).unwrap();
                let rkey = key.clone().replace("-", "");
                options.insert(rkey, value);
                i += 1;
            }
        }

        i += 1;
    }

    return options;
}

pub fn get_arguments(joined: &str) -> Vec<&str> {
    let mut args: Vec<&str> = Vec::new();
    let mut split = joined.split(" ");
    split.next();

    for arg in split {
        if !arg.starts_with("-") {
            args.push(arg);
        }
    }

    return args;
}
