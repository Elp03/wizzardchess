use std::{ops::DerefMut, sync::{Mutex, MutexGuard}};


const ARRAY_REPEAT_VALUE: Option<Piece> = None;
pub static WHITE_PIECES: Mutex<[Option<Piece>; 16]> = Mutex::new([ARRAY_REPEAT_VALUE; 16]);
pub static BLACK_PIECES: Mutex<[Option<Piece>; 16]> = Mutex::new([ARRAY_REPEAT_VALUE; 16]);


// Possible board pieces
#[derive(Debug)] 
enum BoardPieces {
	Pawn, 
	Bishop,
	Knight,
	Rook, 
	Queen,
	King
}

// Board files
#[derive(Debug)] 
enum File {
	A, B, C, D, E, F, G, H,
}

// Board rank
#[derive(Debug, Clone)] 
enum Rank{
	One, Two, Three, Four, Five, Six, Seven, Eight, 
}

// Position based on Files and Rank
#[derive(Debug)] 
pub struct Posistion {
	rank: Rank, 
	file: File, 
}

// Colour of pieces
#[derive(Clone, Debug, PartialEq)] 
enum Colour{
	Black,	White
}

// enum piece 
// color, type piece, life, placement
#[derive(Debug)] 
pub struct Piece {
	posision: Posistion,
	type_pice: BoardPieces,
	colour: Colour,
}


fn match_number(num: u32) -> File {
	match num {
		0 => File::A,
		1 => File::B,
		2 => File::C, 
		3 => File::D, 
		4 => File::E, 
		5 => File::F, 
		6 => File::G, 
		7 => File::H, 
		_ => File::A,
	}
}

// Fills an input array with pieces in correct position and colour. 
fn fill_array_with_pieces(pice_colour: Colour, mut array: MutexGuard<[Option<Piece>;16]>){

	// Choses the correct placement for pieces based on colour
	let pawn_rank: Rank;
	let rest_pieces: Rank;
	if pice_colour == Colour::White{
		pawn_rank = Rank::Two;
		rest_pieces = Rank::One; 
	} else if pice_colour == Colour::Black{
		pawn_rank = Rank::Seven;
		rest_pieces = Rank::Eight;
	}else {
		pawn_rank = Rank::Five;
		rest_pieces = Rank::Four;
	}

	let array_pieces: &mut [Option<Piece>; 16] = array.deref_mut();

	// Fills the array with eight pawns
	for i in 0..8{ 
		array_pieces[i] = Some(Piece{ 
			colour: pice_colour.clone() ,type_pice: BoardPieces::Pawn, posision: Posistion{rank: pawn_rank.clone(), file: match_number(i.try_into().unwrap())}	});
	}

	// Rook
	array_pieces[8] = Some(Piece{
		colour: pice_colour.clone(), 		type_pice: BoardPieces::Rook,		posision: Posistion{rank: rest_pieces.clone(), file: File::A}, });
	// Rook
	array_pieces[9] = Some(Piece{
		colour: pice_colour.clone(), 		type_pice: BoardPieces::Rook,		posision: Posistion{rank: rest_pieces.clone(), file: File::H}, }); 
	// Bishop
	array_pieces[10] = Some(Piece{
		colour: pice_colour.clone(), 		type_pice: BoardPieces::Bishop,		posision: Posistion{rank:  rest_pieces.clone(), file: File::C}, }); 
	// Bishop
	array_pieces[11] = Some(Piece{
		colour: pice_colour.clone(), 		type_pice: BoardPieces::Bishop,		posision: Posistion{rank:  rest_pieces.clone(), file: File::F}, });
	// Knight
	array_pieces[12] = Some(Piece{
		colour: pice_colour.clone(),		type_pice: BoardPieces::Knight,		posision: Posistion{rank:  rest_pieces.clone(), file: File::B}, });
	// Knight
	array_pieces[13] = Some(Piece{
		colour: pice_colour.clone(),		type_pice: BoardPieces::Knight,		posision: Posistion{rank:  rest_pieces.clone(), file: File::G}, });
	// Queen
	array_pieces[14] = Some(Piece{
		colour: pice_colour.clone(), 		type_pice: BoardPieces::Queen,		posision: Posistion{rank:  rest_pieces.clone(), file: File::D}, });
	// King 
	array_pieces[15] = Some(Piece{ 
		colour: pice_colour.clone(),		type_pice: BoardPieces::King,		posision: Posistion{rank:  rest_pieces.clone(), file: File::E}, });
}



pub fn set_up_board() {
	fill_array_with_pieces(Colour::White, WHITE_PIECES.lock().unwrap());
	fill_array_with_pieces(Colour::Black, BLACK_PIECES.lock().unwrap());
}