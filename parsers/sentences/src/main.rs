use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let contents = fs::read_to_string(config.file_path)
        .expect("File was either not found or not a valid path. Please Double Check.");
}

struct Config {
    separator: char,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() != 3 {
            panic!("WRONG NUMBER OF ARGUMENTS. SHOULD BE EXACTLY TWO(2).");
        }

        let separator = args[1]
            .clone()
            .parse()
            .expect("SECOND ARGUMENT WAS NOT ONE(1) CHARACTER");
        let file_path = args[2].clone();

        Config {
            separator,
            file_path,
        }
    }
}
