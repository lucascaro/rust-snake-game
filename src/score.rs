pub struct Score {
  current_score: u32,
  high_scores: Vec<u32>,
}

impl Score {
  pub fn new() -> Score {
    Score {
      current_score: 0,
      high_scores: vec![],
    }
  }

  pub fn from(file_name: &str) -> Score {
    Score {
      current_score: 0,
      high_scores: vec![],
    }
  }

  pub fn set_score(&mut self, score: u32) {
    self.current_score = score;
  }
}
