use rodio::{Sample, Source};

pub struct CustomSource<S, F>(S, Option<F>);

impl<S, F> Iterator for CustomSource<S, F> 
where 
  S: Source,
  S::Item: Sample,
  F: FnOnce() 
{
  type Item = S::Item;

  #[inline]
  fn next(&mut self) -> Option<S::Item> {
    match self.0.next() {
      Some(n) => Some(n),
      None => {
        self.1.take().unwrap()();
        None
      }
    }
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    self.0.size_hint()
  }
}

impl<S, F> Source for CustomSource<S, F>
where
  S: Source,
  S::Item: Sample,
  F: FnOnce()
{
  #[inline]
  fn current_frame_len(&self) -> Option<usize> {
    self.0.current_frame_len()
  }

  #[inline]
  fn channels(&self) -> u16 {
    self.0.channels()
  }

  #[inline]
  fn sample_rate(&self) -> u32 {
    self.0.sample_rate()
  }

  #[inline]
  fn total_duration(&self) -> Option<std::time::Duration> {
    self.0.total_duration()
  }
}

impl<S, F> CustomSource<S, F>
where
  S: Source,
  S::Item: Sample,
  F: FnOnce()
{
  pub fn wrap(source: S, f: F) -> Self {
    Self(source, Some(f))
  }
}




