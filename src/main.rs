use std::path::Path;
use clap::Parser;

mod audio;
mod tui;

use audio::player_tui::PlayerTUI;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Input directory
  #[arg(short, long)]
  dir: String
}

fn main() {
  let args = Args::parse();
  let dir = Path::new(&args.dir);
  let tracks = audio::get_tracks(dir).unwrap();

  let mut terminal = tui::init().unwrap();
  let player_tui = PlayerTUI::new(tracks);
  player_tui.run(&mut terminal);

  // println!("Tracks: {:#?}", tracks);
}