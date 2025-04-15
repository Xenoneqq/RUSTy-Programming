fn main() {
    game_manager();
}

#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

fn game_manager(){
    let mut board = [['-'; 3]; 3];
    let mut player = 'O';
    let mut gamestate = 0;
    let mut spot = 0;
    let mut moves = 0;
    while gamestate == 0 {
        print_board(&board);
        println!("Players {} turn, Pick position 1 - 9: ", player);
        read!(spot as usize);
        if spot <= 0 || spot > 9 {println!("Incorrect input! Try again"); continue;}
        if place(spot-1 , player, &mut board) == 0 {println!("This spot is taken!"); continue;}
        gamestate = check_win(&board);
        if player == 'X' {player = 'O';}
        else {player = 'X';}
        moves += 1;
        if moves == 9 {
            print_board(&board);
            println!("Game ended in a tie!");
            return;
        }
    }
}

fn place(spot: usize, symbol: char, board: &mut [[char; 3]; 3]) -> u32 {
    let row = spot / 3;
    let col = spot % 3;
    if board[row][col] != '-' {return 0;}
    board[row][col] = symbol;
    1
}

fn print_board(board: &[[char; 3]; 3]) {
    print!("{}[2J", 27 as char);
    for row in board {
        println!("{} {} {}", row[0], row[1], row[2]);
    }
    println!("\n")
}

fn check_win(board: &[[char; 3]; 3]) -> u32 {
    // checking for three in a row
    for row in board {
        if row[0] != '-' && row[0] == row[1] && row[1] == row[2] {
            print_board(&board);
            display_end(row[0]);
            return 1;
        }
    }

    // checking for three in a column
    let mut col = 0;
    while col < 3 {
        if board[0][col] != '-' && board[0][col] == board[1][col] && board[1][col] == board[2][col] {
            print_board(&board);
            display_end(board[0][col]);
            return 1;
        }
        col += 1;
    }

    // checking diagonals
    if board[0][0] != '-' && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        print_board(&board);
        display_end(board[0][0]);
        return 1;
    }
    if board[0][2] != '-' && board[0][2] == board[1][1] && board[1][1] == board[2][0]{
        print_board(&board);
        display_end(board[1][1]);
        return 1;
    }


    0
}

fn display_end(player: char){
    println!("wygrywa gracz {}!" , player);
}
