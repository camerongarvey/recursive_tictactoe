mod game_engine;
pub use crate::game_engine::game_engine as game;

pub fn run() {
    let board=game::Board::new();
    game::print_board(board.board);
    
}

