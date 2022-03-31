
// Enumeration for keeping track of gamestate
pub enum Gamestate {
    InProgress,
    Gameover,
}

// Gametype, Human or Computer
pub enum Gametype {
    Human,
    Computer,
}

// Struct representing connect4
pub struct Connect4 {
    pub board: [[u8; 6]; 7],
    pub gamestate: Gamestate,
    pub gametype: Gametype, 
}

// Implement methods for games
impl Connect4 {

        // Create new game 
        pub fn new(gametype: Gametype) -> Self{
            let mut board;
            for i in 0..7 {
                for j in 0..6 {
                    board[i][j] = 0;
                }
            }
            return Connect4{
                board: board,
                gamestate: Gamestate::InProgress,
                gametype: gametype
            }
        }

        
}