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
    T,
    O,
}

// Struct representing TOOTOTTO
pub struct Game2 {
    pub board: [[u8; 6]; 4],
    pub gamestate: Gamestate,
    pub gametype: Gametype, 
    pub curr_player: Player,
}

// Implement methods for games
impl Game2 {
        // Create new game 
        pub fn new(gametype: Gametype) -> Self{
            let board = [[0;6]; 4];
            return Game2{
                board: board,
                gamestate: Gamestate::InProgress,
                gametype: gametype,
                curr_player: Player::T
            }
        }

        // Do a move 
        pub fn game_move(mut self, column: usize) -> Self{
            if column > 5  || column < 0 {
                println!("Column number not in bounds");
                return self;
            } 
            else {
                if self.board[0][column] != 0 {
                    println!("Column is full");
                    return self;
                }
                else {
                    let mut row_num = 3;
                    for _ in 0..3 {
                        if self.board[row_num][column] == 0 {
                            break;
                        }
                        else {
                            row_num -= 1;
                        }
                    }
                    match &self.curr_player {
                        Player::T => {
                            self.board[row_num][column] = 1;
                            self = self.check_winner(column, row_num);
                            self.curr_player = Player::O;
                            return self;
                        },
                        Player::O => {
                            self.board[row_num][column] = 2;
                            self = self.check_winner(column, row_num);
                            self.curr_player = Player::T;
                            return self;
                        },
                    }

                }
                
            }
        }

        // Check who wins
        pub fn check_winner(mut self, column: usize, row: usize) -> Self{
            // let mut num = 1;
            // Vertical win check 
            let mut incr = 0;
            let mut i = 0;
            loop {
                if i > 2 {
                    break;
                }
                if (self.board[i][column] != 0 && self.board[i+1][column] !=0) && (self.board[i][column] != self.board[i+1][column]) {
                    if incr == 1 {
                        if self.board[i][column] == self.board[i-1][column] {
                            incr += 1;
                            i+=2;
                        }
                        else {
                            incr = 0;
                            i+=1;
                        }
                    }
                    else {
                        incr += 1;
                        i+=2;
                    }
                    
                }
                else {
                    incr = 0;
                    i += 1;
                }
                if incr == 2 {
                    i -= 2;
                    let mut winner = "0";
                    if (self.board[i][column] == 2) {
                        winner = "T";
                    }
                    else {
                        winner = "O";
                    }
                    println!("Player {} wins", winner);
                    self.gamestate = Gamestate::Gameover;
                    return self;
                    
                }
            }
            // Horizontal Win Check
            incr = 0;
            i = 0;
            loop {
                if i > 4 {
                    break;
                }
                if (self.board[row][i] != 0 && self.board[row][i+1] != 0) && (self.board[row][i] != self.board[row][i+1]) {
                    if incr == 1 {
                        if self.board[row][i] == self.board[row][i-1] {
                            incr += 1;
                            i+=2;
                        }
                        else {
                            incr = 0;
                            i+=1;
                        }
                    }
                    else {
                        incr += 1;
                        i+=2;
                    }
                }
                else {
                    incr = 0;
                    i+=1;
                }
                if incr == 2 {
                    i -= 2;
                    let mut winner = "0";
                    if (self.board[row][i] == 2) {
                        winner = "T";
                    }
                    else {
                        winner = "O";
                    }
                    println!("Player {} wins", winner);
                    self.gamestate = Gamestate::Gameover;
                    return self;
                }
            }
            // Diagonal Check
            let mut temp_row = row;
            let mut temp_col = column;
        
            loop {
                if temp_row == 3 || temp_col == 0 {
                    break;
                }  
                temp_row += 1;
                temp_col -= 1;
            }
            // Now do checking
            loop {
                println!("{}", temp_col);
                println!("{}", temp_row);
                if temp_row < 1 || temp_col > 3 {
                    break;
                }
                if (self.board[temp_row][temp_col] != 0 && self.board[temp_row - 1][temp_col + 1] != 0) && (self.board[temp_row - 1][temp_col + 1] != self.board[temp_row][temp_col]) {
                    if incr == 1 {
                        if self.board[temp_row][temp_col] == self.board[temp_row + 1][temp_col - 1] {
                            incr += 1;
                        }
                        else {
                            incr = 0;
                            temp_row -= 1;
                            temp_col += 1;
                        }
                    }
                    else {
                        incr += 1;
                        temp_row -= 2;
                        temp_col += 2;
                    }
                }
                else {
                    incr = 0;
                    temp_row -= 1;
                    temp_col += 1;
                }
                if incr == 2 {
                    let mut winner = "0";
                    if (self.board[temp_row][temp_col] == 2) {
                        winner = "T";
                    }
                    else {
                        winner = "O";
                    }
                    println!("Player {} wins", winner);
                    return self;
                }

            }
            println!("No winner");
            return self;
        }        
}



