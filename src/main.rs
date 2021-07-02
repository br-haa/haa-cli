use structopt::StructOpt;
use std::io::BufReader;
use std::fs::File;
#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf
}
fn main() {
let args = Cli::from_args();

let f = File::open(&args.path);
let content = BufReader::new(f)
    .expect("could not read file");

for line in content.lines() {
    if line.contains(&args.pattern) {
        println!("{}", line)
    }
}
}
