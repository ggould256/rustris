use std::io;

fn main() {
    io_loop()
}

fn io_loop() {
    let empty_game = GameState { board: [['.'; WIDTH]; HEIGHT],
                                 score: 0,
                                 lines_cleared: 0};
    let mut game = empty_game;
    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read line");
        let command = command.trim();
        if command == "q" { break; }
        if command == "p" { game.print(); }
        if command == "g" { game.read_board(); }
        if command == "c" { game = empty_game; }
        if command == "?s" { println!("{}", game.score); }
        if command == "?n" { println!("{}", game.lines_cleared); }
    }
}

const WIDTH: usize = 10;
const HEIGHT: usize = 22;
type CellData = char;
type BoardRow = [CellData; WIDTH];
type Board = [BoardRow; HEIGHT];

/// The current state of the Tetris game.
#[derive(Debug, Copy, Clone)]
struct GameState {
    board: Board,
    score: i32,
    lines_cleared: i32,
}

impl GameState {
    /// Print the state of the game to stdout.
    fn print(&self) {
        for row in self.board.into_iter() {
            let mut row_string = "".to_string();
            for cell in row.into_iter() {
                if !row_string.is_empty() {
                    row_string.push(' ');
                }
                row_string.push(*cell);
            }
            println!("{}", row_string);
        }
    }

    /// Read in a game board state from stdin.
    fn read_board(&mut self) {
        let mut new_board = [['.'; WIDTH]; HEIGHT];
        for rownum in 0..HEIGHT {
            let mut line_str = String::new();
            io::stdin().read_line(&mut line_str)
                .expect("Failed to read line");
            let line_str = line_str.trim();
            let mut row = ['.'; WIDTH];
            for (i, item) in line_str.split_whitespace().enumerate() {
                row[i] = item.chars().nth(0).unwrap();
            }
            new_board[rownum] = row;
        }
        self.board = new_board;
    }
}
