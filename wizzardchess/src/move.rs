use crate::pieces::{set_up_board, BLACK_PIECES, WHITE_PIECES};


fn move_position(inserted_piece: Piece, new_position: Posistion){
    let old_rank = inserted_piece::posision::rank; 
    let old_file = inserted_piece::posision::file;  

    inserted_piece::posision::file = new_position::file;
    inserted_piece::posision::rank = new_position::rank;  
}


fn check_position(){

}

fn remove_piece(){

}

