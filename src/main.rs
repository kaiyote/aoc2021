#![feature(path_try_exists)]
use config::{Config, File};
use reqwest::{header, Client};
use std::fs;
use std::io::Write;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
  day: i32,
  #[structopt(short = "d")]
  download_input: bool,
}

#[tokio::main]
async fn main() {
  let Args {
    day,
    download_input,
  } = Args::from_args();

  let code_file = format!("src/day{}.rs", day);
  let input_file = format!("inputs/day{}.txt", day);

  write_test_harness(day, &code_file);
  write_input_file(day, download_input, &input_file).await;
}

fn write_test_harness(day: i32, file: &str) {
  match fs::try_exists(file) {
    Ok(true) => {
      println!("Code File for Day {} already exists, skipping!!!", day)
    }
    _ => {
      fs::write(file, code_file_content(day)).expect("couldn't write code file");

      let mut file = fs::OpenOptions::new()
        .append(true)
        .open("src/lib.rs")
        .unwrap();

      if let Err(e) = writeln!(file, "pub mod day{};", day) {
        eprintln!("Couldn't write to file: {}", e);
      }
    }
  }
}

async fn write_input_file(day: i32, download_input: bool, file: &str) {
  match fs::try_exists(file) {
    Ok(true) => {
      println!("Input File for Day {} already exists, skipping!!!", day)
    }
    _ => {
      let mut content: String = String::from("");

      if download_input {
        let cookie = Config::new()
          .merge(File::with_name("download_auth"))
          .unwrap()
          .get_str("cookie")
          .unwrap();

        if cookie.len() == 0 {
          println!("NO AUTH TOKEN. BAILING!!!");
          return;
        }

        let client = Client::new();
        let resp = client
          .get(format!("https://adventofcode.com/2021/day/{}/input", day))
          .header(header::COOKIE, cookie)
          .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.45 Safari/537.36")
          .header(header::ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
          .send()
          .await
          .unwrap()
          .text()
          .await
          .unwrap();

        content = resp;
      }

      fs::write(file, content).expect("couldn't write input file")
    }
  }
}

fn code_file_content(day: i32) -> String {
  format!(
    "fn day{0}_part1(_input: &str) -> i32 {{
  0
}}

fn day{0}_part2(_input: &str) -> i32 {{
  0
}}

#[cfg(test)]
mod tests {{
  use super::*;
  use std::fs;

  #[test]
  fn day{0}_part1_smoke() {{ }}

  #[test]
  fn day{0}_part1_test() {{
    let contents = fs::read_to_string(\"inputs/day{0}.txt\").expect(\"couldn't read file\");

    let result = day{0}_part1(&contents);
    assert_eq!(result, 0);
  }}

  #[test]
  fn day{0}_part2_smoke() {{ }}

  #[test]
  fn day{0}_part2_test() {{
    let contents = fs::read_to_string(\"inputs/day{0}.txt\").expect(\"couldn't read file\");

    let result = day{0}_part2(&contents);
    assert_eq!(result, 0);
  }}
}}",
    day
  )
}
