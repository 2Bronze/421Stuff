use yew::prelude::*;
use yew::html::Scope;
use rand::prelude::*;


// Enumeration for keeping track of gamestate
enum Gamestate {
    InProgress,
    Gameover,
}

// Gametype, Human or Computer
pub enum Gametype {
    Human,
    ComputerEasy,
    ComputerHard,
}

pub enum GameName {
    Connect4,
    TOOTOTTO,
}
pub enum ViewMode {
    Default,
    ColorBlind
}

// Enum for Player
pub enum Player {
    Player1,
    Player2,
}

enum Msg{
    ColumnZero,
    ColumnOne,
    ColumnTwo,
    ColumnThree,
    ColumnFour,
    ColumnFive,
    ColumnSix,
    Local,
    EasyCPU,
    HardCPU,
    Connect4,
    TootOtto,
    NormalView,
    CBView,
}

struct GameBoardComponent{
    //Game board with 0 representing empty, 1 representing red, 2 representing yellow
    pub conn4_board: [[u8; 7]; 6],
    pub toototto_board: [[u8; 6]; 4],
    pub gamestate: Gamestate,
    pub gametype: Gametype, 
    pub curr_player: Player,
    pub viewmode: ViewMode,
    pub gamename: GameName,
}

impl GameBoardComponent{
    fn easy_bot_connect4(board: [[u8; 7]; 6]) -> u8 {
        let mut rng = thread_rng();
        let mut return_col = rng.gen_range(0..7);
        //check if col is full
        loop {
            if board[5][return_col as usize] == 0 {
                break;
            }
            return_col = rng.gen_range(0..7);
        }
        return_col
    }
    
    fn easy_bot_toototto(board: [[u8; 6]; 4]) -> u8 {
        let mut rng = thread_rng();
        let mut return_col = rng.gen_range(0..6);
        //check if col is full
        loop {
            if board[3][return_col as usize] == 0 {
                break;
            }
            return_col = rng.gen_range(0..6);
        }
        return_col
    }
    
    fn hard_bot_connect4(board: [[u8; 7]; 6]) -> u8 {
        let mut return_col = 255;
        for j in 0..6 {
            for i in 0..7 {
                //Check for block/win
                for player in 1..3 {
                    if board[j][i] == player {
                        //check horizontal (right / left)
                        if i <= 3  {
                            if board[j][i+1] == player && board[j][i+2] == player && board[j][i+3] == 0 {
                                return_col = i as u8 + 3;
                            }
                        }
    
                        if i >= 3 {
                            if board[j][i-1] == player && board[j][i-2] == player && board[j][i-3] == 0 {
                                return_col = i as u8 - 3;
                            }
                        }
    
                        //check vertical (up)
                        if j <= 2 {
                            if board[j+1][i] == player && board[j+2][i] == player && board[j+3][i] == 0 {
                                return_col = i as u8;
                            }
                        }
    
    
                        //check diagonal (up right/ up left)
                        if i <= 3 && j <= 2 {
                            if board[j+1][i+1] == player && board[j+2][i+2] == player && board[j+3][i+3] == 0 {
                                return_col = i as u8 + 3;
                            }
                        }
    
                        if i >= 3 && j <= 2 {
                            if board[j+1][i-1] == player && board[j+2][i-2] == player && board[j+3][i-3] == 0 {
                                return_col = i as u8 - 3;
                            }
                        }
                    }
                }
            }
        }
    
        //No possible block or win found
        if return_col > 7 {
            return_col = GameBoardComponent::easy_bot_connect4(board);
        }
    
        // //check if column is full
        // loop {
        //     if board[5][return_col as usize] == 0 {
        //         break;
        //     }
        //     return_col = easy_bot_connect4();
        // }
        return_col
    }
    
