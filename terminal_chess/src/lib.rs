pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
}

pub fn empty_board() -> String {
    String::from(
        "❰ ❱⟦♔ ⟧",
    )
}
