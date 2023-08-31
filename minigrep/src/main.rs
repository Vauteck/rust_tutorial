use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = parse_config(&args);
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error : {e}");
        process::exit(1);
    }

    /*
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file!");

    println!("File content:\n{contents}");
    */
}




