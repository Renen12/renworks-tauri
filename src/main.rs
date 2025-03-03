use std::{env, fs::{self, read_dir}, process::{exit, Command}};
#[allow(unused_assignments)]
fn main()
{
    let output = String::from_utf8(Command::new("sh").args(["-c", "command -v cargo"]).output().unwrap().stdout).unwrap();
    let exists: bool;
    if output == ""{
        exists = false;
    } else {
        exists = true;
    }
    if !exists {
        eprintln!("Please install create-tauri-app via cargo.");
        exit(1);
    }
    // Analyse diff
    let previous: Vec<_> = fs::read_dir(".").unwrap().collect();
    let mut unwrapped: Vec<String> = Vec::new();
    for direntry in previous {
        unwrapped.push(direntry.unwrap().file_name().into_string().unwrap());
    }
    Command::new("cargo").args(["create-tauri-app", "-m", "npm", "-t", "vanilla"]).status().unwrap();
    let after: Vec<_> = read_dir(".").unwrap().collect();
    let mut fixed: Vec<String> = Vec::new();
    for direntry in after {
        fixed.push(direntry.unwrap().file_name().into_string().unwrap());
    }
    let mut desired_name: &String = &String::new();
    let mut index = 0;
    while index < fixed.len() {
        let current = &fixed[index];
        if !unwrapped.contains(current) {
            desired_name = current;
            break;
        }
        index += 1;
    }
    println!("{desired_name}");
}