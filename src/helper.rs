use std::fs::{self, File};
use std::io::{Result, Write};
use std::path::{Path, PathBuf};
use std::process::exit;

use path_slash::PathExt;
use regex::Regex;

use crate::config;

pub fn generate_list_of_vids(file_format: &str, paths: &[PathBuf]) -> String {
    let mut list = String::new();
    let re = Regex::new(format!(r"\.{}$", regex::escape(file_format)).as_str()).unwrap();

    let (_, _, _, scale) = config::get();

    for path in paths {
        if re.is_match(&format!("{}", path.display())) {
            if scale.is_none() {
                if list.chars().count() == 0 {
                    list = format!("file '{}'", path.file_name().unwrap().to_str().unwrap());
                } else {
                    list = format!(
                        "{}\nfile '{}'",
                        list,
                        path.file_name().unwrap().to_str().unwrap()
                    );
                }
            } else if scale.is_some() {
                if list.chars().count() == 0 {
                    list = format!(
                        "file 'scaled_vids/{}'",
                        path.file_name().unwrap().to_str().unwrap()
                    );
                } else {
                    list = format!(
                        "{}\nfile 'scaled_vids/{}'",
                        list,
                        path.file_name().unwrap().to_str().unwrap()
                    );
                }
            }
        }
    }
    list
}

pub fn get_sorted_paths(input_vids_path: &Path) -> Result<Vec<PathBuf>> {
    let mut paths = fs::read_dir(input_vids_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;
    paths.sort();
    Ok(paths)
}

pub fn split(string: String) -> Vec<String> {
    let file_formats: Vec<_> = string
        .lines()
        .map(|s| s.trim().split(',').map(String::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    file_formats[0].clone()
}

pub fn remove_file(path: &Path) -> Result<()> {
    if Path::new(path).exists() {
        println!("🔥 Removing {}", path.display());
        fs::remove_file(path)?;
    }
    Ok(())
}

pub fn write(path: &Path, string: String) {
    let mut file = File::create(path.to_slash().unwrap()).unwrap();
    file.write_all(string.as_bytes()).unwrap();
}

pub fn exit_when_ffmpeg_not_available() {
    if !is_ffmpeg_available() {
        exit(1);
    }
}

fn is_ffmpeg_available() -> bool {
    if cfg!(target_os = "windows") {
        if which::which("ffmpeg.exe").is_err() {
            eprintln!("ffmpeg.exe not found 😬");
            false
        } else {
            true
        }
    } else if which::which("ffmpeg").is_err() {
        eprintln!("ffmpeg not found 😬");
        false
    } else {
        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_ffmpeg_available() {
        assert_eq!(is_ffmpeg_available(), true);
    }
}
