
pub struct Board {
  // &str not working here why ??
  pub current_state: Vec<Vec<String>>,
  pub current_player: String,
}

impl Default for Board {
  fn default() -> Board {
    Board {
      current_state: vec![vec![String::from("");3];3],
      current_player: String::from("X"),
    }
  }
}

impl std::fmt::Display for Board {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    println!(" ___ ___ ___");
    for row in self.current_state.iter() {
      for col in 0..row.len() {
          let data: String = match row[col].as_str()  {
              "" => String::from(" "),
              "X" => String::from("X"),
              "O" => String::from("O"),
              _ => String::from("E"),
          };
          write!(f,"| {} ", data)?;
          if col == 2 {println!("|")};
      }
      writeln!(f," ___ ___ ___")?;
  }
  Ok(())
  }
}

impl Board {
  pub fn get_current(&self) -> String {
    return self.current_player.clone();
  }

  pub fn switch_current_player(self) -> Board {
    let value = String::from(if self.current_player == "X" {"O"} else {"X"});
    Board {
      current_player: value,
      ..self
    }
  }
  // Didn't work due to ownership issue, anti pattern ?
  fn is_field_empty(&self,row: usize, col: usize) -> bool {
    return if self.current_state[row][col] == "" {
      true
    } else {
      false
    };
  // }
  pub fn update_field(self, row: usize, col: usize, value: String) -> Result<Board,> {
    // if self.current_state[row][col] == "" {
      if self.is_field_empty(row, col) {
      let mut state = self.current_state;
      state[row][col] = value;
       return Ok(Board {
        current_state: state,
        ..self
      });
    } else {
        println!("The position is already filled!!!");
        return Err(None)
    };
  }
}