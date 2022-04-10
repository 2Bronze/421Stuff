use yew::prelude::*;
use yew::html::Scope;


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
    Red,
    Yellow,
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
    pub conn4Board: [[u8; 7]; 6],
    pub tootottoBoard: [[u8; 6]; 4],
    pub gamestate: Gamestate,
    pub gametype: Gametype, 
    pub curr_player: Player,
    pub viewmode: ViewMode,
    pub gamename: GameName,
}

impl GameBoardComponent{
    // Check who wins
    pub fn check_winner(column: usize, row: usize, player: &Player, board: [[u8; 7]; 6]) -> Gamestate{
        let num: u8;
        match player {
            Player::Red => num = 1,
            Player::Yellow => num = 2,
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
        Self { conn4Board : [[0;7]; 6],
            tootottoBoard : [[0;6]; 4],
            gamestate : Gamestate::InProgress,
            gametype : Gametype::Human,
            curr_player : Player::Red,
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
                self.conn4Board = [[0;7]; 6];
                self.tootottoBoard = [[0;6]; 4];
                return true
                },
            Msg::EasyCPU => {
                //Set game mode to easy CPU and restart game
                self.gametype = Gametype::ComputerEasy;
                self.conn4Board = [[0;7]; 6];
                self.tootottoBoard = [[0;6]; 4];
                return true
                },
            Msg::HardCPU => {
                //Set game mode to hard CPU and restart game
                self.gametype = Gametype::ComputerHard;
                self.conn4Board = [[0;7]; 6];
                self.tootottoBoard = [[0;6]; 4];
                return true
                },
            Msg::Connect4 => {
                //Set game name to connect4 and restart game
                self.gamename = GameName::Connect4;
                self.conn4Board = [[0;7]; 6];
                self.tootottoBoard = [[0;6]; 4];
                return true
                }, 
            Msg::TootOtto => {
                //Set game name to toototto and restart game
                self.gamename = GameName::TOOTOTTO;
                self.conn4Board = [[0;7]; 6];
                self.tootottoBoard = [[0;6]; 4];
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
                if self.conn4Board[0][column] != 0 {
                    //Flash stament to the webpage here
                    println!("Column is full");
                    return false;
                }
                //Find the first empty row
                let mut row_num = 5;
                for _ in 0..5 {
                    if self.conn4Board[row_num][column] == 0 {
                        break;
                    }
                    else {
                        row_num -= 1;
                    }
                }
                //Write the new move and change the turn
                match &self.curr_player {
                    Player::Red => {
                        self.conn4Board[row_num][column] = 1;
                        self.gamestate = GameBoardComponent::check_winner(column, row_num, &self.curr_player, self.conn4Board);
                        match self.gamestate {
                            Gamestate::Gameover => {
                                self.conn4Board = [[0;7]; 6];
                                self.gamestate = Gamestate::InProgress;
                            },
                            Gamestate::InProgress => {
                                self.curr_player = Player::Yellow;
                            }
                        }
                    },
                    Player::Yellow => {
                        self.conn4Board[row_num][column] = 2;
                        self.gamestate = GameBoardComponent::check_winner(column, row_num, &self.curr_player, self.conn4Board);
                        match self.gamestate {
                            Gamestate::Gameover => {
                                self.conn4Board = [[0;7]; 6];
                                self.gamestate = Gamestate::InProgress;
                            },
                            Gamestate::InProgress => {
                                self.curr_player = Player::Red;
                            }
                        }
                    },     
                }
                return true;
            },
            GameName::TOOTOTTO =>  {
                return true
            }
        }

    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let link: &Scope<GameBoardComponent> = ctx.link();
        html!{

            <div class = "webapp">
                // Can simply be a 6x7 grid with appropriate colors (Red, Yellow, or White)
                // There must be a better way to do this ....
                <div class="gameboard">
                // ROW NUMBER ONE
                    <div class = "gamesquare">
                        {if self.conn4Board[0][0] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        else if self.conn4Board[0][0] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[0][1] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        else if self.conn4Board[0][1] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[0][2] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                        else if self.conn4Board[0][2] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                    }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[0][3] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        else if self.conn4Board[0][3] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[0][4] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        else if self.conn4Board[0][4] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[0][5] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        else if self.conn4Board[0][5] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[0][6] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        else if self.conn4Board[0][6] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        }
                    </div>
                    
                    //ROW NUMBER TWO
                    <div class = "gamesquare">
                    {if self.conn4Board[1][0] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    else if self.conn4Board[1][0] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[1][1] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        else if self.conn4Board[1][1] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[1][2] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                        else if self.conn4Board[1][2] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                    }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[1][3] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        else if self.conn4Board[1][3] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[1][4] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        else if self.conn4Board[1][4] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[1][5] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        else if self.conn4Board[1][5] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[1][6] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        else if self.conn4Board[1][6] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        }
                    </div>
                //ROW NUMBER THREE
                    <div class = "gamesquare">
                        {if self.conn4Board[2][0] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        else if self.conn4Board[2][0] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[2][1] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        else if self.conn4Board[2][1] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[2][2] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                        else if self.conn4Board[2][2] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                    }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[2][3] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        else if self.conn4Board[2][3] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[2][4] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        else if self.conn4Board[2][4] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[2][5] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        else if self.conn4Board[2][5] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[2][6] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        else if self.conn4Board[2][6] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        }
                    </div>
            //ROW NUMBER FOUR
                    <div class = "gamesquare">
                        {if self.conn4Board[3][0] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        else if self.conn4Board[3][0] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[3][1] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        else if self.conn4Board[3][1] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[3][2] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                        else if self.conn4Board[3][2] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                    }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[3][3] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        else if self.conn4Board[3][3] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[3][4] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        else if self.conn4Board[3][4] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[3][5] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        else if self.conn4Board[3][5] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[3][6] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        else if self.conn4Board[3][6] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        }
                    </div>
        //ROW NUMBER FIVE
                    <div class = "gamesquare">
                        {if self.conn4Board[4][0] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        else if self.conn4Board[4][0] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[4][1] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        else if self.conn4Board[4][1] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[4][2] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                        else if self.conn4Board[4][2] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                    }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[4][3] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        else if self.conn4Board[4][3] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[4][4] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        else if self.conn4Board[4][4] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[4][5] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        else if self.conn4Board[4][5] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[4][6] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        else if self.conn4Board[4][6] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        }
                    </div>
                    //ROW SIX
                    <div class = "gamesquare">
                        {if self.conn4Board[5][0] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        else if self.conn4Board[5][0] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[5][1] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        else if self.conn4Board[5][1] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[5][2] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                        else if self.conn4Board[5][2] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                        }
                    }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[5][3] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        else if self.conn4Board[5][3] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[5][4] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        else if self.conn4Board[5][4] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[5][5] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        else if self.conn4Board[5][5] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                        }
                        }
                    </div>
                    <div class = "gamesquare">
                        {if self.conn4Board[5][6] == 1{
                            html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        else if self.conn4Board[5][6] == 2{
                            html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        else{
                            html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                        }
                        }
                    </div>
                </div>

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
