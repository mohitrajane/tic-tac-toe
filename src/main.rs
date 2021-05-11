mod player;
mod board;
use std::io::stdin;
use player::Player;
use board::Board;

#[allow(dead_code)]
fn create_player() -> Player {
    let mut line  = String::new();
    stdin().read_line(&mut line).ok().expect("Couldn't read line");
    return Player {
        name: line,
        ..Default::default()
    };
}
fn main() {
    #[allow(unused_mut)]
    let mut game_board = Board{..Default::default()};
    // println!("Welcome to TIC-TAC-TOC\nEnter Player X's name:");
    // let player_x = create_player();
    // println!("Enter Player O's name:");
    // let player_o = create_player();
    // println!("{}", player_x);
    // println!("{}", player_o);

    loop {

        let player = game_board.get_current();
        println!("{}", game_board);
        println!("Current:{}", player);
        let mut line  = String::new();
        println!("Enter value(x,y):");
        stdin().read_line(&mut line).ok().expect("Couldn't read line");
        let position = line.trim().split(',').collect::<Vec<&str>>();
        // let result 
        game_board = match game_board.update_field(
            position[0].parse::<usize>().unwrap(),
            position[1].parse::<usize>().unwrap(),
            player
        )   {
            Some(b) => b,
            None =>  println!("None"),
        };
        // let &mut new_game_board;
        // match result {
        //     Some(b) => {
        //         println!("{}",b);
        //         // game_board
        //         new_game_board = b;
        //     },
        //     None =>  println!("None")
        // };
        game_board = game_board.switch_current_player();
        //  Hack to clear screen no idea how it works
        print!("\x1B[2J\x1B[1;1H");
    }
}
