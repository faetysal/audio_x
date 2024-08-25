use std::{path::Path, thread, time::Duration};
use clap::Parser;

mod audio;
mod tui;

use audio::{player::Player, player_tui::PlayerTUI};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Input directory
  #[arg(short, long)]
  dir: String
}

fn main() -> std::io::Result<()> {
  let args = Args::parse();
  let dir = Path::new(&args.dir);
  let tracks = audio::get_tracks(dir).unwrap();

  let mut terminal = tui::init().unwrap();

  let mut player_tui = PlayerTUI::new(tracks);
  let app_result = player_tui.run(&mut terminal);

  tui::restore()?;

  app_result

  /* CLI Only */
  /*let handle = thread::spawn(move || {
    let mut player = Player::init();
    player.set_queue(&tracks, 0);
    // player.play();
  
    let track = player.now_playing().unwrap();
    println!("Title: {}", track.get_title());

    thread::sleep(Duration::from_secs(15));
  });

  handle.join().unwrap();

  Ok(())*/
}