use std::path::Path;
use std::fs;
use rand::seq::SliceRandom;
use rand;

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
        return entries.count() == 0
    }
    false
}


fn main() {
    let wallpapers_path = if cfg!(windows) {
        format!("{}\\.wallpapers", std::env::var("HOMEPATH").unwrap())
    } else {
        format!("{}/.wallpapers", std::env::var("HOME").unwrap())
    };


    // TODO change this to own mirror repository
    let _url = "https://github.com/cat-milk/Anime-Girls-Holding-Programming-Books";

    

    if !Path::new(&wallpapers_path.clone()).exists() {
            match fs::create_dir(wallpapers_path.clone()) {
                Ok(()) => {
                    println!("Folder created.");
                },
                Err(e) => println!("Error: {}", e),
            }

    } 
    

    // TODO: download all the images from the mirror.
    if is_folder_empty(wallpapers_path.clone().as_str()) {
        println!("Folder is empty");   
    }

    let paths = match fs::read_dir(wallpapers_path) {
        Ok(paths) => paths
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>(),
        Err(_) => {
            eprintln!("Failed to read directory.");
            return;
        }
    };

    let mut rng = rand::thread_rng(); 
    let path = paths.choose(&mut rng).unwrap();

    match wallpaper::set_from_path(&path.display().to_string()) {
        Ok(()) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
    
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
}