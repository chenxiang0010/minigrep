use std::process;

use colored::Colorize;

use config::{Config, run};

fn main() {

  let config = Config::build();

  if let Err(e) = run(config) {
    eprintln!("Application error: {}", e.to_string().red());
    process::exit(1);
  }
}



