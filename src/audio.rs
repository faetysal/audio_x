use std::{
  fs, 
  io::{self, BufReader},
  path::PathBuf, time::Duration,
  error::Error,
  path::Path
};

use lofty::prelude::*;
use lofty::probe::Probe;

pub mod player_tui;
pub mod player;
mod custom_source;

#[derive(Debug, Clone)]
pub struct Track {
  title: String,
  pub artist: Option<String>,
  pub album: Option<String>,
  path: PathBuf,
  duration: Duration
}

impl Track {
  fn from_path(path: &PathBuf) -> Self {
    let file = Probe::open(path)
      .expect("Invalid path")
      .read()
      .expect("Failed to read file");

    let tag = match file.primary_tag() {
      Some(tag) => tag,
      None => file.first_tag().expect("No tags found")
    };

    Self { 
      title: Self::get_title(tag.title().as_deref(), path),
      artist: tag.artist().map(String::from), 
      album: tag.album().map(String::from), 
      path: path.clone(),
      duration: file.properties().duration()
    }
  }

  pub fn get_title(title_opt: Option<&str>, path: &PathBuf) -> String {
    title_opt
      .map(String::from)
      .unwrap_or_else(|| {
        let path_str = path.to_str().unwrap();
        let paths: Vec<&str> = path_str.split("/").collect();
        let title = paths.last().unwrap();

        title.to_string()
      })
  }
}

fn get_file_paths(dir: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
  let mut paths = Vec::new();
  if dir.is_dir() {
    for entry in fs::read_dir(dir)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_dir() {
        let mut sub_paths = get_file_paths(&path)?;
        paths.append(&mut sub_paths);
      } else {
        paths.push(path);
      }
    }
  }

  Ok(paths)
}

fn get_audio_paths(paths: Vec<PathBuf>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
  let entries = paths
    .iter()
    .filter_map(|path| {
      const EXTS: [&str; 4] = ["mp3", "aac", "m4a", "wav"];
      if path.extension().map_or(false, |ext| EXTS.contains(&ext.to_str().unwrap())) {
        return Some(path.to_owned());
      }

      None
    })
    .collect::<Vec<_>>();

  Ok(entries)
}

pub fn get_tracks(dir: &Path) -> Result<Vec<Track>, Box<dyn Error>> {
  let audio_paths = get_audio_paths(get_file_paths(dir)?)?;
  let tracks = audio_paths
    .iter()
    .map(|path| {
      Track::from_path(path)
    })
    .collect();
    
  Ok(tracks)
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn audio_path() {
    let paths = get_audio_paths(get_file_paths(Path::new("data")).unwrap()).unwrap();
    assert_eq!(6, paths.len());
  }
}