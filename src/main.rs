use rand;
use rand::seq::SliceRandom;
use std::fs;
use std::path::Path;

use git2::Repository;

use wallpaper;

/// Checks if the specified folder is empty.
///
/// ## Arguments
///
/// * `folder_path` - A string slice that holds the path of the folder to check.
///
/// ## Returns
///
/// * `bool` - Returns true if the folder is empty, otherwise false.
fn is_folder_empty(folder_path: &str) -> bool {
    if let Ok(entries) = fs::read_dir(folder_path) {
        return entries.count() == 0;
    }
    false
}

fn main() {
    let wallpapers_path = if cfg!(windows) {
        format!("{}\\.wallpapers", std::env::var("HOMEPATH").unwrap())
    } else {
        format!("{}/.wallpapers", std::env::var("HOME").unwrap())
    };

    let url = "https://github.com/akumarujon/wall-rs-mirror";

    if !Path::new(&wallpapers_path.clone()).exists() {
        println!("Folder not found.");
        println!("Clonning the repo.");
        match Repository::clone(url, wallpapers_path.clone()) {
            Ok(_repo) => {
                println!("Clonned the repo.");
            }
            Err(e) => println!("Error: {}", e),
        }
    }

    if is_folder_empty(wallpapers_path.clone().as_str()) {
        std::fs::remove_dir_all(wallpapers_path.clone()).unwrap();
        println!("No content was found. Clonning the repo.");
        match Repository::clone(url, wallpapers_path.clone()) {
            Ok(_repo) => {
                println!("Clonned the repo.")
            }
            Err(e) => println!("Error: {}", e),
        }
        
    }

    let paths = match fs::read_dir(wallpapers_path) {
        Ok(paths) => paths.map(|entry| entry.unwrap().path()).collect::<Vec<_>>(),
        Err(_) => {
            eprintln!("Failed to read directory.");
            return;
        }
    };

    let mut rng = rand::thread_rng();
    let path = paths.choose(&mut rng).unwrap();

    println!("{}", path.display());

    if !path.exists() {
        println!("{} does not exist.", path.display());
        std::process::exit(1);
    }

    match wallpaper::set_from_path(&path.display().to_string()) {
        Ok(()) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }

    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
}
