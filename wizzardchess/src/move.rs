use crate::pieces::{set_up_board, BLACK_PIECES, WHITE_PIECES};
use std::{ops::DerefMut, sync::{Mutex, MutexGuard}};


fn move_position(inserted_piece: Piece, new_position: Posistion){
    let old_rank = inserted_piece::posision::rank; 
    let old_file = inserted_piece::posision::file;  

    inserted_piece::posision::file = new_position::file;
    inserted_piece::posision::rank = new_position::rank;  
}

// Check if position is taken
fn check_position(position: Posistion) -> Option<Piece>{

    if (){

    }
}


fn piece_position_in_array(piece: Piece, mut array: MutexGuard<[Option<Piece>;16]>) -> u8 {

}

// Takes inn the array where the piece shall be removed from and the position of the piece in the array.
fn remove_piece(mut array: MutexGuard<[Option<Piece>;16]>, array_position: u8){
    let array_pieces: &mut [Option<Piece>; 16] = array.deref_mut();

    array_pieces[array_position] = None; 
}

