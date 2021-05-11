use std::{thread, time};

pub struct Board {
  // &str not working here why ??
  pub current_state: Vec<Vec<String>>,
  pub current_player: String,
  // TODO: should be not pub  ?
  pub no_of_moves: u8,
  pub winner: String,
}

impl Default for Board {
  fn default() -> Board {
    Board {
      current_state: vec![vec![String::from("");3];3],
      current_player: String::from("X"),
      no_of_moves: 0,
      winner: String::from(""),
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

  pub fn switch_current_player(&mut self) {
    let value = String::from(if self.current_player == "X" {"O"} else {"X"});
    self.current_player = value;
  }

  fn is_field_empty(&self,row: usize, col: usize) -> bool {
    return if self.current_state[row][col] == "" {
      true
    } else {
      false
    };
  }

  pub fn is_game_over(&self) -> bool {
    if self.no_of_moves > 8 || self.winner != "" {
      return true
    } else {
      false
    }

  }
  fn update_no_of_move(&mut self) {
    self.no_of_moves = self.no_of_moves + 1;
  }

  pub fn update_field(&mut self, row: usize, col: usize, value: String) {

      if self.is_field_empty(row, col) {
      self.current_state[row][col] = value;
      self.switch_current_player();
      self.update_no_of_move();
    } else {
        println!("The position is already filled!!!\nPlease re-entry");
        thread::sleep(time::Duration::from_secs(2));
    };
  }
  fn is_diagonal_win(&mut self) {

    if self.current_state[0][0] != "" && self.current_state[0][0] == self.current_state[1][1] && self.current_state[1][1] == self.current_state[2][2] { 
      self.winner = self.current_state[0][0].clone();
      // return true;
    } else if self.current_state[0][2]!="" &&self.current_state[0][2] == self.current_state[1][1] && self.current_state[1][1] == self.current_state[2][1] { 
      println!("ELSE IF CONDITION");
      self.winner = self.current_state[0][2].clone();
      // return true;
    } else {
      // return false;
    }
  }

  fn is_across_win(&mut self) {
    for i in 0..3{
      if self.current_state[i][0] != "" && self.current_state[i][0] == self.current_state[i][1] && self.current_state[i][1] == self.current_state[i][2] {
        self.winner = self.current_state[i][0].clone();
        // return true;
        break;
      }
    }
    // return false;
  }
  fn is_vertical_win(&mut self) {
    for i in 0..3 {
      if self.current_state[0][i] != "" && self.current_state[0][i] == self.current_state[1][i] && self.current_state[1][i] == self.current_state[2][i] {
        self.winner = self.current_state[0][i].clone();
        // return true;
        break;
      }
    }
    // return false;
  }
  pub fn find_winner(&mut self) {
    // TODO NATIVE FIND BETTER SOLUTION
    self.is_diagonal_win();
    self.is_across_win();
    self.is_vertical_win();
  }
}