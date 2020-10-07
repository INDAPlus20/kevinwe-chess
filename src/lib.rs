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

#[derive(Clone, Debug, PartialEq)]
pub struct Piece{
    pieceType: String,
    colour: Colour,
    // should've just made rank + file a single data type
    rank: i8,
    file: i8,
    taken: bool,
    has_moved: bool
//possible_moves vec
}

#[derive(Clone, PartialEq)]
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
    pub fn make_piece(colour: Colour, pieceType: String, rank: i8, file: i8) -> Piece {
        Piece {
            colour,
            pieceType,
            rank,
            file,
            taken: false,
            has_moved: false,
        }
    }
    pub fn position_check(&self, _rank: i8, _file: i8) -> Option<Piece>{
        for piece in self.pieces.clone(){
            if piece.rank == _rank && piece.file == _file{
                return Some(piece);
            }
        }
        return None;
    }
    pub fn get_possible_moves(&self, piece: Piece) -> Vec<(i8, i8)>{
        let mut moves = Vec::new();
        
        if piece.pieceType == "P".to_string() {
            // whites go up
            if piece.colour == Colour::White && self.position_check(piece.rank + 1, piece.file).is_none(){
                if piece.has_moved == false && self.position_check(piece.rank + 2, piece.file).is_none(){
                    moves.push((piece.rank + 2, piece.file))
                }
                moves.push((piece.rank + 1, piece.file))
            }
            // blacks go down
            if piece.colour == Colour::Black && self.position_check(piece.rank - 1, piece.file).is_none(){
                if piece.has_moved == false && self.position_check(piece.rank - 2, piece.file).is_none(){
                    moves.push ((piece.rank - 2, piece.file))
                }
                moves.push((piece.rank - 1, piece.file))
            }
            // we can move diagonally if we can take
            if piece.colour == Colour::White && self.position_check(piece.rank + 1, piece.file + 1).is_some() && self.position_check(piece.rank + 1, piece.file + 1).unwrap().colour == Colour::Black{
                moves.push((piece.file + 1, piece.file + 1))
            }
            if piece.colour == Colour::White && self.position_check(piece.rank + 1, piece.file - 1).is_some() && self.position_check(piece.rank + 1, piece.file - 1).unwrap().colour == Colour::Black{
                moves.push((piece.file + 1, piece.file - 1))
            }
            if piece.colour == Colour::Black && self.position_check(piece.rank - 1, piece.file + 1).is_some() && self.position_check(piece.rank - 1, piece.file + 1).unwrap().colour == Colour::White{
                moves.push((piece.file - 1, piece.file + 1))
            }
            if piece.colour == Colour::Black && self.position_check(piece.rank - 1, piece.file + 1).is_some() && self.position_check(piece.rank - 1, piece.file - 1).unwrap().colour == Colour::White{
                moves.push((piece.file - 1, piece.file - 1))
            }
        }
        if piece.pieceType == "R".to_string() {
            // TODO: add boundary checks!


            // make offset to loop through different directions
            let offset: Vec<(i8, i8)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            // for each direction
            for direction in offset {
                //for each distance
                for scalar in 1..8 {
                    //check if something's there
                    if self.position_check(piece.rank + scalar * direction.0, piece.file + scalar * direction.1).is_some() {
                        //if it's not the same colour, we can move there
                        if self.position_check(piece.rank + scalar * direction.0, piece.file + scalar * direction.1).unwrap().colour != piece.colour{
                            moves.push((piece.rank + scalar * direction.0, piece.file + scalar * direction.1));
                        }
                        //if not, we can't move there and we can't move past it
                        break
                    }
                    //if nothing's there we can move there
                    moves.push((piece.rank + scalar * direction.0, piece.file + scalar * direction.1))
                }
            }
        }
 
        if piece.pieceType == "B".to_string() {
            // rook code with different offset -> different movement direction
            let offset: Vec<(i8, i8)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
            for direction in offset {
                for scalar in 1..8 {
                    if self.position_check(piece.rank + scalar * direction.0, piece.file + scalar * direction.1).is_some(){
                        if self.position_check(piece.rank + scalar * direction.0, piece.file + scalar * direction.1).unwrap().colour != piece.colour{
                            moves.push((piece.rank + scalar * direction.0, piece.file + scalar * direction.1));
                        }
                        break
                    }
                    moves.push((piece.rank + scalar * direction.0, piece.file + scalar * direction.1))
                }
            }
        } 
    
        if piece.pieceType == "Kn".to_string() {
            // ponies can move in 8 different directions, no scalar needed
            let offset: Vec<(i8, i8)> =
            vec![(1, 2), (2, 1), (-1, 2), (2, -1), (1, -2), (-2, 1), (-1, -2), (-2, -1)];
            for direction in offset{
                //as long as no friendly piece, it can move there
                //check if something's there
                if self.position_check(piece.rank + direction.0, piece.file + direction.1).is_some(){
                    //if it's not the same colour, we can move there
                    if self.position_check(piece.rank + direction.0, piece.file + direction.1).unwrap().colour != piece.colour{
                        moves.push((piece.rank + direction.0, piece.file + direction.1));
                    }
                }
                //if nothing is there we can also move there
                if self.position_check(piece.rank + direction.0, piece.file + direction.1).is_none(){
                    moves.push((piece.rank + direction.0, piece.file + direction.1));
                }
                
            }    
        }
        if piece.pieceType == "K".to_string() {
            //kings move like knights except not as far
            let offset: Vec<(i8, i8)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
            for direction in offset{
                if self.position_check(piece.rank + direction.0, piece.file + direction.1).is_some(){
                    //if it's not the same colour, we can move there
                    if self.position_check(piece.rank + direction.0, piece.file + direction.1).unwrap().colour != piece.colour{
                        moves.push((piece.rank + direction.0, piece.file + direction.1));
                    }
                //if nothing is there we can also move there
                if self.position_check(piece.rank + direction.0, piece.file + direction.1).is_none(){
                    moves.push((piece.rank + direction.0, piece.file + direction.1));
                    }
                }
            }
        }
        if piece.pieceType == "Q".to_string() {
            //queens move like rook and bishop combined
            let offset: Vec<(i8, i8)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
            for direction in offset {
                for scalar in 1..8 {
                    if self.position_check(piece.rank + scalar * direction.0, piece.file + scalar * direction.1).is_some(){
                        if self.position_check(piece.rank + scalar * direction.0, piece.file + scalar * direction.1).unwrap().colour != piece.colour{
                            moves.push((piece.rank + scalar * direction.0, piece.file + scalar * direction.1));
                        }
                        break
                    }
                    moves.push((piece.rank + scalar * direction.0, piece.file + scalar * direction.1))
                }
            }
        }
        return moves    
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


pub fn draw(game: Game){ 
    //for each line in board, we make a line
    for rank in 1..=8 {
        let mut _line: String = "".to_string();
        //for each square we check what's there and add it to the line
        for file in 1..=8{
            if game.position_check(rank, file).is_some(){
                    _line += " ";
                    //TODO: fix, i still don't understand how strings work in this language
                    //_line += game.position_check(rank, file).unwrap().pieceType;
                    _line += " ";
            }
            else {_line += " . "}
        println! ("{}",_line)
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
    use super::Piece;
    use super::Colour;

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

        let mut rook = Game::make_piece(Colour::White, "R".to_string(), 4, 4);
        game.pieces.push(rook);
        println! ("{:?}", Game::get_possible_moves(&game, game.pieces[0].clone()));
    }
}