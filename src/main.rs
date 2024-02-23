use std::path::Path;
use std::fs;
use rand::seq::SliceRandom;
use rand;

use wallpaper;

fn is_folder_empty(folder_path: &str) -> bool {
    if let Ok(entries) = fs::read_dir(folder_path) {
        return entries.count() == 0
    }
    false
}


fn main() {
    let _path: String = "/home/t34/dev/projects/wall-rs/pics/Kaori_Miyazono_Rust.png".to_string();
    let wallpapers_path = "/home/t34/dev/projects/wall-rs/pics/";

    let _url = "https://github.com/cat-milk/Anime-Girls-Holding-Programming-Books";

    

    if !Path::new(&wallpapers_path).exists() {
            match fs::create_dir(wallpapers_path) {
                Ok(()) => {
                    println!("Folder created.");
                },
                Err(e) => println!("Error: {}", e),
            }

    } 
    
    if is_folder_empty(wallpapers_path) {
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

    println!("{}", path.display());

    if !Path::new(&path).exists() {
        println!("File not found");
    }

    match wallpaper::set_from_path(&path.display().to_string()) {
        Ok(()) => println!("Success"),
        Err(e) => println!("Error: {}", e),
    }
    
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
}