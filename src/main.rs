extern crate glob;
use self::glob::glob;
use std::{ env, fs, path::Path };
use std::fs::File;
use std::io::prelude::*;

struct Song {
    video: String,
    music: String,
    tag: String
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    
    let mut music_dir = String::from(get_home());
    music_dir.push_str("\\Music\\");
    let action = args[0].to_string();
    println!("Arguments: {:?}", args);
    match action.as_str() {
        "playlist" => {
            let videos = get_videos(args[1].to_string());
            write_playlist(videos, args[1].to_string());
        },
        _=> println!("Unknown argument {}", args[0])
    }
}

fn write_playlist(files: Vec<String>, tag: String) {
    let mut playlist = String::from(get_home());
    playlist.push_str("\\Documents\\playlists\\");
    playlist.push_str(&tag);
    playlist.push_str(".txt");
    let playlist_path = Path::new(&playlist);
    let display = playlist_path.display();
    match playlist_path.exists() {
        true => Ok(fs::remove_file(playlist_path)),
        false => Err(false),
    };

    let mut file = match File::create(&playlist_path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file
    };

    let file_contents = files.join("\n");
    println!("contents: {}", file_contents);

    match file.write_all(file_contents.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(file) => println!("successfully wrote to {}", display),
    }
}

fn get_videos(dir: String) -> Vec<String>{
    let mut video_dir = String::from(get_home());
    video_dir.push_str("\\Videos\\");
    video_dir.push_str(&dir);
    video_dir.push_str("\\*.mp4");

    return get_files(video_dir);
}

fn get_music(dir: String) -> Vec<String> {
    let mut music_dir = String::from(get_home());
    music_dir.push_str("\\Music\\");
    music_dir.push_str(&dir);
    music_dir.push_str("\\*.mp3");

    return get_files(music_dir);
}

fn get_files(dir: String) -> Vec<String> {
    let mut results = vec![];

    for entry in glob(&dir).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => results.push(path.display().to_string()),
            Err(e) => println!("Error: {:?}", e)
        }
    }
    
    return results;
}

fn make_song(video: String, music: String, tag: String) -> Song {
    Song {
        video: video,
        music: music,
        tag: tag
    }
}

fn get_home() -> String {
    match home::home_dir() {
        Some(path) => return path.display().to_string(),
        None => return String::new(),
    }
}
