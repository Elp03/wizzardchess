use crate::pieces::{set_up_board, BLACK_PIECES, WHITE_PIECES};

pub fn test_fill_array_with_pieces(){
	set_up_board();

	let white_pieces = WHITE_PIECES.lock().unwrap();

    // Print the values stored in WHITE_PIECES
    for (index, piece) in white_pieces.iter().enumerate() {
        if let Some(piece) = piece {
            println!("Index {}: {:?}", index, piece);
        } else {
            println!("Index {}: None", index);
        }
    }
	
	let black_pieces = BLACK_PIECES.lock().unwrap();

    // Print the values stored in BLACK_PIECES
    for (index, piece) in black_pieces.iter().enumerate() {
        if let Some(piece) = piece {
            println!("Index {}: {:?}", index, piece);
        } else {
            println!("Index {}: None", index);
        }
    }
}
