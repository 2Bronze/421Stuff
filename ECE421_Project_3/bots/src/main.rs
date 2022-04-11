use rand::prelude::*;

fn easy_bot_connect4(board: [[u8; 7]; 6]) -> u8 {
    let mut rng = thread_rng();
    let mut return_col = rng.gen_range(0..7);
    //check if col is full
    loop {
        if board[0][return_col as usize] == 0 {
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
        if board[0][return_col as usize] == 0 {
            break;
        }
        return_col = rng.gen_range(0..6);
    }
    return_col
}

fn hard_bot_connect4(board: [[u8; 7]; 6]) -> u8 {
    let mut return_col = 255;
    for j in (0..6).rev() {
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
                    if j > 2 {
                        if board[j-1][i] == player && board[j-2][i] == player && board[j-3][i] == 0 {
                            return_col = i as u8;
                        }
                    }


                    //check diagonal (up right/ up left)
                    if i <= 3 && j > 2 {
                        if board[j-1][i+1] == player && board[j-2][i+2] == player && board[j-3][i+3] == 0 {
                            return_col = i as u8 + 3;
                        }
                    }

                    if i >= 3 && j > 2 {
                        if board[j-1][i-1] == player && board[j-2][i-2] == player && board[j-3][i-3] == 0 {
                            return_col = i as u8 - 3;
                        }
                    }
                }
            }
        }
    }

    //No possible block or win found
    if return_col > 7 {
        return_col = easy_bot_connect4(board);
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
    for j in (0..4).rev() {
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
                    if j == 3 {
                        if board[j-1][i] == board[j-2][i] && board[j-1][i] == player && board[j-3][i] == 0 {
                            return_col = i as u8;
                        }
                    }

                    //check diagonal (up right/ up left)
                    if i <= 2 && j == 3 {
                        if board[j-1][i+1] == board[j-2][i+2] && board[j-1][i+1] == player && board[j-3][i+3] == 0 {
                            return_col = i as u8 + 3;
                        }
                    }

                    if i >= 3 && j == 3 {
                        if board[j-1][i-1] == board[j-2][i-2] && board[j-1][i-1] == player && board[j-3][i-3] == 0 {
                            return_col = i as u8 - 3;
                        }
                    }
                }
            }
        }
    }
    //No possible block or win found
    if return_col > 7 {
        return_col = easy_bot_toototto(board);
    }
    return_col
}

fn print_c4_board(board: [[u8; 7]; 6]) {
    for j in 0..6 {
        for i in 0..7 {
            print!("{} ", board[j][i]);
        }
        println!("");
    }
}

fn print_toototto_board(board: [[u8; 6]; 4]) {
    for j in 0..4 {
        for i in 0..6 {
            print!("{:?} ", board[j][i]);
        }
        println!("");
    }
}

fn main() {
    let mut c4_board = [[0; 7]; 6];
    let mut ot_board = [[0; 6]; 4];
    println!("{:?}", easy_bot_connect4(c4_board));
    println!("{:?}", easy_bot_toototto(ot_board));
    let mut c4_board = [[0; 7]; 6];
    c4_board[5][5] = 2;
    c4_board[3][3] = 2;
    c4_board[4][4] = 2;

    print_c4_board(c4_board);
    println!("COLUMN {:?}", hard_bot_connect4(c4_board));
    let mut ot_board = [[0; 6]; 4];
    ot_board[3][1] = 1;
    ot_board[3][2] = 2;
    ot_board[3][3] = 2;
    ot_board[2][2] = 1;
    // ot_board[1][3] = 1;
    // ot_board[1][1] = 1;

    print_toototto_board(ot_board);
    println!("COLUMN: {:?}", hard_bot_toototto(ot_board));
}
