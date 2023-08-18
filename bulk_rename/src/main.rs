use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    println!("arguments: {:?}", args);

    let relative_path = &args[1];
    let current_dir = env::current_dir()?;
    println!("current directory: {:?}", current_dir);
    let directory_path = current_dir.join(relative_path);
    let entries = fs::read_dir(directory_path)?;
    for (i, entry_result) in entries.enumerate() {
        let entry = entry_result?;
        let file_path = entry.path();
        println!("{:?}", file_path);
        let parent_dir = file_path
            .parent()
            .expect("Parent directory of file not found!");
        let new_file_name = format!("renamed_{}.png", i);
        fs::rename(&file_path, parent_dir.join(new_file_name))?;
    }

    Ok(())
}
