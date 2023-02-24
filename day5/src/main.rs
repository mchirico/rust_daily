const BOARD_SIZE: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq)]
struct Square(i32, i32);


impl Square {
    fn new(x: i32, y: i32) -> Option<Self> {
        if x >= 0 && y >= 0 && x < BOARD_SIZE as i32 && y < BOARD_SIZE as i32 {
            Some(Square(x, y))
        } else {
            None
        }
    }

    fn neighbors(&self) -> Vec<Square> {
        let mut neighbors = Vec::new();

        for &(dx, dy) in &[(2, 1), (1, 2), (-1, 2), (-2, 1), (-2, -1), (-1, -2), (1, -2), (2, -1)] {
            if let Some(square) = Square::new(self.0 + dx, self.1 + dy) {
                neighbors.push(square);
            }
        }

        neighbors
    }
}

struct Chessboard {
    board: [[Option<usize>; BOARD_SIZE]; BOARD_SIZE],
    count: usize,
}

impl Chessboard {
    fn new() -> Self {
        Chessboard {
            board: [[None; BOARD_SIZE]; BOARD_SIZE],
            count: 0,
        }
    }

    fn place_knight(&mut self, square: Square, count: usize) {
        self.board[square.0 as usize][square.1 as usize] = Some(count);
        self.count += 1;
    }

    fn remove_knight(&mut self, square: Square) {
        self.board[square.0 as usize][square.1 as usize] = None;
        self.count -= 1;
    }

    fn is_visited(&self, square: Square) -> bool {
        self.board[square.0 as usize][square.1 as usize].is_some()
    }

    fn is_complete(&self) -> bool {
        self.count == BOARD_SIZE * BOARD_SIZE
    }

    fn get_legal_moves(&self, square: Square) -> Vec<Square> {
        let mut legal_moves = Vec::new();

        for neighbor in square.neighbors() {
            if !self.is_visited(neighbor) {
                legal_moves.push(neighbor);
            }
        }

        legal_moves
    }

    fn get_best_move(&self, square: Square) -> Option<Square> {
        let legal_moves = self.get_legal_moves(square);

        if legal_moves.is_empty() {
            None
        } else if legal_moves.len() == 1 {
            Some(legal_moves[0])
        } else {
            let mut neighbor_counts = Vec::new();

            for neighbor in legal_moves {
                let count = self.get_legal_moves(neighbor).len();
                neighbor_counts.push((count, neighbor));
            }

            neighbor_counts.sort();
            Some(neighbor_counts[0].1)
        }
    }

    fn solve(&mut self, square: Square, count: usize) -> bool {
        self.place_knight(square, count);

        if self.is_complete() {
            return true;
        }

        let next_move = self.get_best_move(square);

        if let Some(next_square) = next_move {
            if self.solve(next_square, count + 1) {
                return true;
            }
        }

        self.remove_knight(square);
        false
    }
}

fn main() {
    let mut chessboard = Chessboard::new();

    if chessboard.solve(Square(0, 0), 0) {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                print!("{:3} ", chessboard.board[row][col].unwrap());
            }
            println!();
        }
    } else {
        println!("No solution found");
    }
}


