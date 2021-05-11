extern crate glob;
use self::glob::glob;
use std::{ env, fs, path::Path, fs::File, io::prelude::*, process::Command, process::Stdio };

struct Song<'a> {
    video: &'a Path,
    music: &'a Path,
    tag: &'a String
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
        "convert" => {
            let videos = get_videos(args[1].to_string());
            convert_mp4s(videos, args[1].to_string());
        },
        _=> println!("Unknown argument {}", args[0])
    }
}

fn convert_mp4s(files: Vec<String>, tag: String) {
    for file in files {
        println!("Beginning to convert: {}", file);
        let local_path: Vec<&str> = file.split("\\").skip(4).collect();
        let joined = local_path.join("\\");
        let mut music_path = String::from(get_home());
        music_path.push_str("\\Music\\");
        music_path.push_str(&joined);
        music_path = music_path.replace(".mp4", ".mp3");
        // let song = make_song(&file, &music_path, &tag);
        let song = Song {
            video: Path::new(&file),
            music: Path::new(&music_path),
            tag: &tag
        };

        match song.music.exists() {
            true => Ok(fs::remove_file(song.music)),
            false => Err(false)
        };

        let process = match Command::new("ffmpeg")
            .args(&["-y", "-i", &song.video.display().to_string(), &song.music.display().to_string()])
            .stdout(Stdio::piped())
            .spawn() {
                Err(why) => panic!("couldn't spawn ffmpeg: {}", why),
                Ok(process) => process,
        };

        let mut s = String::new();
        match process.stdout.unwrap().read_to_string(&mut s) {
            Err(why) => panic!("Couldn't read ffmpeg stdout: {}", why),
            Ok(_) => print!("{}", s), 
        }
        // let _output = Command::new("cmd")
        //     .output()
        //     .expect("Failed to starts CMD");
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
        Ok(_file) => println!("successfully wrote to {}", display),
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

fn get_home() -> String {
    match home::home_dir() {
        Some(path) => return path.display().to_string(),
        None => return String::new(),
    }
}
