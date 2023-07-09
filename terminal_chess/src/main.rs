use terminal_chess::*;

fn main() {
    // print_piece(BLACK, K);

    let chess = Chess::new([
        [r, n, b, q, k, b, n, r],
        [p, p, p, p, p, p, p, p],
        [E, E, E, E, E, E, E, E],
        [E, E, E, E, E, E, E, E],
        [E, E, E, E, P, E, E, E],
        [E, E, E, E, E, E, E, E],
        [P, P, P, P, E, P, P, P],
        [R, N, B, Q, K, B, N, R],
    ]);

    chess.print_board();

    chess.move_piece("a2a4");
}
