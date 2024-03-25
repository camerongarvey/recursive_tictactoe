mod game_engine;
pub use crate::game_engine::game_engine as game;

pub fn run() {
    let mut board=game::Board::new(); 
    loop {
        board.print_board();
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

