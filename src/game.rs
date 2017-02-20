use std::io;

const EMPTY: char = '.';
const WIDTH: usize = 10;
const HEIGHT: usize = 22;
type CellData = char;
type BoardRow = Vec<CellData>;
type Board = Vec<BoardRow>;

#[derive(Debug, Clone)]
pub struct Tmino {
    pub name: char,
    pub grid: Board,
}

pub fn make_i_tmino() -> Tmino {
    let grid = vec!(vec!(EMPTY, EMPTY, EMPTY, EMPTY),
                    vec!('c', 'c', 'c', 'c'),
                    vec!(EMPTY, EMPTY, EMPTY, EMPTY),
                    vec!(EMPTY, EMPTY, EMPTY, EMPTY));
    return Tmino { name: 'I', grid: grid };
}

pub fn make_o_tmino() -> Tmino {
    let grid = vec!(vec!('y', 'y'),
                    vec!('y', 'y'));
    return Tmino { name: 'O', grid: grid };
}

pub fn make_z_tmino() -> Tmino {
    let grid = vec!(vec!('r', 'r', EMPTY),
                    vec!(EMPTY, 'r', 'r'),
                    vec!(EMPTY, EMPTY, EMPTY));
    return Tmino { name: 'Z', grid: grid };
}

pub fn make_s_tmino() -> Tmino {
    let grid = vec!(vec!(EMPTY, 'g', 'g'),
                    vec!('g', 'g', EMPTY),
                    vec!(EMPTY, EMPTY, EMPTY));
    return Tmino { name: 'S', grid: grid };
}

pub fn make_j_tmino() -> Tmino {
    let grid = vec!(vec!('b', EMPTY, EMPTY,),
                    vec!('b', 'b', 'b'),
                    vec!(EMPTY, EMPTY, EMPTY));
    return Tmino { name: 'J', grid: grid };
}

pub fn make_l_tmino() -> Tmino {
    let grid = vec!(vec!(EMPTY, EMPTY, 'o'),
                    vec!('o', 'o', 'o'),
                    vec!(EMPTY, EMPTY, EMPTY));
    return Tmino { name: 'L', grid: grid };
}

pub fn make_t_tmino() -> Tmino {
    let grid = vec!(vec!(EMPTY, 'm', EMPTY),
                    vec!('m', 'm', 'm'),
                    vec!(EMPTY, EMPTY, EMPTY));
    return Tmino { name: 'T', grid: grid };
}

pub fn rotate_clockwise(t: &Tmino) -> Tmino {
    let size = t.grid.len();
    let mut ret = Tmino { name: t.name, grid: vec!(vec!(EMPTY; size); size) };
    for y in 0..size {
        for x in 0..size {
            ret.grid[y][x] = t.grid[size - x - 1][y];
        }
    }
    return ret;
}

pub fn print_grid(grid: &Board) {
    for row in grid {
        let mut row_string = "".to_string();
        for cell in row {
            if !row_string.is_empty() {
                row_string.push(' ');
            }
            row_string.push(*cell);
        }
    println!("{}", row_string);
    }
}

/// The current state of the Tetris game.
#[derive(Debug, Clone)]
pub struct GameState {
    pub board: Board,
    pub score: i32,
    pub lines_cleared: i32,
    pub current_tmino: Option<Tmino>,
}

pub fn empty_game() -> GameState {
    let empty_board = vec!(vec!(EMPTY; WIDTH); HEIGHT);
    return GameState {
        board: empty_board,
        score: 0,
        lines_cleared: 0,
        current_tmino: None};
}

impl GameState {
    /// Print the state of the game to stdout.
    pub fn print(&self) { print_grid(&self.board); }

    /// Read in a game board state from stdin.
    pub fn read_board(&mut self) {
        let mut new_board = vec!(vec!(EMPTY; WIDTH); HEIGHT);
        for rownum in 0..HEIGHT {
            let mut line_str = String::new();
            io::stdin().read_line(&mut line_str)
                .expect("Failed to read line");
            let line_str = line_str.trim();
            let mut row = vec!('.'; WIDTH);
            for (i, item) in line_str.split_whitespace().enumerate() {
                row[i] = item.chars().nth(0).unwrap();
            }
            new_board[rownum] = row;
        }
        self.board = new_board;
    }

    /// Advance the game by one "tick".
    pub fn step(&mut self) {
        let mut new_board = self.board.clone();
        for rownum in 0..HEIGHT {
            if new_board[rownum].iter().all(|&x| x != EMPTY) {
                new_board[rownum] = vec!('.'; WIDTH);
                self.score += 100;
                self.lines_cleared += 1;
            }
        }
        self.board = new_board;
    }
}
