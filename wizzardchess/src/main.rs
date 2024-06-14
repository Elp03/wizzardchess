


// enum a posibility of pieces. 
enum BoardPieces {
	pawn, 
	bishop,
	knight,
	rook, 
	queen,
	king
}


struct PosistionHolder {
	row: u8, 
	colum: u8, 
}

// enum piece 
// color, type piece, life, placement
struct Pice {
	posision: posistion_holder,
	colour: board_pieces,
}

static mut WhitePieces: [Option<Pice>; 16] = [None; 16];

// array with pieces, color and placement and if alive or not,
void MakeArrayWithPieces(){
	for i in (1..8){
		
	}
}

void AssignPlaceBoardpiecesArray (){
	
}


// chessboard array whit the placement of the bricks. 


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");


}
