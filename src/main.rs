use std::{env, process};

use colored::Colorize;

use config::{Config, run};


fn main() {

  let config = Config::build(env::args()).unwrap_or_else(|err| {
    eprintln!("Problem parsing arguments: {}", err.red());
    process::exit(1);
  });

  if let Err(e) = run(config) {
    eprintln!("Application error: {}", e.to_string().red());

    process::exit(1);
  }
}



