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
                ";" => println!(""),

                "I" => game.current_tmino = Some(make_i_tmino()),
                "O" => game.current_tmino = Some(make_o_tmino()),
                "Z" => game.current_tmino = Some(make_z_tmino()),
                "S" => game.current_tmino = Some(make_s_tmino()),
                "J" => game.current_tmino = Some(make_j_tmino()),
                "L" => game.current_tmino = Some(make_l_tmino()),
                "T" => game.current_tmino = Some(make_t_tmino()),

                ")" => match game.current_tmino.clone() {
                    None => println!("No current tmino"),
                    Some(ref tmino) => {
                        game.current_tmino = Some(rotate_clockwise(tmino))},
                },
                _ => println!("Bad Command \"{}\"!", command),
            }
        }
    }
}
