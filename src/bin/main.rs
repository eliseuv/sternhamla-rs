use sternhalma_rs::Board;

fn main() {
    let board = Board::new().unwrap();

    println!("{board}");

    for (index, movements) in board.list_possible_first_movements(sternhalma_rs::Piece::Player1) {
        for movemvent in movements {
            let mut new_board = Board::new().unwrap();
            new_board.apply_movement(index, movemvent).unwrap();
            println!("{new_board}");
        }
    }
}
