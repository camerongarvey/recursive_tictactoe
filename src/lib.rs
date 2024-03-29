mod game_engine;
mod robot;
pub use crate::game_engine::game_engine as game;
pub use crate::robot::robot as bot;

pub fn run() {
    let mut board=game::Board::new(); 
    let mut bot = bot::Master::new();


    println!("Do you want to be 'x' or 'o'?: ");
    let mut player_turn: String = String::new();
        
    std::io::stdin()
        .read_line(&mut player_turn)
        .expect("Failed to read input");
    if player_turn == "x\n" {
        board.turn = 'o';
    } 
    loop { 
        board.print_board();
        
        if board.check_winner() || board.check_tie(){
            break
        }
        if board.turn == 'x' {
            board.turn = 'o';
            let robot_move = bot.get_move(board.board, board.turn, board.turn);
            println!("The Robot moves: {}", robot_move); 
            board.make_move(robot_move, 'o')

        } else {
            board.turn = 'x';
            board.player_move();
        }
        
    }
   

}


