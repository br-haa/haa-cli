use dialoguer::{theme::ColorfulTheme, MultiSelect};
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let multi = &["car", "bike", "plane", "roller blades"];
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("how move?")
        .items(&multi[..])
        .interact()
        .unwrap();

    if selections.is_empty() {
        println!("hello?")
    } else {
        println!("You selected these things:");
        for selection in selections {
            let m = multi[selection];
            println!(" {}", m);

            if m == "bike" {
                Command::new("ls").spawn().expect("idk");
            } else if m == "plane" {
                let mut f = File::create("text.txt")?;
                f.write_all(b"plane")?;
                f.sync_all()?;
            } else if m == "roller blades" {
                Command::new("git").arg("status").spawn().expect("xd");
            } else if m == "car" {
                let mut f = File::create("text.txt")?;
                f.write_all(b"car")?;
                f.sync_all()?;
            }
        }
    }

    Ok(())
}
