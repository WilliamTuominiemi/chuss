#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Clone, Copy, Debug)]
pub enum Player {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub kind: PieceType,
    pub color: Player,
}

pub struct Board {
    pub squares: [Option<Piece>; 64],
}

impl Board {
    pub fn index(x: usize, y: usize) -> usize {
        y * 8 + x
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Piece> {
        self.squares[Self::index(x, y)]
    }

    pub fn get_all_pieces(&self) -> [Option<Piece>; 64] {
        return self.squares;
    }

    pub fn set(&mut self, x: usize, y: usize, piece: Option<Piece>) {
        self.squares[Self::index(x, y)] = piece;
    }

    pub fn start_position() -> Self {
        let mut board = Board {
            squares: [None; 64],
        };

        use PieceType::*;
        use Player::*;

        // Place pawns
        for x in 0..8 {
            board.set(
                x,
                1,
                Some(Piece {
                    kind: Pawn,
                    color: White,
                }),
            );
            board.set(
                x,
                6,
                Some(Piece {
                    kind: Pawn,
                    color: Black,
                }),
            );
        }

        // White back row
        board.set(
            0,
            0,
            Some(Piece {
                kind: Rook,
                color: White,
            }),
        );
        board.set(
            1,
            0,
            Some(Piece {
                kind: Knight,
                color: White,
            }),
        );
        board.set(
            2,
            0,
            Some(Piece {
                kind: Bishop,
                color: White,
            }),
        );
        board.set(
            3,
            0,
            Some(Piece {
                kind: Queen,
                color: White,
            }),
        );
        board.set(
            4,
            0,
            Some(Piece {
                kind: King,
                color: White,
            }),
        );
        board.set(
            5,
            0,
            Some(Piece {
                kind: Bishop,
                color: White,
            }),
        );
        board.set(
            6,
            0,
            Some(Piece {
                kind: Knight,
                color: White,
            }),
        );
        board.set(
            7,
            0,
            Some(Piece {
                kind: Rook,
                color: White,
            }),
        );

        // Black back row
        board.set(
            0,
            7,
            Some(Piece {
                kind: Rook,
                color: Black,
            }),
        );
        board.set(
            1,
            7,
            Some(Piece {
                kind: Knight,
                color: Black,
            }),
        );
        board.set(
            2,
            7,
            Some(Piece {
                kind: Bishop,
                color: Black,
            }),
        );
        board.set(
            3,
            7,
            Some(Piece {
                kind: Queen,
                color: Black,
            }),
        );
        board.set(
            4,
            7,
            Some(Piece {
                kind: King,
                color: Black,
            }),
        );
        board.set(
            5,
            7,
            Some(Piece {
                kind: Bishop,
                color: Black,
            }),
        );
        board.set(
            6,
            7,
            Some(Piece {
                kind: Knight,
                color: Black,
            }),
        );
        board.set(
            7,
            7,
            Some(Piece {
                kind: Rook,
                color: Black,
            }),
        );

        board
    }
}
