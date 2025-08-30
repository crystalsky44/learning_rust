use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let configure_value = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argumetns: {err}");
        process::exit(1);
    });
    
    /*
    let args: Vec<String> = env::args().collect();

    // let (query, file_path) = parse_config(&args);
    let configure_value = Config::build(&args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {err}");
            process::exit(1);
        });
    */

    println!("Searching for {}", configure_value.query);
    println!("In file {}", configure_value.file_path);

    if let Err(e) = minigrep::run(configure_value) {
        println!("Applicadtion error: {e}");
        process::exit(1);
    }
}
/*
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
           return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
*/
