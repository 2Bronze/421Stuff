use yew::prelude::*;
use yew::html::Scope;
mod text_input;
use text_input::TextInput;
use wasm_request::{get_options, request, Method, DataType};

use std::collections::HashMap;
use std::error::Error;
use reqwest::prelude::*;
use http::StatusCode;



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
    SignUp,
    SignIn,
    UsernameInput(String),
    PasswordInput(String),
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
    pub username: String,
    pub password: String,
    pub is_authenticated: bool,

}

impl GameBoardComponent{

    //Send a POST req to the database saving the game
    pub fn save_game_details(){
        
    }

    pub async fn post_new_user(user: User){
        let options = get_options::<User>(
            "https://localhost:4000",
            Method::POST,
            None,
            Some(DataType::Json(user)),
            );
            let resp = request(options).await.unwrap().into_serde::<SignInResponse>().unwrap();
            resp;
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
        for i in 1..5 {
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
        if board[row][0] != 0 {
            incr += 1;
        }
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
        // Get to top row or leftmost column 
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
            username: String::new(),
            password: String::new(),
            is_authenticated: false,
        }
    }


    fn update(&mut self , _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let column;
        match msg{
            Msg::SignIn =>{
                let mut map = HashMap::new();
                map.insert("username", self.username);
                map.insert("password", self.password);

                let client = reqwest::blocking::Client::new();
                let res = client.post("http://localhost/4000")
                    .json(&map)
                    .send();
                if res.unwrap().status() == StatusCode::OK{
                    self.is_authenticated = true;
                }
                return true;
            },
            Msg::SignUp =>{
                let mut map: HashMap<&str,()>= HashMap::new();
                map.insert("username", self.username);
                map.insert("password", self.password);
                map.insert("wins", 0);
                map.insert("losses", 0);

                let client = reqwest::blocking::Client::new();
                let res = client.post("http://localhost/4000")
                    .json(&map)
                    .send();
                if res.unwrap().status() == StatusCode::OK{
                    self.is_authenticated = true;
                }
                return true;
            },
            Msg::UsernameInput(uname) =>{
                self.username = uname;
                return false;
            },
            Msg::PasswordInput(pass)=>{
                self.password = pass;
                return false; 
            },
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
                return true
                },
            Msg::EasyCPU => {
                //Set game mode to easy CPU and restart game
                self.gametype = Gametype::ComputerEasy;
                self.conn4_board = [[0;7]; 6];
                self.toototto_board = [[0;6]; 4];
                return true
                },
            Msg::HardCPU => {
                //Set game mode to hard CPU and restart game
                self.gametype = Gametype::ComputerHard;
                self.conn4_board = [[0;7]; 6];
                self.toototto_board = [[0;6]; 4];
                return true
                },
            Msg::Connect4 => {
                //Set game name to connect4 and restart game
                self.gamename = GameName::Connect4;
                self.conn4_board = [[0;7]; 6];
                self.toototto_board = [[0;6]; 4];
                return true
                }, 
            Msg::TootOtto => {
                //Set game name to toototto and restart game
                self.gamename = GameName::TOOTOTTO;
                self.conn4_board = [[0;7]; 6];
                self.toototto_board = [[0;6]; 4];
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
                                //GameBoardComponent::save_game_details();
                                self.conn4_board = [[0;7]; 6];
                                self.gamestate = Gamestate::InProgress;
                            },
                            Gamestate::InProgress => {
                                self.curr_player = Player::Player2;
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
                                self.curr_player = Player::Player2;
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

                <div class = "login-box">
                    <h1>{"Sign In / Sign Up"}</h1>
                    <TextInput on_change = {link.callback(Msg::UsernameInput)} value={self.username.clone()} />
                    <TextInput on_change ={link.callback(Msg::PasswordInput)} value={self.password.clone()} />
                    <button onclick = {link.callback(|_| Msg::SignUp)}>{"Sign Up"}</button>
                    <button onclick = {link.callback(|_| Msg::SignIn)}>{"Sign In"}</button>
                </div>
            </div>
        }
    }
    }




fn main() {
    yew::start_app::<GameBoardComponent>();
}
