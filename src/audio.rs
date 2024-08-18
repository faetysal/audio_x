use std::{
  fs, 
  io::{self, BufReader},
  path::PathBuf, time::Duration,
  error::Error,
  path::Path
};

use id3::{Tag, TagLike};

pub mod player;

#[derive(Debug)]
pub struct Track {
  title: String,
  artist: String,
  album: String,
  path: PathBuf
}

impl Track {
  fn from_tag(tag: Tag) -> Self {
    Self { 
      title: String::from(tag.title().unwrap_or("-")), 
      artist: String::from(tag.artist().unwrap_or("-")), 
      album: String::from(tag.album().unwrap_or("-")), 
      path: PathBuf::new() 
    }
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
      let tag = Tag::read_from_path(path).unwrap();
      let mut track = Track::from_tag(tag);
      track.path = path.clone();
      track
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