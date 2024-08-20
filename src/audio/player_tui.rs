use std::io::Result;
use ratatui::{buffer::Buffer, crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style, Stylize}, text::{Line, Span, Text}, widgets::{block::Title, Block, Borders, Cell, Gauge, List, Padding, Paragraph, Row, ScrollbarState, StatefulWidget, Table, TableState, Widget}, Frame};
use crate::tui;

use super::{player::Player, Track};

pub struct PlayerTUI {
  pub playlist: Vec<Track>,
  playlist_state: TableState,
  playlist_scroll_state: ScrollbarState,
  pub player: Player,
  exit: bool
}

impl PlayerTUI {
  pub fn new(tracks: Vec<Track>) -> Self {
    Self {
      playlist_state: TableState::default().with_selected(0),
      playlist_scroll_state: ScrollbarState::new(tracks.len() - 1),
      exit: false,
      playlist: tracks,
      player: Player::init()
    }
  }

  pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
    while !self.exit {
      terminal.draw(|frame| {
        self.render_frame(frame);
      })?;

      self.handle_events()?;
    }

    Ok(())
  }

  fn render_frame(&self, frame: &mut Frame) {
    frame.render_widget(self, frame.area());
  }

  pub fn down(&mut self) {
    let i = match self.playlist_state.selected() {
      Some(i) => {
        if i >= self.playlist.len() - 1 {
          0
        } else {
          i + 1
        }
      },
      None => 0
    };

    self.playlist_state.select(Some(i));
    self.playlist_scroll_state = self.playlist_scroll_state.position(i);
  }

  pub fn up(&mut self) {
    let i = match self.playlist_state.selected() {
      Some(i) => {
        if i == 0 {
          self.playlist.len() - 1
        } else {
          i - 1
        }
      },
      None => 0
    };

    self.playlist_state.select(Some(i));
    self.playlist_scroll_state = self.playlist_scroll_state.position(i);
  }

  fn handle_events(&mut self) -> Result<()> {
    match event::read()? {
      Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
        self.handle_key_event(key_event)
      },
      _ => {}
    };

    Ok(())
  }

  fn handle_key_event(&mut self, event: KeyEvent) {
    match event.code {
      KeyCode::Up => self.up(),
      KeyCode::Down => self.down(),
      KeyCode::Enter => self.play(),
      KeyCode::Char('q') | KeyCode::Char('Q') => self.exit(),
      _ => {}
    }
  }

  /*fn set_queue(&'a mut self, start_idx: usize) {
    let playlist_count = self.playlist.len();
    for i in 0..self.playlist.len() {
      let idx = (playlist_count + i + start_idx) % playlist_count;
      let track = self.playlist.get(idx).unwrap();
      self.queue.push(track);
    }

    println!("Queue set");
  }*/

  pub fn play(&mut self) {
    self.player.set_queue(&self.playlist, 0);
    self.player.play();
  }

  fn exit(&mut self) {
    self.exit = true;
  }
}

