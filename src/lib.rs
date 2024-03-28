mod game_engine;
mod robot;
pub use crate::game_engine::game_engine as game;
pub use crate::robot::robot as bot;

pub fn run() {
    let mut board=game::Board::new(); 
    let mut bot = bot::Master::new();
    loop { 
        board.print_board();
        println!("You should move: {}",bot.get_move(board.board, board.turn, board.turn));   
        board.player_move();
        if board.check_winner() || board.check_tie(){
            break
        }
        if board.turn == 'x' {
            board.turn = 'o';

        } else {
            board.turn = 'x'
        }
        
    }
   

}


