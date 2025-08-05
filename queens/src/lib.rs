#[derive(Debug)]
pub struct ChessPosition {
    pub rank: i32,
    pub file: i32,
}

#[derive(Debug)]
pub struct Queen {
    pub position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >=0 && rank <= 7 && file >=0 && file <= 7 {
            return Some(Self {rank,file})
        }
        None

    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {

        Self{position}


    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let y1 = self.position.rank;
        let x1 = self.position.file;
        let y2 = other.position.rank;
        let x2 = other.position.file;
        if x1 == x2 || y1 == y2 {
            return true
        }
        if (x2-x1).abs() == (y2-y1).abs(){
            return  true;
        } 
        return false

    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_is_valid() {
        assert!(ChessPosition::new(2, 4).is_some());
        assert!(ChessPosition::new(-1, 2).is_none());
        assert!(ChessPosition::new(8, 2).is_none());
        assert!(ChessPosition::new(5, -1).is_none());
        assert!(ChessPosition::new(5, 8).is_none());
    }

    #[test]
    fn test_queen_valid_position() {
        Queen::new(ChessPosition::new(2, 4).unwrap());
    }

    #[test]
    fn test_can_not_attack() {
        let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
        let black_queen = Queen::new(ChessPosition::new(6, 6).unwrap());
        let white_queen2 = Queen::new(ChessPosition::new(1, 2).unwrap());
        let black_queen2 = Queen::new(ChessPosition::new(0, 4).unwrap());
        let white_queen3 = Queen::new(ChessPosition::new(5, 3).unwrap());
        let black_queen3 = Queen::new(ChessPosition::new(0, 6).unwrap());
        let white_queen4 = Queen::new(ChessPosition::new(3, 7).unwrap());
        let black_queen4 = Queen::new(ChessPosition::new(2, 0).unwrap());

        assert!(!white_queen.can_attack(&black_queen));
        assert!(!white_queen2.can_attack(&black_queen2));
        assert!(!white_queen3.can_attack(&black_queen3));
        assert!(!white_queen4.can_attack(&black_queen4));
    }

    #[test]
    fn test_same_rank() {
        let white_queen = Queen::new(ChessPosition::new(2, 4).unwrap());
        let black_queen = Queen::new(ChessPosition::new(2, 6).unwrap());
        let white_queen2 = Queen::new(ChessPosition::new(1, 2).unwrap());
        let black_queen2 = Queen::new(ChessPosition::new(1, 6).unwrap());
        let white_queen3 = Queen::new(ChessPosition::new(4, 7).unwrap());
        let black_queen3 = Queen::new(ChessPosition::new(4, 3).unwrap());
        let white_queen4 = Queen::new(ChessPosition::new(5, 3).unwrap());
        let black_queen4 = Queen::new(ChessPosition::new(5, 1).unwrap());

        assert!(white_queen.can_attack(&black_queen));
        assert!(white_queen2.can_attack(&black_queen2));
        assert!(white_queen3.can_attack(&black_queen3));
        assert!(white_queen4.can_attack(&black_queen4));
    }

    #[test]
    fn test_same_file() {
        let white_queen = Queen::new(ChessPosition::new(4, 5).unwrap());
        let black_queen = Queen::new(ChessPosition::new(3, 5).unwrap());
        let white_queen2 = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen2 = Queen::new(ChessPosition::new(3, 2).unwrap());
        let white_queen3 = Queen::new(ChessPosition::new(2, 6).unwrap());
        let black_queen3 = Queen::new(ChessPosition::new(1, 6).unwrap());
        let white_queen4 = Queen::new(ChessPosition::new(2, 7).unwrap());
        let black_queen4 = Queen::new(ChessPosition::new(5, 7).unwrap());

        assert!(white_queen.can_attack(&black_queen));
        assert!(white_queen2.can_attack(&black_queen2));
        assert!(white_queen3.can_attack(&black_queen3));
        assert!(white_queen4.can_attack(&black_queen4));
    }

    #[test]
    fn test_same_diagonal() {
        let white_queen = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen = Queen::new(ChessPosition::new(0, 4).unwrap());
        let white_queen2 = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen2 = Queen::new(ChessPosition::new(3, 1).unwrap());
        let white_queen3 = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen3 = Queen::new(ChessPosition::new(1, 1).unwrap());
        let white_queen4 = Queen::new(ChessPosition::new(2, 2).unwrap());
        let black_queen4 = Queen::new(ChessPosition::new(5, 5).unwrap());

        assert!(white_queen.can_attack(&black_queen));
        assert!(white_queen2.can_attack(&black_queen2));
        assert!(white_queen3.can_attack(&black_queen3));
        assert!(white_queen4.can_attack(&black_queen4));
    }
}