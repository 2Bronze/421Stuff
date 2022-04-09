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
    Computer,
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
}

struct GameBoardComponent{
    //Game board with 0 representing empty, 1 representing red, 2 representing yellow
    pub board: [[u8; 7]; 6],
    pub gamestate: Gamestate,
    pub gametype: Gametype, 
    pub curr_player: Player,
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
        for i in 0..7 {
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
        // NEED TO ADD A DIAGONAL CHECK, NOT SURE HOW THATS GOING TO WORK 
        println!("No winner");
        return Gamestate::InProgress;
    }       
}


impl Component for GameBoardComponent{
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self{
        Self { board : [[0;7]; 6],
            gamestate : Gamestate::InProgress,
            gametype : Gametype::Human,
            curr_player : Player::Red
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
        }
        if self.board[0][column] != 0 {
            //Flash stament to the webpage here
            println!("Column is full");
            return false;
        }
        //Find the first empty row
        let mut row_num = 5;
        for _ in 0..5 {
            if self.board[row_num][column] == 0 {
                break;
            }
            else {
                row_num -= 1;
            }
        }
        //Write the new move and change the turn
        match &self.curr_player {
            Player::Red => {
                self.board[row_num][column] = 1;
                self.gamestate = GameBoardComponent::check_winner(column, row_num, &self.curr_player, self.board);
                match self.gamestate {
                    Gamestate::Gameover => {
                        self.board = [[0;7]; 6];
                        self.gamestate = Gamestate::InProgress;
                    },
                    Gamestate::InProgress => {
                        self.curr_player = Player::Yellow;
                    }
                }
            },
            Player::Yellow => {
                self.board[row_num][column] = 2;
                self.gamestate = GameBoardComponent::check_winner(column, row_num, &self.curr_player, self.board);
                match self.gamestate {
                    Gamestate::Gameover => {
                        self.board = [[0;7]; 6];
                        self.gamestate = Gamestate::InProgress;
                    },
                    Gamestate::InProgress => {
                        self.curr_player = Player::Red;
                    }
                }
            },     
        }
        return true;
    }


    fn view(&self, ctx: &Context<Self>) -> Html {
        let link: &Scope<GameBoardComponent> = ctx.link();
        html!{
            // Can simply be a 6x7 grid with appropriate colors (Red, Yellow, or White)
            // There must be a better way to do this ....
            <div class="gameboard">
            // ROW NUMBER ONE
                <div class = "gamesquare">
                    {if self.board[0][0] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                     }
                     else if self.board[0][0] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                     }
                     else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                     }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[0][1] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    else if self.board[0][1] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[0][2] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                    else if self.board[0][2] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                }
                </div>
                <div class = "gamesquare">
                    {if self.board[0][3] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    else if self.board[0][3] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[0][4] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    else if self.board[0][4] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[0][5] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    else if self.board[0][5] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[0][6] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    else if self.board[0][6] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    }
                </div>
                
                //ROW NUMBER TWO
                <div class = "gamesquare">
                {if self.board[1][0] == 1{
                    html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                 }
                 else if self.board[1][0] == 2{
                    html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                 }
                 else{
                    html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                 }
                }
                </div>
                <div class = "gamesquare">
                    {if self.board[1][1] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    else if self.board[1][1] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[1][2] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                    else if self.board[1][2] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                }
                </div>
                <div class = "gamesquare">
                    {if self.board[1][3] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    else if self.board[1][3] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[1][4] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    else if self.board[1][4] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[1][5] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    else if self.board[1][5] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[1][6] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    else if self.board[1][6] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    }
                </div>
            //ROW NUMBER THREE
                <div class = "gamesquare">
                    {if self.board[2][0] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    else if self.board[2][0] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[2][1] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    else if self.board[2][1] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[2][2] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                    else if self.board[2][2] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                }
                </div>
                <div class = "gamesquare">
                    {if self.board[2][3] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    else if self.board[2][3] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[2][4] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    else if self.board[2][4] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[2][5] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    else if self.board[2][5] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[2][6] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    else if self.board[2][6] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    }
                </div>
        //ROW NUMBER FOUR
                <div class = "gamesquare">
                    {if self.board[3][0] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    else if self.board[3][0] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[3][1] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    else if self.board[3][1] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[3][2] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                    else if self.board[3][2] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                }
                </div>
                <div class = "gamesquare">
                    {if self.board[3][3] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    else if self.board[3][3] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[3][4] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    else if self.board[3][4] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[3][5] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    else if self.board[3][5] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[3][6] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    else if self.board[3][6] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    }
                </div>
    //ROW NUMBER FIVE
                <div class = "gamesquare">
                    {if self.board[4][0] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    else if self.board[4][0] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[4][1] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    else if self.board[4][1] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[4][2] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                    else if self.board[4][2] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                }
                </div>
                <div class = "gamesquare">
                    {if self.board[4][3] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    else if self.board[4][3] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[4][4] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    else if self.board[4][4] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[4][5] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    else if self.board[4][5] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[4][6] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    else if self.board[4][6] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    }
                </div>
                //ROW SIX
                <div class = "gamesquare">
                    {if self.board[5][0] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    else if self.board[5][0] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnZero)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[5][1] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    else if self.board[5][1] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnOne)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[5][2] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                    else if self.board[5][2] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnTwo)}></button>}
                    }
                }
                </div>
                <div class = "gamesquare">
                    {if self.board[5][3] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    else if self.board[5][3] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnThree)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[5][4] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    else if self.board[5][4] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFour)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[5][5] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    else if self.board[5][5] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnFive)}></button>}
                    }
                    }
                </div>
                <div class = "gamesquare">
                    {if self.board[5][6] == 1{
                        html!{<button style = "background-color:#ff0000;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    else if self.board[5][6] == 2{
                        html!{<button style = "background-color:#ffff00;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    else{
                        html!{<button style = "background-color:#ffffff;" onclick = {link.callback(|_| Msg::ColumnSix)}></button>}
                    }
                    }
                </div>

            </div>
        }
    }
}




fn main() {
    yew::start_app::<GameBoardComponent>();
}
