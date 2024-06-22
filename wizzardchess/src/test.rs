


#[cfg(test)]
mod tests {
    use crate::{MakeArrayWithPieces, WHITE_PIECES};

    #[test]
    fn work(){
        MakeArrayWithPieces();
        println!("{:?}", WHITE_PIECES);
        assert!(true);
    }

}