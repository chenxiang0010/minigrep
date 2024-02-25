use std::{env, fs};
use std::error::Error;

pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl Config {
  pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
    // 第一个参数是程序名
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string")
    };

    let file_path = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file path")
    };

    let ignore_case_flag = env::var("IGNORE_CASE").ok();
    let ignore_case = match ignore_case_flag.as_ref().map(String::as_ref) {
      None => false,
      Some("0") => false,
      Some("1") => true,
      _ => false
    };

    Ok(Config { query, file_path, ignore_case })
  }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_path)?;

  for line in search(&config.query, &contents, config.ignore_case) {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
  contents.lines()
    .filter(|line| match ignore_case {
      true => line.to_uppercase().contains(&query.to_uppercase()),
      false => line.contains(&query)
    })
    .collect()
}

