use std::sync::Mutex;
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
#[derive(Debug)] 
enum Rank{
	One, Two, Three, Four, Five, Six, Seven, Eight, 
}

#[derive(Debug)] 
struct PosistionHolder {
	rank: Rank, 
	file: File, 
}

#[derive(Debug)] 
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
fn MakeArrayWithPieces(){
	// Pawns 
	let mut white_pieces = WHITE_PIECES.lock().unwrap();
	for i in (0..8){ 
		
		white_pieces[i] = Some(Pice{ 
				colour: Colour::White,type_pice: BoardPieces::Pawn,posision: PosistionHolder{rank: Rank::Two, file: matchnumber(i.try_into().unwrap())}	});
	}

	// Rook left
	white_pieces[8] = Some(Pice{
		colour: Colour::White, 		type_pice: BoardPieces::Rook,		posision: PosistionHolder{rank: Rank::One, file: File::A}, });
	// Rook right
	white_pieces[9] = Some(Pice{
		colour: Colour::White, 		type_pice: BoardPieces::Rook,		posision: PosistionHolder{rank: Rank::One, file: File::H}, }); 
	// Bishop Left
	white_pieces[10] = Some(Pice{
		colour: Colour::White, 		type_pice: BoardPieces::Bishop,		posision: PosistionHolder{rank: Rank::One, file: File::C}, }); 
	// Bishop right
	white_pieces[11] = Some(Pice{
		colour: Colour::White, 		type_pice: BoardPieces::Bishop,		posision: PosistionHolder{rank: Rank::One, file: File::F}, });
	// Knight right
	white_pieces[12] = Some(Pice{
		colour: Colour::White,		type_pice: BoardPieces::Knight,		posision: PosistionHolder{rank: Rank::One, file: File::B}, });
	// Knight left
	white_pieces[13] = Some(Pice{
		colour: Colour::White,		type_pice: BoardPieces::Knight,		posision: PosistionHolder{rank: Rank::One, file: File::G}, });
	// Queen
	white_pieces[14] = Some(Pice{
		colour: Colour::White, 		type_pice: BoardPieces::Queen,		posision: PosistionHolder{rank: Rank::One, file: File::D}, });
	// King 
	white_pieces[15] = Some(Pice{ 
		colour: Colour::White,		type_pice: BoardPieces::King,		posision: PosistionHolder{rank: Rank::One, file: File::E}, });
}

// void AssignPlaceBoardpiecesArray (){
	
// }


// chessboard array whit the placement of the bricks. 


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");
	MakeArrayWithPieces();
	let white_pieces = WHITE_PIECES.lock().unwrap();

    // Print the values stored in WHITE_PIECES
    for (index, piece) in white_pieces.iter().enumerate() {
        if let Some(piece) = piece {
            println!("Index {}: {:?}", index, piece);
        } else {
            println!("Index {}: None", index);
        }
    }

}
