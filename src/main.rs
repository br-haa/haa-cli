use structopt::StructOpt;
use std::io::{stdin};
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf
}
fn input() {
  let mut input = String::new();
  stdin().read_line(&mut input).expect("error: unable to read user input");
  println!("You typed: {}",input);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
input();
let args = Cli::from_args();

let result = std::fs::read_to_string(&args.path);
let content = match result {
    Ok(content) => {content},
    Err(error) => {panic!("error time!! {}", error)}
};
for line in content.lines() {
    if line.contains(&args.pattern) {
        println!("{}", line);
    }
}
Ok(())
}
