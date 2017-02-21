mod game;

use std::io;
use game::*;

fn main() {
    io_loop()
}

fn io_loop() {
    let mut game = empty_game();
    loop {
        let mut commands = String::new();
        io::stdin().read_line(&mut commands)
            .expect("Failed to read line");
        let mut query_mode = false;
        for command in commands.trim().chars() {
            match command {
                ' ' => continue,
                'q' => return,
                'p' => game.print(),
                'g' => game.read_board(),
                'c' => game = empty_game(),
                '?' => query_mode = true,
                's' => if query_mode {
                           println!("{}", game.score)
                       } else {
                           game.step()
                       },
                'n' => if query_mode {
                           println!("{}", game.lines_cleared)
                       } else {
                           println!("Bad command!")
                       },
                't' => match game.current_tmino {
                    None => println!("No current tmino"),
                    Some(ref tmino) => print_grid(&tmino.grid),
                },
                ';' => println!(""),

                'P' => print_grid(&game.board_overlaying_current()),

                'I' => game.start_new_tmino(make_i_tmino()),
                'O' => game.start_new_tmino(make_o_tmino()),
                'Z' => game.start_new_tmino(make_z_tmino()),
                'S' => game.start_new_tmino(make_s_tmino()),
                'J' => game.start_new_tmino(make_j_tmino()),
                'L' => game.start_new_tmino(make_l_tmino()),
                'T' => game.start_new_tmino(make_t_tmino()),

                ')' => match game.current_tmino.clone() {
                    None => println!("No current tmino"),
                    Some(ref tmino) => {
                        game.current_tmino = Some(rotate_clockwise(tmino))}},
                '(' => match game.current_tmino.clone() {
                    None => println!("No current tmino"),
                    Some(ref tmino) => {
                        game.current_tmino = Some(rotate_counterclockwise(tmino))}},
                '<' => game.shift_left(),
                '>' => game.shift_right(),
                'v' => game.shift_down(),
                _ => { println!("Bad Command \'{}\'!", command); return; },
            }
            if command != '?' { query_mode = false; }
        }
    }
}
