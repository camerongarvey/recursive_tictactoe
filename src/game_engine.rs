pub mod game_engine {

    
impl Board {
    pub fn new() -> Board {
        Board {board:['1','2','3','4','5','6','7','8','9'], turn: 'x'}
    }

    fn make_move(&mut self, target:char, replacement: char) {
        let mut temp_vec: Vec<char> = Vec::new();
        for x in self.board {
            if target == x {temp_vec.push(replacement);}
            else {temp_vec.push(x);}
        }
        let new_board = <[char; 9]>::try_from(temp_vec);
        self.board = new_board.unwrap();
    }
    
    fn cheack_legal_move(&self, target:char) -> bool {
        if self.board.contains(&target) { 
            return true
        }
        false
    }

    fn get_move(&self) -> char {
        println!("It is {}'s turn. Input move below: ", self.turn);
        
            let mut target: String = String::new();
        
            std::io::stdin()
                .read_line(&mut target)
                .expect("Failed to read input");
        
            let target: char = target.chars().next().unwrap();
            return target }   
    
    pub fn print_board(&self) {
        for line in self.generate_print() {
            println!("{line}")
        }
    }

    fn generate_print(&self) -> Vec<String> {
        let mut vec_board: Vec<String> = Vec::new();

        for row in 0..3 {
            //let mut line: Vec<String> = Vec::new();
            vec_board.push(self.board[row*3].to_string() + &" " + &self.board[row*3+1].to_string() + &" " + &self.board[row*3+2].to_string());

        }
        vec_board

    }

    pub fn player_move(&mut self) {
        let target: char = self.get_move();
        if self.cheack_legal_move(target) {
            self.make_move(target, self.turn);
        } else {
            self.player_move();
        }
    }

    pub fn check_winner(&self) -> bool {
        let winning_combinations: [[usize; 3]; 8] = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8],  // Horizontal
            [0, 3, 6], [1, 4, 7], [2, 5, 8],  // Vertical
            [0, 4, 8], [2, 4, 6],             // Diagonal
        ];

        for &combo in winning_combinations.iter() {
            let symbols: Vec<char> = combo.iter().map(|&i| self.board[i]).collect();
            if symbols.iter().all(|&symbol| symbol == self.turn) {
                println!("{} Wins!", self.turn);
                return true;
            }
        }

        false
    }

    pub fn check_tie(&self) -> bool {
        println!("It's a tie!");
        self.board.iter().all(|&symbol| symbol == 'x' || symbol == 'o')
     
    }
}   

pub struct Board {
    pub board: [char; 9],
    pub turn: char,
}

#[cfg(test)]
mod tests {
}
}










