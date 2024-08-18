use std::path::Path;
use clap::Parser;

mod audio;
use audio::player;


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

  player::init(tracks);

  // println!("Tracks: {:#?}", tracks);
}