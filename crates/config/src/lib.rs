use std::error::Error;
use std::fs;

use clap::Parser;

/// MiniGrep
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// 搜索的字符串
  #[arg(short, long)]
  query: String,

  /// 文件路径
  #[arg(short, long)]
  file_path: String,

  /// 是否忽略大小写
  #[arg(short, long, default_value_t = false)]
  ignore_case: bool,
}


pub struct Config {
  pub query: String,
  pub file_path: String,
  pub ignore_case: bool,
}

impl Config {
  pub fn build() -> Config {
    let args = Args::parse();

    let query = args.query;
    let file_path = args.file_path;
    let ignore_case = args.ignore_case;

    Config { query, file_path, ignore_case }
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

