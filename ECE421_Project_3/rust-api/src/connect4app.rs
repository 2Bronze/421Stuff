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

// Enum for Player
pub enum Player {
    Red,
    Yellow,
}

// Struct representing connect4
pub struct Connect4 {
    pub board: [[u8; 7]; 6],
    pub gamestate: Gamestate,
    pub gametype: Gametype, 
    pub curr_player: Player,
}

// Implement methods for games
impl Connect4 {

        // Create new game 
        pub fn new(gametype: Gametype) -> Self{
            let board = [[0;7]; 6];
            return Connect4{
                board: board,
                gamestate: Gamestate::InProgress,
                gametype: gametype,
                curr_player: Player::Red
            }
        }

        // Do a move 
        pub fn game_move(mut self, column: usize) -> Self{
            if column > 6  || column < 0 {
                println!("Column number not in bounds");
                return self;
            } 
            else {
                if self.board[0][column] != 0 {
                    println!("Column is full");
                    return self;
                }
                else {
                    let mut row_num = 5;
                    for _ in 0..5 {
                        if self.board[row_num][column] == 0 {
                            break;
                        }
                        else {
                            row_num -= 1;
                        }
                    }
                    match &self.curr_player {
                        Player::Red => {
                            self.board[row_num][column] = 1;
                            self = self.check_winner(column, row_num);
                            self.curr_player = Player::Yellow;
                            return self;
                        },
                        Player::Yellow => {
                            self.board[row_num][column] = 2;
                            self = self.check_winner(column, row_num);
                            self.curr_player = Player::Red;
                            return self;
                        },
                    }

                }
                
            }
        }

        // Check who wins
        pub fn check_winner(mut self, column: usize, row: usize) -> Self{
            let num: u8;
            match &self.curr_player {
                Player::Red => num = 1,
                Player::Yellow => num = 2,
            }
            // Vertical win check 
            let mut incr = 0;
            for i in 1..6 {
                if self.board[i][column] != 0 && self.board[i][column] == self.board[i-1][column] {
                    incr += 1;
                }
                else {
                    incr = 0;
                }
                if incr == 3 {
                    println!("Player {} wins", num);
                    self.gamestate = Gamestate::Gameover;
                    return self;
                    
                }
            }
            // Horizontal Win Check
            incr = 0;
            for i in 0..7 {
                if self.board[row][i] != 0 && self.board[row][i] == self.board[row][i-1] {
                    incr += 1;
                }
                else {
                    incr = 0;
                }
                if incr == 3 {
                    println!("Player {} wins", num);
                    self.gamestate = Gamestate::Gameover;
                    return self;
                }
            }
            // NEED TO ADD A DIAGONAL CHECK, NOT SURE HOW THATS GOING TO WORK 
            println!("No winner");
            return self;
        }       
}