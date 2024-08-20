use std::{fs::File, io::BufReader, time::Duration};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use crate::tui;

use super::{player_tui, Track};

pub struct Player {
  _stream: OutputStream,
  stream_handle: OutputStreamHandle,
  sink: Sink,
  queue: Vec<Decoder<BufReader<File>>>
}

impl<'a> Player {
  pub fn init() -> Self {
    let (_stream, stream_handle) = OutputStream::try_default()
      .unwrap();
    let (sink, _) = Sink::new_idle();
    let mut player = Player {
      _stream,
      stream_handle,
      sink,
      queue: vec![]
    };

    player.sink = Sink::try_new(&player.stream_handle).unwrap();

    player
  }
  
  pub fn set_queue(&'a mut self, playlist: &'a Vec<Track>, start_idx: usize) {
    self.sink.clear();

    let playlist_count = playlist.len();
    for i in 0..playlist_count {
      let idx = (playlist_count + i + start_idx) % playlist_count;
      let track = playlist.get(idx).unwrap();
      let file = File::open(track.path.clone()).expect("File not found");
      let source = Decoder::new(BufReader::new(file)).unwrap();
      self.sink.append(source);
    }
  }

  pub fn play(&self) {
    self.sink.play();
  }

  fn a_to_b(source: Decoder<BufReader<File>>) {
    // start: 1:40, stop: 2:12
    source.skip_duration(Duration::from_secs(100))
      .take_duration(Duration::from_secs(32))
      .repeat_infinite();
  }
}