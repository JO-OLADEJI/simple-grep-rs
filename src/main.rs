mod structs;

use std::{env, fs, process};
use structs::config::Config;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    match fs::read_to_string(config.file_path) {
        Ok(utf8_content) => {
            let search_result = utf8_content.lines()
                .enumerate()
                .filter(|(_, line)| {
                    match config.ignore_case {
                        true => line.to_lowercase().contains(&config.query.to_lowercase()),
                        false => line.contains(&config.query)
                    }
                });
            let fmt_search_result = search_result.map(|(i, line)| "#".to_owned() + &(i+1).to_string() + " - " + line);

            println!("{}", fmt_search_result.collect::<Vec<String>>().join("\n"));
        },
        Err(error) => {
            eprintln!("{:?}", error);
            process::exit(1);
        }
    }
}