impl Widget for &PlayerTUI {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let wrapper_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Length(1),
        Constraint::Min(1),
        ])
      .split(area);

    Paragraph::new("Music Player")
      .centered()
      .bold()
      .style(
        Style::new()
          .fg(Color::from_u32(0xFFd0d0d0))
          .bg(Color::from_u32(0xFF091d26))
        )
        .render(wrapper_layout[0], buf);

    let main_layout = Layout::default()
      .direction(Direction::Horizontal)
      .constraints(vec![
        Constraint::Min(70),
        Constraint::Max(25)
      ])
      .split(wrapper_layout[1]);

    let left_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Min(5),
        Constraint::Length(7),
      ])
      .split(main_layout[0]);

    let lib_block = Block::bordered()
      .bg(Color::from_u32(0xFF091d26))
      .fg(Color::from_u32(0xFFd0d0d0))
      .title(Title::from("Library"))
      .padding(Padding::proportional(1));

    let rows = self.playlist.iter().map(|track| {
      let t = [&track.title, &track.artist, &track.album];
      t.into_iter().map(|content| Cell::from(Text::from(format!("{content}")))).collect::<Row>()
    });
  
    let widths = [
      Constraint::Percentage(40),
      Constraint::Ratio(1, 3),
      Constraint::Max(8)
    ];

    let table = Table::new(rows, widths)
      .column_spacing(1)
      // .fg(Color::from_u32(0xFF124d54))
      .header(
        Row::new(vec!["Title", "Artist", "Album"])
          .style(Style::new().bold())
          // .bottom_margin(1)
      )
      .highlight_style(Style::new().reversed())
      // .highlight_symbol(">>")
      .block(lib_block);

    let mut state = self.playlist_state.clone();
    StatefulWidget::render(table, left_layout[0], buf, &mut state);

    let now_playing_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints(vec![
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1)
      ])
      .split(left_layout[1]);

    let now_playing_block = Block::new()
      .bg(Color::from_u32(0xFF091d26))
      .fg(Color::from_u32(0xFFd0d0d0))
      .borders(Borders::TOP)
      .title_alignment(Alignment::Center)
      .title(Title::from(" Now Playing "));

    Paragraph::new(Text::from(""))
      .block(now_playing_block)
      .render(now_playing_layout[0], buf);
    Paragraph::new(Text::from("Track Title"))
      .bg(Color::from_u32(0xFF091d26))
      .fg(Color::from_u32(0xFF00bebe))
      .bold()
      .render(now_playing_layout[1], buf);
    Paragraph::new(Text::from("Artist Name"))
      .bg(Color::from_u32(0xFF091d26))
      .fg(Color::from_u32(0xFFaaaaaa))
      .render(now_playing_layout[2], buf);
    Paragraph::new("")
      .bg(Color::from_u32(0xFF091d26))
      .render(now_playing_layout[3], buf);
    
    Gauge::default()
    .gauge_style(
      Style::default()
      .fg(Color::from_u32(0xFF48cbc5))
      .bg(Color::from_u32(0xFF094044))
      // .bold()
      // .italic()
    )
    .percent(20)
    .label("0:00 / 2:30")
    .render(now_playing_layout[4], buf);

    let player_state_layout = Layout::horizontal([
      Constraint::Min(30),
      Constraint::Length(15)
    ]).split(now_playing_layout[5]);

    let lines = vec![
      Line::from(vec![
        Span::from("Shuffle: "),
        Span::styled("On", Style::new().bold().fg(Color::from_u32(0xFF48CBC5))),
        Span::from(" | "),
        Span::from("Repeat: "),
        Span::styled("Off", Style::default()),
      ])
    ];
    Paragraph::new(Text::from(lines))
      .bg(Color::from_u32(0xFF091d26))
      .fg(Color::DarkGray)
      .render(player_state_layout[0], buf);
    Paragraph::new("Volume: 55%")
      .bg(Color::from_u32(0xFF091d26))
      .fg(Color::from_u32(0xFF48cbc5))
      .alignment(Alignment::Right)
      .render(player_state_layout[1], buf);
    Paragraph::new("")
      .bg(Color::from_u32(0xFF091d26))
      .render(now_playing_layout[6], buf);

    let control_block = Block::new()
      .bg(Color::from_u32(0xFF091d26))
      .padding(Padding::proportional(1));

    let list = List::new([
      "Keyboard Controls\n\n",
      "[↑] Move Up\n\n",
      "[↓] Move Down\n\n",
      "[←] Previous Track\n\n",
      "[→] Next Track\n\n",
      "[Enter] Select Track\n\n",
      "[Space] Play/Pause\n\n",
      "[Ctrl ↑] Volume Up\n\n",
      "[Ctrl ↓] Volume Down\n\n",
      "[Ctrl -] Slower\n\n",
      "[Ctrl +] Faster\n\n",
      "[M] Mute\n\n",
      "[Q] Quit\n\n"
    ])
    .style(
      Style::default()
      .fg(Color::from_u32(0xFF636363))
    )
    .block(control_block);
  
    Widget::render(list, main_layout[1], buf);
  }

}
