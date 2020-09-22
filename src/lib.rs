use std::fmt;
use std::option;


/**
 * Chess template. Complementary to task-5.
 * Author: Viola Söderlund <violaso@kth.se>
 */

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Check,
    GameOver
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour {
    White, Black
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    King, Queen, Bishop, Knight, Rook, Pawn
}

pub struct Piece{
    pieceType: PieceType,
    colour: Colour,
    rank: i8,
    file: i8,
    taken: bool,
    hasMoved: bool
//possible_moves vec
}

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    pieces: Vec<Piece>,
    ranks: Vec<u8>,
    files: Vec<u8>,
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            pieces: Vec::new(),
            ranks: (1..=8).map(|v| v ).collect::<Vec<u8>>(),
            files: (1..=8).map(|v| v ).collect::<Vec<u8>>(),
        }
    }
    //makes a new piece of a given type at a given position
    pub fn make_piece(colour: Colour, pieceType: PieceType, rank: i8, file: i8) -> Piece {
        Piece {
            colour,
            pieceType,
            rank,
            file,
            taken: false,
            hasMoved: false,
        }
    }

    /// If the current game state is InProgress and the move is legal, 
    /// move a piece and return the resulting state of the game.
    pub fn make_move(&mut self, _from: String, _to: String) -> Option<GameState> {
        None
    }

    /// Set the piece type that a peasant becames following a promotion.
    pub fn set_promotion(&mut self, _piece: String) -> () {
        ()
    }

    /// Get the current game state.
    pub fn get_game_state(&self) -> GameState {
        self.state
    }
}    
    /// If a piece is standing on the given tile, return all possible 
    /// new positions of that piece. Don't forget to the rules for check. 
    /// 
    /// (optional) Don't forget to include en passent and castling.
    //pub fn get_possible_moves(&self, _postion: String) -> Option<Vec<String>> {
    //  None
    //}


    // [0 1 0 -1]
    // [1 0 -1 0]
    // while not all directions stopped do each direction, much better


impl Piece{
    pub fn get_possible_moves(&self) -> Vec<(i8, i8)>{
        // storing moves in vec, can maybe use collect() for this?
        let moves = Vec::new();
        if self.pieceType == PieceType::Pawn{
            
            if self.colour == Colour::White && positionCheck(self.rank + 1, self.file).is_none(){
                if self.hasMoved == false && positionCheck(self.rank + 2, self.file).is_none(){
                    moves.push((self.rank + 2, self.file))
                }
                moves.push((self.rank + 1, self.file))
            }
            if self.colour == Colour::Black && positionCheck(self.rank - 1, self.file).is_none(){
                if self.hasMoved == false && positionCheck(self.rank - 2, self.file).is_none(){
                    moves.push ((self.rank - 2, self.file))
                }
                moves.push((self.rank - 1, self.file))
            }
        }
        if self.pieceType == PieceType::Rook{
            
            // for each direction, offset 1 and then go outwards
            // [0, 1, 0, -1] rank offset
            // [1, 0, -1, 0] file offset

            let offset: Vec<(i8, i8)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            for direction in offset {
                for scalar in 1..8 {
                    if positionCheck(self.rank + scalar * direction.0, self.file + scalar * direction.1).is_some(){
                        if positionCheck(self.rank + scalar * direction.0, self.file + scalar * direction.1).unwrap().colour != self.colour{
                            moves.push((self.rank + scalar * direction.0, self.file + scalar * direction.1));
                        }
                    break
                    }
                    moves.push((self.rank + scalar * direction.0, self.file + scalar * direction.1))
                }
            }
        }
 
        if self.pieceType == PieceType::Bishop{
            // rook code with different offset
            let offset: Vec<(i8, i8)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
            for direction in offset {
                for scalar in 1..8 {
                    if positionCheck(self.rank + scalar * direction.0, self.file + scalar * direction.1).is_some(){
                        if positionCheck(self.rank + scalar * direction.0, self.file + scalar * direction.1).unwrap().colour != self.colour{
                            moves.push((self.rank + scalar * direction.0, self.file + scalar * direction.1));
                        }
                    break
                    }
                    moves.push((self.rank + scalar * direction.0, self.file + scalar * direction.1))
                }
            }
        } 

        if self.pieceType == PieceType::Knight{
            let offset = Vec<(i8, i8)> =
            vec![(1, 2), (2, 1), (-1, 2), (2, -1), (1, -2), (-2, 1), (-1, -2), (-2, -1)];    
            
        }
        if self.pieceType == PieceType::King{
            
        }
        if self.pieceType == PieceType::Queen{
            
        }
        return moves    
}
pub fn positionCheck(_rank, _file) -> Option<Piece>{
    unimplemented!()
}

pub fn draw(){ 
    for rank in 1..=8 {
        let mut _line: String;
        for file in 1..=8{
            if positionCheck(rank, file).is_some(){
                    _line +=  " " + positionCheck(rank, file).unwrap().pieceType + " " ;
            }
            else {_line += " . "}
        format! _line
        }
    }
}


/// Implement print routine for Game.
/// 
/// Output example:
/// |:----------------------:|
/// | R  Kn B  K  Q  B  Kn R |
/// | P  P  P  P  P  P  P  P |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | *  *  *  *  *  *  *  * |
/// | P  P  P  P  P  P  P  P |
/// | R  Kn B  K  Q  B  Kn R |
/// |:----------------------:|
impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        /* build board representation string */
        
        write!(f, "")
    }
}

// --------------------------
// ######### TESTS ##########
// --------------------------

#[cfg(test)]
mod tests {
    use super::Game;
    use super::GameState;

    // check test framework
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // example test
    // check that game state is in progress after initialisation
    #[test]
    fn game_in_progress_after_init() {

        let game = Game::new();

        println!("{:?}", game);

        assert_eq!(game.get_game_state(), GameState::InProgress);
    }
}