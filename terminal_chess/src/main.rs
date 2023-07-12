use std::io;
use terminal_chess::*;

fn main() {
    // print_piece(BLACK, K);

    let mut chess = Chess::new_game();
    loop {
        chess.print_board();
        println!();
        println!("Your move: ");
        let mut chess_move = String::new();

        io::stdin()
            .read_line(&mut chess_move)
            .expect("Failed to read line");

        match chess.move_piece(&chess_move.trim()) {
            Ok(_) => (),
            Err(err) => println!("Problem parsing move: {err}"),
        }
    }
}
