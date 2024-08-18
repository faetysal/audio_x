use std::{fs::File, io::BufReader, time::Duration};
use rodio::{Decoder, OutputStream, Sink, Source};
use crate::tui;

use super::{player_tui, Track};

pub fn init(tracks: Vec<Track>) {
  let (_stream, stream_handle) = OutputStream::try_default()
    .unwrap();
  let sink = Sink::try_new(&stream_handle).unwrap();
  for track in tracks {
    let file = File::open(track.path).expect("File not found");
    let source = Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
  }
  
  sink.sleep_until_end();
}

fn a_to_b(source: Decoder<BufReader<File>>) {
  // start: 1:40, stop: 2:12
  source.skip_duration(Duration::from_secs(100))
    .take_duration(Duration::from_secs(32))
    .repeat_infinite();
}