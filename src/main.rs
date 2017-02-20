use std::io;

const EMPTY: char = '.';
const WIDTH: usize = 10;
const HEIGHT: usize = 22;
type CellData = char;
type BoardRow = Vec<CellData>;
type Board = Vec<BoardRow>;

fn print_grid(grid: &Board) {
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
struct GameState {
    board: Board,
    score: i32,
    lines_cleared: i32,
    current_tmino: Option<Tmino>,
}

fn empty_game() -> GameState {
    let empty_board = vec!(vec!(EMPTY; WIDTH); HEIGHT);
    return GameState {
        board: empty_board,
        score: 0,
        lines_cleared: 0,
        current_tmino: None};
}

impl GameState {
    /// Print the state of the game to stdout.
    fn print(&self) { print_grid(&self.board); }

    /// Read in a game board state from stdin.
    fn read_board(&mut self) {
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
    fn step(&mut self) {
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

#[derive(Debug, Clone)]
struct Tmino {
    name: char,
    grid: Board,
}

fn make_i_tmino() -> Tmino {
    let grid = vec!(vec!(EMPTY, EMPTY, EMPTY, EMPTY),
                    vec!('c', 'c', 'c', 'c'),
                    vec!(EMPTY, EMPTY, EMPTY, EMPTY),
                    vec!(EMPTY, EMPTY, EMPTY, EMPTY));
    return Tmino { name: 'I', grid: grid };
}

fn make_o_tmino() -> Tmino {
    let grid = vec!(vec!('y', 'y'),
                    vec!('y', 'y'));
    return Tmino { name: 'O', grid: grid };
}

fn make_z_tmino() -> Tmino {
    let grid = vec!(vec!('r', 'r', EMPTY),
                    vec!(EMPTY, 'r', 'r'),
                    vec!(EMPTY, EMPTY, EMPTY));
    return Tmino { name: 'Z', grid: grid };
}

fn make_s_tmino() -> Tmino {
    let grid = vec!(vec!(EMPTY, 'g', 'g'),
                    vec!('g', 'g', EMPTY),
                    vec!(EMPTY, EMPTY, EMPTY));
    return Tmino { name: 'S', grid: grid };
}

fn make_j_tmino() -> Tmino {
    let grid = vec!(vec!('b', EMPTY, EMPTY,),
                    vec!('b', 'b', 'b'),
                    vec!(EMPTY, EMPTY, EMPTY));
    return Tmino { name: 'J', grid: grid };
}

fn make_l_tmino() -> Tmino {
    let grid = vec!(vec!(EMPTY, EMPTY, 'o'),
                    vec!('o', 'o', 'o'),
                    vec!(EMPTY, EMPTY, EMPTY));
    return Tmino { name: 'L', grid: grid };
}

fn make_t_tmino() -> Tmino {
    let grid = vec!(vec!(EMPTY, 'm', EMPTY),
                    vec!('m', 'm', 'm'),
                    vec!(EMPTY, EMPTY, EMPTY));
    return Tmino { name: 'T', grid: grid };
}

fn main() {
    io_loop()
}

fn io_loop() {
    let mut game = empty_game();
    loop {
        let mut commands = String::new();
        io::stdin().read_line(&mut commands)
            .expect("Failed to read line");
        for command in commands.split_whitespace() {
            match command {
                "q" => return,
                "p" => game.print(),
                "g" => game.read_board(),
                "c" => game = empty_game(),
                "?s" => println!("{}", game.score),
                "?n" => println!("{}", game.lines_cleared),
                "s" => game.step(),
                "t" => match game.current_tmino {
                    None => println!("No current tmino"),
                    Some(ref tmino) => print_grid(&tmino.grid),
                },
                "I" => game.current_tmino = Some(make_i_tmino()),
                "O" => game.current_tmino = Some(make_o_tmino()),
                "Z" => game.current_tmino = Some(make_z_tmino()),
                "S" => game.current_tmino = Some(make_s_tmino()),
                "J" => game.current_tmino = Some(make_j_tmino()),
                "L" => game.current_tmino = Some(make_l_tmino()),
                "T" => game.current_tmino = Some(make_t_tmino()),
                _ => println!("Bad Command \"{}\"!", command),
            }
        }
    }
}
