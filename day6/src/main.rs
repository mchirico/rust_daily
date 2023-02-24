use std::io;

fn main() {
    let mut board = [
        ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'],
        ['P', 'P', 'P', 'P', 'P', 'P', 'P', 'P'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['.', '.', '.', '.', '.', '.', '.', '.'],
        ['p', 'p', 'p', 'p', 'p', 'p', 'p', 'p'],
        ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
    ];

    print_board(&board);

    loop {
        println!("Enter your move (e.g. e2e4):");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let x1 = input.chars().nth(0).unwrap() as usize - 'a' as usize;
        let y1 = input.chars().nth(1).unwrap() as usize - '1' as usize;
        let x2 = input.chars().nth(2).unwrap() as usize - 'a' as usize;
        let y2 = input.chars().nth(3).unwrap() as usize - '1' as usize;

        let piece = board[y1][x1];
        board[y1][x1] = '.';
        board[y2][x2] = piece;

        print_board(&board);
    }
}

fn print_board(board: &[[char; 8]; 8]) {
    println!("  a b c d e f g h");
    for y in 0..8 {
        print!("{} ", y + 1);
        for x in 0..8 {
            print!("{} ", board[y][x]);
        }
        println!("{}", y + 1);
    }
    println!("  a b c d e f g h");
}
