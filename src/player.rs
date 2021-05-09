// #[derive(Default)]
pub struct Player {
  pub name: String,
  pub no_of_moves: u8,
}

// https://stackoverflow.com/a/36439447/7157529
impl std::fmt::Display for Player {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "\nPlayer\nName: {}\n", self.name)
  }
}

// https://doc.rust-lang.org/std/fmt/index.html#formatting-traits
impl std::fmt::Debug for Player {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "\nPlayer\nName: {} \nNo of moves: {}\n",
      self.name, self.no_of_moves
    )
  }
}

impl Default for Player {
  fn default() -> Player {
    Player {
      name: String::from("GAMER"),
      no_of_moves: 0,
    }
  }
}