    fn hard_bot_toototto(board: [[u8; 6]; 4]) -> u8 { // T = 1, O == 2, Bot is OTTO player
        let mut return_col = 255;
        for j in 0..4 {
            for i in 0..6 {
                //look for block/win
                for player in 1..3 {
                    if board[j][i] != 0 {
                        //check horizontal (right / left)
                        if i <= 2 {
                            if board[j][i+1] == board[j][i+2] && board[j][i+1] == player && board[j][i+3] == 0 {
                                return_col = i as u8 + 3;
                            }
                        }
    
                        if i >= 3 {
                            if board[j][i-1] == board[j][i-2] && board[j][i+1] == player && board[j][i-3] == 0 {
                                return_col = i as u8 - 3;
                            }
                        }
    
                        //check vertical (up)
                        if j == 0 {
                            if board[j+1][i] == board[j+2][i] && board[j+1][i] == player && board[j+3][i] == 0 {
                                return_col = i as u8;
                            }
                        }
    
                        //check diagonal (up right/ up left)
                        if i <= 2 && j == 0 {
                            if board[j+1][i+1] == board[j+2][i+2] && board[j+1][i+1] == player && board[j+3][i+3] == 0 {
                                return_col = i as u8 + 3;
                            }
                        }
    
                        if i >= 3 && j == 0 {
                            if board[j+1][i-1] == board[j+2][i-2] && board[j+1][i-1] == player && board[j+3][i-3] == 0 {
                                return_col = i as u8 - 3;
                            }
                        }
                    }
                }
            }
        }
        //No possible block or win found
        if return_col > 7 {
            return_col = GameBoardComponent::easy_bot_toototto(board);
        }
        return_col
    }

