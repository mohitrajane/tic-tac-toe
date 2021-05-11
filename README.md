# Tic Tac Toe 

The old school tic tac toe game implemented in rust. The idea is to get familiarized with rust.

## Running

1. Install rust
    For macos / linux (taken from [official docs](https://www.rust-lang.org/tools/install))

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2. clone repo
3. open the cloned folder in a terminal
4. `cargo run`

The main branch might not be always working, if it's not working try branch `vX`

### To DO

#### V1 :white_check_mark:
~1. Able to play the game and show the winner or detect if draw~
##### V2
1. Add validation to position input
2. Switch from String to str (not sure if thats better)
3. Better solution currently naive

#### V3
1. Refactor code
2. Try to remove all the mut used (is supposed to be the rust way?)
3. algo to find if only draw possible in next move ?


