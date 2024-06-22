use std::{ops::{Deref, DerefMut}, sync::{Mutex, MutexGuard}};
mod test; 


#[derive(Debug)] 
// enum a posibility of pieces. 
enum BoardPieces {
	Pawn, 
	Bishop,
	Knight,
	Rook, 
	Queen,
	King
}

#[derive(Debug)] 
enum File {
	A, B, C, D, E, F, G, H,
}
#[derive(Debug, Clone)] 
enum Rank{
	One, Two, Three, Four, Five, Six, Seven, Eight, 
}

#[derive(Debug)] 
struct PosistionHolder {
	rank: Rank, 
	file: File, 
}

#[derive(Clone, Debug, PartialEq)] 
enum Colour{
	Black,	White
}

// enum piece 
// color, type piece, life, placement
#[derive(Debug)] 
struct Pice {
	posision: PosistionHolder,
	type_pice: BoardPieces,
	colour: Colour,
}
const ARRAY_REPEAT_VALUE: Option<Pice> = None;
static WHITE_PIECES: Mutex<[Option<Pice>; 16]> = Mutex::new([ARRAY_REPEAT_VALUE; 16]);
static BLACK_PIECES: Mutex<[Option<Pice>; 16]> = Mutex::new([ARRAY_REPEAT_VALUE; 16]);
static mut BlackPieces: [Option<Pice>; 16] = [ARRAY_REPEAT_VALUE; 16];



fn matchnumber(num: u32) -> File {
	match num {
		0 => (File::A),
		1 => (File::B),
		2 => (File::C), 
		3 => (File::D), 
		4 => (File::E), 
		5 => (File::F), 
		6 => (File::G), 
		7 => (File::H), 
		_ => File::A,
	}
}

// array with pieces, color and placement and if alive or not,
fn MakeArrayWithPieces(pice_colour: Colour, mut array: MutexGuard<[Option<Pice>;16]>){
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


	// Pawns 
	let array_pieces: &mut [Option<Pice>; 16] = array.deref_mut();

	for i in (0..8){ 
		array_pieces[i] = Some(Pice{ 
				colour: pice_colour.clone() ,type_pice: BoardPieces::Pawn, posision: PosistionHolder{rank: pawn_rank.clone(), file: matchnumber(i.try_into().unwrap())}	});
	}

	// Rook left
	array_pieces[8] = Some(Pice{
		colour: pice_colour.clone(), 		type_pice: BoardPieces::Rook,		posision: PosistionHolder{rank: rest_pieces.clone(), file: File::A}, });
	// Rook right
	array_pieces[9] = Some(Pice{
		colour: pice_colour.clone(), 		type_pice: BoardPieces::Rook,		posision: PosistionHolder{rank: rest_pieces.clone(), file: File::H}, }); 
	// Bishop Left
	array_pieces[10] = Some(Pice{
		colour: pice_colour.clone(), 		type_pice: BoardPieces::Bishop,		posision: PosistionHolder{rank:  rest_pieces.clone(), file: File::C}, }); 
	// Bishop right
	array_pieces[11] = Some(Pice{
		colour: pice_colour.clone(), 		type_pice: BoardPieces::Bishop,		posision: PosistionHolder{rank:  rest_pieces.clone(), file: File::F}, });
	// Knight right
	array_pieces[12] = Some(Pice{
		colour: pice_colour.clone(),		type_pice: BoardPieces::Knight,		posision: PosistionHolder{rank:  rest_pieces.clone(), file: File::B}, });
	// Knight left
	array_pieces[13] = Some(Pice{
		colour: pice_colour.clone(),		type_pice: BoardPieces::Knight,		posision: PosistionHolder{rank:  rest_pieces.clone(), file: File::G}, });
	// Queen
	array_pieces[14] = Some(Pice{
		colour: pice_colour.clone(), 		type_pice: BoardPieces::Queen,		posision: PosistionHolder{rank:  rest_pieces.clone(), file: File::D}, });
	// King 
	array_pieces[15] = Some(Pice{ 
		colour: pice_colour.clone(),		type_pice: BoardPieces::King,		posision: PosistionHolder{rank:  rest_pieces.clone(), file: File::E}, });
}

// move the posision of a piece from a place to another
fn movePieces(){

}


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

	MakeArrayWithPieces(Colour::White, WHITE_PIECES.lock().unwrap());
	let white_pieces = WHITE_PIECES.lock().unwrap();

    // Print the values stored in WHITE_PIECES
    for (index, piece) in white_pieces.iter().enumerate() {
        if let Some(piece) = piece {
            println!("Index {}: {:?}", index, piece);
        } else {
            println!("Index {}: None", index);
        }
    }

	
	MakeArrayWithPieces(Colour::Black, BLACK_PIECES.lock().unwrap());
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
