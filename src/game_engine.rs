pub mod game_engine {
    
impl Board {
    pub fn new() -> Board {
        Board {board:['1','2','3','4','5','6','7','8','9']}
    }
}   
    pub fn print_board(board: [char; 9]) {
        for line in generate_print(board) {
            println!("{line}")
        }
    }
    fn generate_print(board: [char; 9]) -> Vec<String> {
        let mut vec_board: Vec<String> = Vec::new();

        for row in 0..3 {
            //let mut line: Vec<String> = Vec::new();
            vec_board.push(board[row*3].to_string() + &" " + &board[row*3+1].to_string() + &" " + &board[row*3+2].to_string());

        }
        vec_board

    }
    

    

pub struct Board {
    pub board: [char; 9],
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn print_board() {
        let board: [char; 9]  = ['1','2','3','4','5','6','7','8','9'];
        assert_eq!(vec!["1 2 3", "4 5 6", "7 8 9"], generate_print(board))
    }}


}







