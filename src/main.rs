use dialoguer::{theme::ColorfulTheme, MultiSelect};
use std::process::Command;
use std::fs::File;

fn main () -> std::io::Result<()>{
    let multi = &[
        "car",
        "bike",
        "plane",
        "roller blades"
    ];
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("how move?")
        .items(&multi[..])
        .interact()
        .unwrap();

        if selections.is_empty() {
            println!("hello?")
        } else  {
            println!("You selected these things:");
            for selection in selections {
                let m = multi[selection];
                println!(" {}", m);

                if m == "bike" {
                Command::new("ls")
                    .spawn()
                    .expect("idk");
                } else if m == "plane" {
                    File::create("text.txt");
                }
            }
        }
            
Ok(())
}