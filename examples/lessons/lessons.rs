use std::io::{self, Write};
use std::process::Command;
use std::path::Path;

fn main() {
    let folder = "print";

    println!("Scegli la lezione (1-10):");
    for i in 1..=10 {
        print!("{}) Lezione {}   ", i, i);
        if i == 5 {
            println!();
        }
    }
    println!();
    
    let mut lesson_choice = String::new();
    print!("> ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut lesson_choice).unwrap();
    
    let lesson_num: u32 = match lesson_choice.trim().parse() {
        Ok(num) if num >= 1 && num <= 10 => num,
        _ => return,
    };

    let relative_path = format!("examples/lessons/{}/{}.ns", folder, lesson_num);

    if !Path::new(&relative_path).exists() {
        return;
    }

    let _status = Command::new("cargo")
        .args(&["run", "--quiet", "--", "build", &relative_path])
        .status();
}