        // Check who wins toot otto
        pub fn check_toototto_winner(column: usize, row: usize, player: &Player, board: [[u8; 6]; 4]) -> Gamestate{
            // let mut num = 1;
            // Vertical win check 
            let mut incr = 0;
            let mut i = 0;
            loop {
                if i > 2 {
                    break;
                }
                if (board[i][column] != 0 && board[i+1][column] !=0) && (board[i][column] != board[i+1][column]) {
                    if incr == 1 {
                        if board[i][column] == board[i-1][column] {
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
                    if (board[i][column] == 2) {
                        winner = "T";
                    }
                    else {
                        winner = "O";
                    }
                    println!("Player {} wins", winner);
                    return Gamestate::Gameover;
                    
                }
            }
            // Horizontal Win Check
            incr = 0;
            i = 0;
            let mut second = false;
            loop {
                if i > 4 {
                    break;
                }
                if (board[row][i] != 0 && board[row][i+1] != 0) && (board[row][i] != board[row][i+1]) {
                    if incr == 1 {
                        if board[row][i] == board[row][i-1] {
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
                    if (board[row][i] == 2) {
                        winner = "T";
                    }
                    else {
                        winner = "O";
                    }
                    println!("Player {} wins", winner);
                    return Gamestate::Gameover;

                }
                // Go back and check even 
                if i == 5 && second == false{
                    incr = 0;
                    i = 1;
                    second = true;
                }
            }
            // Diagonal Check
            incr = 0;
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
                if temp_row < 1 || temp_col > 3 {
                    break;
                }
                if (board[temp_row][temp_col] != 0 && board[temp_row - 1][temp_col + 1] != 0) && (board[temp_row - 1][temp_col + 1] != board[temp_row][temp_col]) {
                    if incr == 1 {
                        if board[temp_row][temp_col] == board[temp_row + 1][temp_col - 1] {
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
                    if (board[temp_row][temp_col] == 2) {
                        winner = "T";
                    }
                    else {
                        winner = "O";
                    }
                    println!("Player {} wins", winner);
                    return Gamestate::Gameover;
                }

            }

            // Other Diagonal Check
            incr = 0;
            let mut temp_row2 = row;
            let mut temp_col2 = column;
        
            loop {
                if temp_row2 == 3 || temp_col2 == 5 {
                    break;
                }  
                temp_row2 += 1;
                temp_col2 += 1;
            }
            // Now do checking
            loop {
                if temp_row2 < 1 || temp_col2 < 2 {
                    break;
                }
                if (board[temp_row2][temp_col2] != 0 && board[temp_row2 - 1][temp_col2 - 1] != 0) && (board[temp_row2 - 1][temp_col2 - 1] != board[temp_row2][temp_col2]) {
                    if incr == 1 {
                        if board[temp_row2][temp_col2] == board[temp_row2 + 1][temp_col2 + 1] {
                            incr += 1;
                        }
                        else {
                            incr = 0;
                            temp_row2 -= 1;
                            temp_col2 -= 1;
                        }
                    }
                    else {
                        incr += 1;
                        temp_row2 -= 2;
                        temp_col2 -= 2;
                    }
                }
                else {
                    incr = 0;
                    temp_row2 -= 1;
                    temp_col2 -= 1;
                }
                if incr == 2 {
                    let mut winner = "0";
                    if (board[temp_row2][temp_col2] == 2) {
                        winner = "T";
                    }
                    else {
                        winner = "O";
                    }
                    println!("Player {} wins", winner);
                    return Gamestate::Gameover;
                }

            }
            println!("No winner");
            return Gamestate::InProgress;
        }     


    // Check who wins connect 4
    pub fn check_connect4_winner(column: usize, row: usize, player: &Player, board: [[u8; 7]; 6]) -> Gamestate{
        let num: u8;
        match player {
            Player::Player1 => num = 1,
            Player::Player2 => num = 2,
        }
        // Vertical win check 
        let mut incr = 0;
        for i in 1..6 {
            if board[i][column] != 0 && board[i][column] == board[i-1][column] {
                incr += 1;
            }
            else {
                incr = 0;
            }
            if incr == 3 {
                println!("Player {} wins", num);
                return Gamestate::Gameover;
                
            }
        }
        // Horizontal Win Check
        incr = 0;
        // if board[row][0] != 0 {
        //     incr += 1;
        // }
        for i in 1..7 {
            if board[row][i] != 0 && board[row][i] == board[row][i-1] {
                incr += 1;
            }
            else {
                incr = 0;
            }
            if incr == 3 {
                println!("Player {} wins", num);
                return Gamestate::Gameover;
            }
        }
        // Diagonal Check 
        incr = 0;
        // Get to bottom row or leftmost column 
        let mut temp_row = row;
        let mut temp_col = column;
        loop {
            if temp_row == 5 || temp_col == 0 {
                break;
            }  
            temp_row += 1;
            temp_col -= 1;
        }
        if  board[temp_row][temp_col] != 0 {
            incr += 1;
        }
        // Now do checking
        loop {
            if temp_row == 0 || temp_col == 6 {
                break;
            }
            if board[temp_row - 1][temp_col + 1] != 0 && board[temp_row - 1][temp_col + 1] == board[temp_row][temp_col] {
                incr += 1;
            }
            else {
                incr = 0;
            }
            if incr == 4 {
                println!("Player {} wins", num);
                return Gamestate::Gameover;
            }
            temp_row -= 1;
            temp_col += 1;

        }
        // Other diagonal Check
        incr = 0;
        // Get to bottom row or rightmost column 
        let mut temp_row2 = row;
        let mut temp_col2 = column;
        loop {
            if temp_row2 == 5 || temp_col2 == 6 {
                break;
            }  
            temp_row2 += 1;
            temp_col2 += 1;
        }
        if  board[temp_row2][temp_col2] != 0 {
            incr += 1;
        }
        // Now do checking
        loop {
            if temp_row2 == 0 || temp_col2 == 0 {
                break;
            }
            if board[temp_row2 - 1][temp_col2 - 1] != 0 && board[temp_row2 - 1][temp_col2 - 1] == board[temp_row2][temp_col2] {
                incr += 1;
            }
            else {
                incr = 0;
            }
            if incr == 4 {
                println!("Player {} wins", num);
                return Gamestate::Gameover;
            }
            temp_row2 -= 1;
            temp_col2 -= 1;

        }
        println!("No winner");
        return Gamestate::InProgress;
    }       
}


impl Component for GameBoardComponent{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self{
        Self { conn4_board : [[0;7]; 6],
            toototto_board : [[0;6]; 4],
            gamestate : Gamestate::InProgress,
            gametype : Gametype::Human,
            curr_player : Player::Player1,
            viewmode: ViewMode::Default,
            gamename: GameName::Connect4,
        }
    }


    fn update(&mut self , _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let column;
        match msg{
            Msg::ColumnZero => {
                column = 0;
                },
            Msg::ColumnOne => {
                column = 1;
                },
            Msg::ColumnTwo => {
                column = 2;
                }, 
            Msg::ColumnThree => {
                column = 3;
                }, 
            Msg::ColumnFour => {
                column = 4;
                }, 
            Msg::ColumnFive => {
                column = 5;
                }, 
            Msg::ColumnSix => {
                column = 6;
                }, 
            Msg::Local => {
                //Set game mode to local and restart game
                self.gametype = Gametype::Human;
                self.conn4_board = [[0;7]; 6];
                self.toototto_board = [[0;6]; 4];
                self.curr_player = Player::Player1;
                return true
                },
            Msg::EasyCPU => {
                //Set game mode to easy CPU and restart game
                self.gametype = Gametype::ComputerEasy;
                self.conn4_board = [[0;7]; 6];
                self.toototto_board = [[0;6]; 4];
                self.curr_player = Player::Player1;
                return true
                },
            Msg::HardCPU => {
                //Set game mode to hard CPU and restart game
                self.gametype = Gametype::ComputerHard;
                self.conn4_board = [[0;7]; 6];
                self.toototto_board = [[0;6]; 4];
                self.curr_player = Player::Player1;
                return true
                },
            Msg::Connect4 => {
                //Set game name to connect4 and restart game
                self.gamename = GameName::Connect4;
                self.conn4_board = [[0;7]; 6];
                self.toototto_board = [[0;6]; 4];
                self.curr_player = Player::Player1;
                return true
                }, 
            Msg::TootOtto => {
                //Set game name to toototto and restart game
                self.gamename = GameName::TOOTOTTO;
                self.conn4_board = [[0;7]; 6];
                self.toototto_board = [[0;6]; 4];
                self.curr_player = Player::Player1;
                return true
                },
            Msg::NormalView => {
                //Set viewmode to normal
                self.viewmode = ViewMode::Default;
                return true
                }, 
            Msg::CBView=> {
                //Set viewmode to colorblind
                self.viewmode = ViewMode::ColorBlind;
                return true
                }, 
        }
        match self.gamename{
            GameName::Connect4 => {
                if self.conn4_board[0][column] != 0 {
                    //Flash stament to the webpage here
                    println!("Column is full");
                    return false;
                }
                //Find the first empty row
                let mut row_num = 5;
                for _ in 0..5 {
                    if self.conn4_board[row_num][column] == 0 {
                        break;
                    }
                    else {
                        row_num -= 1;
                    }
                }
                //Write the new move and change the turn
                match &self.curr_player {
                    Player::Player1 => {
                        self.conn4_board[row_num][column] = 1;
                        self.gamestate = GameBoardComponent::check_connect4_winner(column, row_num, &self.curr_player, self.conn4_board);
                        match self.gamestate {
                            Gamestate::Gameover => {
                                self.conn4_board = [[0;7]; 6];
                                self.gamestate = Gamestate::InProgress;
                            },
                            Gamestate::InProgress => {
                                match self.gametype {
                                    Gametype::ComputerEasy =>{
                                        let cpu_col = usize::from(GameBoardComponent::easy_bot_connect4(self.conn4_board));
                                        let mut cpu_row = 5;
                                        for _ in 0..5 {
                                            if self.conn4_board[cpu_row][cpu_col] == 0 {
                                                break;
                                            }
                                            else {
                                                cpu_row -= 1;
                                            }
                                        }
                                        self.conn4_board[cpu_row][cpu_col] = 2;
                                        self.gamestate = GameBoardComponent::check_connect4_winner(column, cpu_row, &self.curr_player, self.conn4_board);
                                        if matches!(self.gamestate, Gamestate::Gameover){
                                            self.conn4_board = [[0;7]; 6];
                                            self.gamestate = Gamestate::InProgress;
                                            self.curr_player = Player::Player1;
                                        }
                                    },
                                    Gametype::ComputerHard =>{
                                        let cpu_col = usize::from(GameBoardComponent::hard_bot_connect4(self.conn4_board));
                                        let mut cpu_row = 5;
                                        for _ in 0..5 {
                                            if self.conn4_board[cpu_row][cpu_col] == 0 {
                                                break;
                                            }
                                            else {
                                                cpu_row -= 1;
                                            }
                                        }
                                        self.conn4_board[cpu_row][cpu_col] = 2;
                                        self.gamestate = GameBoardComponent::check_connect4_winner(column, cpu_row, &self.curr_player, self.conn4_board);
                                        if matches!(self.gamestate, Gamestate::Gameover){
                                            self.conn4_board = [[0;7]; 6];
                                            self.gamestate = Gamestate::InProgress;
                                            self.curr_player = Player::Player1;
                                        }
                                    },
                                    Gametype::Human =>{
                                        self.curr_player = Player::Player2;
                                    }
                                }
                            }
                        }
                    },
                    Player::Player2 => {
                        self.conn4_board[row_num][column] = 2;
                        self.gamestate = GameBoardComponent::check_connect4_winner(column, row_num, &self.curr_player, self.conn4_board);
                        match self.gamestate {
                            Gamestate::Gameover => {
                                self.conn4_board = [[0;7]; 6];
                                self.gamestate = Gamestate::InProgress;
                            },
                            Gamestate::InProgress => {
                                self.curr_player = Player::Player1;
                            }
                        }
                    },     
                }
                return true;
            },
            GameName::TOOTOTTO =>  {
                if self.toototto_board[0][column] != 0 {
                    //Flash stament to the webpage here
                    println!("Column is full");
                    return false;
                }
                //Find the first empty row
                let mut row_num = 3;
                for _ in 0..3 {
                    if self.toototto_board[row_num][column] == 0 {
                        break;
                    }
                    else {
                        row_num -= 1;
                    }
                }
                //Write the new move and change the turn
                match &self.curr_player {
                    Player::Player1 => {
                        self.toototto_board[row_num][column] = 1;
                        self.gamestate = GameBoardComponent::check_toototto_winner(column, row_num, &self.curr_player, self.toototto_board);
                        match self.gamestate {
                            Gamestate::Gameover => {
                                self.toototto_board = [[0;6]; 4];
                                self.gamestate = Gamestate::InProgress;
                            },
                            Gamestate::InProgress => {
                                match self.gametype {
                                    Gametype::ComputerEasy =>{
                                        let cpu_col = usize::from(GameBoardComponent::easy_bot_toototto(self.toototto_board));
                                        let mut cpu_row = 3;
                                        for _ in 0..3 {
                                            if self.toototto_board[cpu_row][cpu_col] == 0 {
                                                break;
                                            }
                                            else {
                                                cpu_row -= 1;
                                            }
                                        }
                                        self.toototto_board[cpu_row][cpu_col] = 2;
                                        self.gamestate = GameBoardComponent::check_toototto_winner(column, cpu_row, &self.curr_player, self.toototto_board);
                                        if matches!(self.gamestate, Gamestate::Gameover){
                                            self.toototto_board = [[0;6]; 4];
                                            self.gamestate = Gamestate::InProgress;
                                            self.curr_player = Player::Player1;
                                        }
                                    },
                                    Gametype::ComputerHard =>{
                                        let cpu_col = usize::from(GameBoardComponent::hard_bot_toototto(self.toototto_board));
                                        let mut cpu_row = 3;
                                        for _ in 0..3 {
                                            if self.toototto_board[cpu_row][cpu_col] == 0 {
                                                break;
                                            }
                                            else {
                                                cpu_row -= 1;
                                            }
                                        }
                                        self.toototto_board[cpu_row][cpu_col] = 2;
                                        self.gamestate = GameBoardComponent::check_toototto_winner(column, cpu_row, &self.curr_player, self.toototto_board);
                                        if matches!(self.gamestate, Gamestate::Gameover){
                                            self.toototto_board = [[0;6]; 4];
                                            self.gamestate = Gamestate::InProgress;
                                            self.curr_player = Player::Player1;
                                        }
                                    },
                                    Gametype::Human =>{
                                        self.curr_player = Player::Player2;
                                    }
                                }
                            }
                        }
                    },
                    Player::Player2 => {
                        self.toototto_board[row_num][column] = 2;
                        self.gamestate = GameBoardComponent::check_toototto_winner(column, row_num, &self.curr_player, self.toototto_board);
                        match self.gamestate {
                            Gamestate::Gameover => {
                                self.toototto_board = [[0;6]; 4];
                                self.gamestate = Gamestate::InProgress;
                            },
                            Gamestate::InProgress => {
                                self.curr_player = Player::Player1;
                            }
                        }
                    },     
                }
                return true;
            }
        }
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let link: &Scope<GameBoardComponent> = ctx.link();
        let player_one_color: &str;
        let player_two_color: &str;
        match self.viewmode{
            ViewMode::Default =>{
                //RED AND YELLOW
                player_one_color = "background-color:#ff0000;";
                player_two_color = "background-color:#ffff00;";
            },
            ViewMode::ColorBlind=>{
                //BLUE AND ORANGE
                player_one_color = "background-color:#e66100;";
                player_two_color = "background-color:#5d3a9b;";
            }
        }
        html!{

            <div class = "webapp">
                // Can simply be a 6x7 grid with appropriate colors (Red, Yellow, or White)
                // There must be a better way to do this ....
                {match self.gamename{
                    GameName::Connect4 =>{
                        html!{ <div class="connect4-gameboard">
                        // ROW NUMBER ONE
                            <div class = "gamesquare">
                                {if self.conn4_board[0][0] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                else if self.conn4_board[0][0] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[0][1] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                else if self.conn4_board[0][1] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[0][2] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                else if self.conn4_board[0][2] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                            }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[0][3] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                else if self.conn4_board[0][3] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[0][4] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                else if self.conn4_board[0][4] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[0][5] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                else if self.conn4_board[0][5] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[0][6] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                else if self.conn4_board[0][6] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                }
                            </div>
                            
                            //ROW NUMBER TWO
                            <div class = "gamesquare">
                            {if self.conn4_board[1][0] == 1{
                                html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                            }
                            else if self.conn4_board[1][0] == 2{
                                html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                            }
                            else{
                                html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                            }
                            }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[1][1] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                else if self.conn4_board[1][1] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[1][2] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                else if self.conn4_board[1][2] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                            }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[1][3] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                else if self.conn4_board[1][3] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[1][4] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                else if self.conn4_board[1][4] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[1][5] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                else if self.conn4_board[1][5] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[1][6] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                else if self.conn4_board[1][6] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                }
                            </div>
                        //ROW NUMBER THREE
                            <div class = "gamesquare">
                                {if self.conn4_board[2][0] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                else if self.conn4_board[2][0] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[2][1] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                else if self.conn4_board[2][1] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[2][2] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                else if self.conn4_board[2][2] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                            }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[2][3] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                else if self.conn4_board[2][3] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[2][4] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                else if self.conn4_board[2][4] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[2][5] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                else if self.conn4_board[2][5] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[2][6] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                else if self.conn4_board[2][6] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                }
                            </div>
                    //ROW NUMBER FOUR
                            <div class = "gamesquare">
                                {if self.conn4_board[3][0] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                else if self.conn4_board[3][0] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[3][1] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                else if self.conn4_board[3][1] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[3][2] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                else if self.conn4_board[3][2] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                            }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[3][3] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                else if self.conn4_board[3][3] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[3][4] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                else if self.conn4_board[3][4] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[3][5] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                else if self.conn4_board[3][5] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[3][6] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                else if self.conn4_board[3][6] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                }
                            </div>
                //ROW NUMBER FIVE
                            <div class = "gamesquare">
                                {if self.conn4_board[4][0] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                else if self.conn4_board[4][0] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[4][1] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                else if self.conn4_board[4][1] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[4][2] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                else if self.conn4_board[4][2] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                            }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[4][3] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                else if self.conn4_board[4][3] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[4][4] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                else if self.conn4_board[4][4] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[4][5] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                else if self.conn4_board[4][5] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[4][6] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                else if self.conn4_board[4][6] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                }
                            </div>
                            //ROW SIX
                            <div class = "gamesquare">
                                {if self.conn4_board[5][0] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                else if self.conn4_board[5][0] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[5][1] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                else if self.conn4_board[5][1] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[5][2] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                else if self.conn4_board[5][2] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                            }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[5][3] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                else if self.conn4_board[5][3] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[5][4] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                else if self.conn4_board[5][4] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[5][5] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                else if self.conn4_board[5][5] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.conn4_board[5][6] == 1{
                                    html!{<button style = {player_one_color} onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                else if self.conn4_board[5][6] == 2{
                                    html!{<button style = {player_two_color} onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                                }
                                }
                            </div>
                        </div>}
                    },
                    GameName::TOOTOTTO => {
                        html!{
                        <div class = "toototto-gameboard">
                            //ROW ONE
                            <div class = "gamesquare">
                                {if self.toototto_board[0][0] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnZero)}>{"T"}</button>}
                                }
                                else if self.toototto_board[0][0] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnZero)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[0][1] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnOne)}>{"T"}</button>}
                                }
                                else if self.toototto_board[0][1] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnOne)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[0][2] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnTwo)}>{"T"}</button>}
                                }
                                else if self.toototto_board[0][2] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnTwo)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[0][3] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnThree)}>{"T"}</button>}
                                }
                                else if self.toototto_board[0][3] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnThree)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[0][4] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFour)}>{"T"}</button>}
                                }
                                else if self.toototto_board[0][4] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFour)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[0][5] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFive)}>{"T"}</button>}
                                }
                                else if self.toototto_board[0][5] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFive)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                }
                            </div>
                            //ROW TWO
                            <div class = "gamesquare">
                                {if self.toototto_board[1][0] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnZero)}>{"T"}</button>}
                                }
                                else if self.toototto_board[1][0] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnZero)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[1][1] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnOne)}>{"T"}</button>}
                                }
                                else if self.toototto_board[1][1] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnOne)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[1][2] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnTwo)}>{"T"}</button>}
                                }
                                else if self.toototto_board[1][2] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnTwo)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[1][3] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnThree)}>{"T"}</button>}
                                }
                                else if self.toototto_board[1][3] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnThree)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[1][4] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFour)}>{"T"}</button>}
                                }
                                else if self.toototto_board[1][4] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFour)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[1][5] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFive)}>{"T"}</button>}
                                }
                                else if self.toototto_board[1][5] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFive)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                }
                            </div>
                            //ROW THREE
                            <div class = "gamesquare">
                                {if self.toototto_board[2][0] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnZero)}>{"T"}</button>}
                                }
                                else if self.toototto_board[2][0] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnZero)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[2][1] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnOne)}>{"T"}</button>}
                                }
                                else if self.toototto_board[2][1] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnOne)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[2][2] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnTwo)}>{"T"}</button>}
                                }
                                else if self.toototto_board[2][2] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnTwo)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[2][3] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnThree)}>{"T"}</button>}
                                }
                                else if self.toototto_board[2][3] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnThree)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[2][4] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFour)}>{"T"}</button>}
                                }
                                else if self.toototto_board[2][4] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFour)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[2][5] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFive)}>{"T"}</button>}
                                }
                                else if self.toototto_board[2][5] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFive)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                }
                            </div>
                            //ROW FOUR
                            <div class = "gamesquare">
                                {if self.toototto_board[3][0] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnZero)}>{"T"}</button>}
                                }
                                else if self.toototto_board[3][0] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnZero)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[3][1] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnOne)}>{"T"}</button>}
                                }
                                else if self.toototto_board[3][1] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnOne)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[3][2] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnTwo)}>{"T"}</button>}
                                }
                                else if self.toototto_board[3][2] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnTwo)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[3][3] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnThree)}>{"T"}</button>}
                                }
                                else if self.toototto_board[3][3] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnThree)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[3][4] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFour)}>{"T"}</button>}
                                }
                                else if self.toototto_board[3][4] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFour)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                                }
                                }
                            </div>
                            <div class = "gamesquare">
                                {if self.toototto_board[3][5] == 1{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFive)}>{"T"}</button>}
                                }
                                else if self.toototto_board[3][5] == 2{
                                    html!{<button style = "font-size:50px;" onclick = {link.callback(|_| Msg::ColumnFive)}>{"O"}</button>}
                                }
                                else{
                                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                                }
                                }
                            </div>
                        </div>
                        }
                    }
                }
            }
                //Sidebar button area 
                <div class = "game-buttons">
                    <div class = "game-type-settings">
                        <button onclick = {link.callback(|_| Msg::Local)}>{"Local"}</button>
                        <button onclick = {link.callback(|_| Msg::EasyCPU)}>{"Easy CPU"}</button>
                        <button onclick = {link.callback(|_| Msg::HardCPU)}>{"Hard CPU"}</button>
                    </div>
                    <div class = "game-name-settings">
                        <button onclick = {link.callback(|_| Msg::Connect4)}>{"Connect4"}</button>
                        <button onclick = {link.callback(|_| Msg::TootOtto)}>{"TOOT-OTTO"}</button>

                    </div>
                    <div class = "game-color-settings">
                        <button onclick = {link.callback(|_| Msg::NormalView)}>{"Normal"}</button>
                        <button onclick = {link.callback(|_| Msg::CBView)}>{"Color Blind"}</button>
                    </div>
                </div>
            </div>
        }
    }
    }




fn main() {
    yew::start_app::<GameBoardComponent>();
}
