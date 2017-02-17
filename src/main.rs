use std::io;

fn main() {
    io_loop()
}

fn io_loop() {
    let empty_game = GameState { board: [[EMPTY; WIDTH]; HEIGHT],
                                 score: 0,
                                 lines_cleared: 0,
                                 current_tmino: None};
    let mut game = empty_game;
    loop {
        let mut command = String::new();
        io::stdin().read_line(&mut command)
            .expect("Failed to read line");
        let command = command.trim();
        match command {
            "q" => break,
            "p" => game.print(),
            "g" => game.read_board(),
            "c" => game = empty_game,
            "?s" => println!("{}", game.score),
            "?n" => println!("{}", game.lines_cleared),
            "s" => game.step(),
            "t" => match game.current_tmino {
                None => println!("No current tmino"),
                Some(x) => game.print_current_tmino(),},
            _ => println!("Bad Command!"),
        }
    }
}

const EMPTY: char = '.';
const WIDTH: usize = 10;
const HEIGHT: usize = 22;
type CellData = char;
type BoardRow = [CellData; WIDTH];
type Board = [BoardRow; HEIGHT];

fn print_grid(grid: &[[CellData]]) {
    for row in grid.iter() {
        let mut row_string = "".to_string();
        for cell in row.iter() {
            if !row_string.is_empty() {
                row_string.push(' ');
            }
            row_string.push(*cell);
        }
        println!("{}", row_string);
    }
}

/// The current state of the Tetris game.
#[derive(Debug, Copy, Clone)]
struct GameState {
    board: Board,
    score: i32,
    lines_cleared: i32,
    current_tmino: Option<Tmino>,
}

impl GameState {
    /// Print the state of the game to stdout.
    fn print(&self) { print_grid(self.board); }

    fn print_current_tmino(&self) { print_grid(self.current_tmino.grid); }

    /// Read in a game board state from stdin.
    fn read_board(&mut self) {
        let mut new_board = [[EMPTY; WIDTH]; HEIGHT];
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

    /// Advance the game by one "tick".
    fn step(&mut self) {
        let mut new_board = self.board;
        for rownum in 0..HEIGHT {
            if new_board[rownum].iter().all(|&x| x != EMPTY) {
                new_board[rownum] = ['.'; WIDTH];
                self.score += 100;
                self.lines_cleared += 1;
            }
        }
        self.board = new_board;
    }
}

const TMINO_HEIGHT: usize = 4;
const TMINO_WIDTH: usize = 4;
type TminoRow = [CellData; TMINO_WIDTH];
type TminoGrid = [TminoRow; TMINO_HEIGHT];

#[derive(Debug, Copy, Clone)]
struct Tmino {
    name: char,
    grid: TminoGrid,
}

const I_TMINO: Tmino = Tmino {name: 'I',
                              grid: [[EMPTY, EMPTY, EMPTY, EMPTY],
                                     ['c', 'c', 'c', 'c'],
                                     [EMPTY, EMPTY, EMPTY, EMPTY],
                                     [EMPTY, EMPTY, EMPTY, EMPTY]]};

