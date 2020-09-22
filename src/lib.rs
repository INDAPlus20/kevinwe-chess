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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece{
    pieceType: PieceType,
    colour: Colour,
    rank: i8,
    file: i8,
    taken: bool,
    has_moved: bool
//possible_moves vec
}

pub struct Game {
    /* save board, active colour, ... */
    state: GameState,
    pieces: Vec<Piece>,
}

impl Game {
    /// Initialises a new board with pieces.
    pub fn new() -> Game {
        Game {
            /* initialise board, set active colour to white, ... */
            state: GameState::InProgress,
            pieces: Vec::new(),
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
            has_moved: false,
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

pub fn position_check(_rank: i8, _file: i8) -> Option<Piece>{
    unimplemented!()
}

impl Piece{
    //given a piece, returns vector of possible move targets
    pub fn get_possible_moves(self) -> Vec<(i8, i8)>{
        let mut moves = Vec::new();
        
        if self.pieceType == PieceType::Pawn {
            
            if self.colour == Colour::White && position_check(self.rank + 1, self.file).is_none(){
                if self.has_moved == false && position_check(self.rank + 2, self.file).is_none(){
                    moves.push((self.rank + 2, self.file))
                }
                moves.push((self.rank + 1, self.file))
            }
            if self.colour == Colour::Black && position_check(self.rank - 1, self.file).is_none(){
                if self.has_moved == false && position_check(self.rank - 2, self.file).is_none(){
                    moves.push ((self.rank - 2, self.file))
                }
                moves.push((self.rank - 1, self.file))
            }
            if self.colour == Colour::White && position_check(self.rank + 1, self.file + 1).is_some(){
                moves.push((self.file + 1, self.file + 1))
            }
            if self.colour == Colour::White && position_check(self.rank + 1, self.file + 1).is_some(){
                moves.push((self.file + 1, self.file - 1))
            }
            if self.colour == Colour::Black && position_check(self.rank + 1, self.file + 1).is_some(){
                moves.push((self.file - 1, self.file + 1))
            }
            if self.colour == Colour::Black && position_check(self.rank + 1, self.file + 1).is_some(){
                moves.push((self.file - 1, self.file - 1))
            }
        }
        if self.pieceType == PieceType::Rook {
            
            // make offset to loop through different directions

            let offset: Vec<(i8, i8)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            // for each direction
            for direction in offset {
                //for each distance
                for scalar in 1..8 {
                    //check if something's there
                    if position_check(self.rank + scalar * direction.0, self.file + scalar * direction.1).is_some(){
                        //if it's the same colour, we can move there
                        if position_check(self.rank + scalar * direction.0, self.file + scalar * direction.1).unwrap().colour != self.colour{
                            moves.push((self.rank + scalar * direction.0, self.file + scalar * direction.1));
                        }
                        //if not, we can't move there and we can't move past it
                        break
                    }
                    //if nothing's there we can move there
                    moves.push((self.rank + scalar * direction.0, self.file + scalar * direction.1))
                }
            }
        }
 
        if self.pieceType == PieceType::Bishop {
            // rook code with different offset -> different movement direction
            let offset: Vec<(i8, i8)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
            for direction in offset {
                for scalar in 1..8 {
                    if position_check(self.rank + scalar * direction.0, self.file + scalar * direction.1).is_some(){
                        if position_check(self.rank + scalar * direction.0, self.file + scalar * direction.1).unwrap().colour != self.colour{
                            moves.push((self.rank + scalar * direction.0, self.file + scalar * direction.1));
                        }
                        break
                    }
                    moves.push((self.rank + scalar * direction.0, self.file + scalar * direction.1))
                }
            }
        } 
    
        if self.pieceType == PieceType::Knight {
            // ponies can move in 8 different directions, no scalar needed
            let offset: Vec<(i8, i8)> =
            vec![(1, 2), (2, 1), (-1, 2), (2, -1), (1, -2), (-2, 1), (-1, -2), (-2, -1)];
            for direction in offset{
                //as long as no friendly piece, it can move there
                //check if something's there
                if position_check(self.rank + direction.0, self.file + direction.1).is_some(){
                    //if it's not the same colour, we can move there
                    if position_check(self.rank + direction.0, self.file + direction.1).unwrap().colour != self.colour{
                        moves.push((self.rank + direction.0, self.file + direction.1));
                    }
                }
                //if nothing is there we can also move there
                if position_check(self.rank + direction.0, self.file + direction.1).is_none(){
                    moves.push((self.rank + direction.0, self.file + direction.1));
                }
                
            }    
        }
        if self.pieceType == PieceType::King {
            //kings move like knights except not as far
            let offset: Vec<(i8, i8)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
            for direction in offset{
                if position_check(self.rank + direction.0, self.file + direction.1).is_some(){
                    //if it's not the same colour, we can move there
                    if position_check(self.rank + direction.0, self.file + direction.1).unwrap().colour != self.colour{
                        moves.push((self.rank + direction.0, self.file + direction.1));
                    }
                //if nothing is there we can also move there
                if position_check(self.rank + direction.0, self.file + direction.1).is_none(){
                    moves.push((self.rank + direction.0, self.file + direction.1));
                    }
                }
            }
        }
        if self.pieceType == PieceType::Queen {
            //queens move like rook and bishop combined
            let offset: Vec<(i8, i8)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
            for direction in offset {
                for scalar in 1..8 {
                    if position_check(self.rank + scalar * direction.0, self.file + scalar * direction.1).is_some(){
                        if position_check(self.rank + scalar * direction.0, self.file + scalar * direction.1).unwrap().colour != self.colour{
                            moves.push((self.rank + scalar * direction.0, self.file + scalar * direction.1));
                        }
                        break
                    }
                    moves.push((self.rank + scalar * direction.0, self.file + scalar * direction.1))
                }
            }
        }
        return moves    
    }
}


//pub fn draw(){ 
    //for rank in 1..=8 {
      //  let mut _line: String;
        //for file in 1..=8{
          //  if position_check(rank, file).is_some(){
            //        _line +=  " " + position_check(rank, file).unwrap().pieceType + " " ;
            //}
            //else {_line += " . "}
        //println! ("{}",_line)
        //}
    //}
//}


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
    use super::Piece;
    use super::Colour;
    use super::PieceType;

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
    #[test]
    fn rook_movement(){

        let mut game = Game::new();

        let mut rook = Game::make_piece(Colour::White, PieceType::Rook, 4, 4);
        game.pieces.push(rook);
        println! ("{:?}", Piece::get_possible_moves(game.pieces[0]));
    }
}