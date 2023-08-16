use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("arguments: {:?}", args);

    let relative_path = &args[1];

    if let Ok(current_dir) = env::current_dir() {
        println!("current directory: {:?}", current_dir);

        let directory_path = current_dir.join(relative_path);

        if let Ok(entries) = fs::read_dir(directory_path) {
            for entry in entries {
                println!("{:?}", entry);
            }
        }
    } else {
        println!("error retrieving current directory");
    }
}
