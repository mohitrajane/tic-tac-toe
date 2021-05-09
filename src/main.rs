mod player;
use std::io::stdin;
use player::Player;

fn create_player() -> Player {
    let mut line  = String::new();
    stdin().read_line(&mut line).ok().expect("Couldn't read line");
    return Player {
        name: line,
        ..Default::default()
    };
}
fn main() {
    let mut board: [[&str;3];3] = [["";3];3];
    board[0][2] ="X";
    board[2][0] ="O";
    println!("Welcome to TIC-TAC-TOC\nEnter Player X's name:");
    let player_x = create_player();
    println!("Enter Player O's name:");
    let player_o = create_player();
    println!("{}", player_x);
    println!("{}", player_o);

    //  Hack to clear screen no idea how it works
    print!("\x1B[2J\x1B[1;1H");

    println!(" ___ ___ ___");
    for row in board.iter() {
        for col in 0..row.len() {
            match row[col] {
                "" => print!("|   "),
                "X" => print!("| X "),
                "O" => print!("| O "),
                _ => print!("E")
            }
            if col == 2 {println!("|")};
        }
        println!(" ___ ___ ___");
    }

}
