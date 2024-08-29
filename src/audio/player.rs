use std::{fs::File, io::BufReader, time::Duration};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::sync::{Arc, Mutex};

use super::{custom_source::{self, CustomSource}, player_tui, Track};

pub struct Player {
  _stream: OutputStream,
  stream_handle: OutputStreamHandle,
  sink: Sink,
  queue: Vec<Track>,
  queue_idx: Arc<Mutex<usize>>,
  now_playing: Option<Track>
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
      queue: vec![],
      queue_idx: Arc::new(Mutex::new(0)),
      now_playing: None
    };

    player.sink = Sink::try_new(&player.stream_handle).unwrap();

    player
  }
  
  pub fn set_queue(&'a mut self, playlist: &'a Vec<Track>, start_idx: usize) {
    self.sink.clear();
    self.queue.clear();
    *self.queue_idx.lock().unwrap() = 0;

    let playlist_count = playlist.len();
    for i in 0..playlist_count {
      let idx = (playlist_count + i + start_idx) % playlist_count;
      let track = playlist.get(idx).unwrap();
      let file = File::open(track.path.clone()).expect("File not found");
      let source = Decoder::new(BufReader::new(file)).unwrap();
      let q_idx = Arc::clone(&self.queue_idx);
      let custom_source = CustomSource::wrap(source, move || {
        let mut idx = q_idx.lock().unwrap();
        *idx += 1;
      });
      self.sink.append(custom_source);
      let track_cloned: Track = track.clone();
      self.queue.push(track_cloned);
    }
  }
  
  fn reset_queue(&self) {
    self.sink.clear();
    let start = self.queue_idx.lock().unwrap();
    for i in *start..self.queue.len() {
      let track = self.queue.get(i).unwrap();
      let file = File::open(track.path.clone()).expect("File not found");
      let source = Decoder::new(BufReader::new(file)).unwrap();
      let q_idx = Arc::clone(&self.queue_idx);
      let custom_source = CustomSource::wrap(source, move || {
        let mut idx = q_idx.lock().unwrap();
        *idx += 1;
      });
      self.sink.append(custom_source);
    }
  }

  pub fn play(&mut self) {
    self.set_now_playing();
    self.sink.play();
  }

  pub fn toggle_play(&self) {
    if self.sink.is_paused() {
      self.sink.play();
    } else {
      self.sink.pause();
    }
  }

  pub fn next(&self) {
    *self.queue_idx.lock().unwrap() += 1;
    // println!("qidx(+): {}", *self.queue_idx.lock().unwrap());
    self.sink.skip_one();
  }
  
  pub fn prev(&self) {
    *self.queue_idx.lock().unwrap() -= 1;
    self.reset_queue();
    // println!("qidx(-): {}", *self.queue_idx.lock().unwrap());
    self.sink.play();
  }

  fn set_now_playing(&mut self) {
    let idx = *self.queue_idx.lock().unwrap();
    let track = self.queue.get(idx);
    self.now_playing = track.cloned();
  }

  pub fn now_playing(&self) -> Option<&Track> {
    self.now_playing.as_ref()
  }

  fn total_duration(&self) -> Duration {
    match &self.now_playing {
      Some(track) => track.duration,
      None => Duration::ZERO
    }
  }

  pub fn current_duration(&self) -> Duration {
    self.sink.get_pos()
  }

  pub fn print_duration(&self, current_dur: Duration) -> String {
    let tot_duration = self.total_duration();
    let tot_duration_secs = tot_duration.as_secs() % 60;
    let tot_duration_min = (tot_duration.as_secs() - tot_duration_secs) / 60;

    let cur_duration_secs = current_dur.as_secs() % 60;
    let cur_duration_min = (current_dur.as_secs() - cur_duration_secs) / 60;

    format!(
      "{:02}:{:02} / {:02}:{:02}", 
      cur_duration_min,
      cur_duration_secs,
      tot_duration_min,
      tot_duration_secs
    )
  }

  fn a_to_b(source: Decoder<BufReader<File>>) {
    // start: 1:40, stop: 2:12
    source.skip_duration(Duration::from_secs(100))
      .take_duration(Duration::from_secs(32))
      .repeat_infinite();
  }
}