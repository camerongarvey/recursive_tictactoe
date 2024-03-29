pub mod robot {
    use std::collections::HashMap;

pub struct Iterator {
    pub board: [char; 9],
    pub robot_turn: char,
    pub current_turn: char,
    pub history: Vec<char>,
}

impl Iterator {
    pub fn new(board: [char; 9], robot_turn:char, current_turn: char, history:Vec<char> ) -> Iterator {
        Iterator {board, robot_turn, current_turn, history: history}
    }
    
    pub fn iterate(&mut self, mut list: &mut Vec<Vec<char>>) {
        if self.board.iter().all(|&symbol| symbol == 'x' || symbol == 'o') {
            self.history.push('t');
            list.push(self.history.clone());
            //println!("{:?}", self.history);
            
            return
        } else {
            for sqaure in 0..9 {
                if self.board[sqaure] != 'x' && self.board[sqaure] != 'o' {
                    let mut new_bot = Iterator::new(self.board.clone(), self.robot_turn.clone(), self.current_turn.clone(), self.history.clone());
                    new_bot.history.push(self.board[sqaure]);
                    new_bot.board[sqaure] = self.current_turn;

                    if new_bot.check_winner() {
                        if new_bot.current_turn == new_bot.robot_turn {
                            new_bot.history.push('w');
                            list.push(new_bot.history.clone());
                            //return;
                        } else {
                                new_bot.history.push('l');
                                list.push(new_bot.history.clone());
                                //return;
                            }
                    
                    } else {
                        if 'x' == self.current_turn {new_bot.current_turn = 'o'} else {new_bot.current_turn = 'x'}
                        new_bot.iterate(&mut list);
                    }
                }}
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
            if symbols.iter().all(|&symbol| symbol == self.current_turn) {
                return true;
            }
        }

        false
    }

}

pub struct Master {
    pub moves: Vec<Vec<char>>,
}

impl Master {
    pub fn new() -> Master {
        Master {moves: Vec::new()}
    }
    pub fn get_move(&mut self, board: [char; 9], robot_turn: char, current_turn: char,) -> char {
        self.moves = vec![];
        let mut bot = Iterator::new(board, robot_turn, current_turn, Vec::new());
        bot.iterate(&mut self.moves);
        self.find_best()
    }
    fn find_best(&self) -> char {
        let mut win_moves:Vec<Vec<char>> = Vec::new();
        let mut tie_moves: Vec<Vec<char>> = Vec::new();
        let mut lose_moves: Vec<Vec<char>> = Vec::new();
        for possible_move in self.moves.clone() {
            if possible_move.contains(&'w') {
                win_moves.push(possible_move.clone());
                
            } else if possible_move.contains(&'t') {
                tie_moves.push(possible_move.clone());
            } else {
                lose_moves.push(possible_move.clone());
            }
        } 

        win_moves.sort_by_key(|inner_vec| inner_vec.len());
        lose_moves.sort_by_key(|inner_vec| inner_vec.len());
        
        if win_moves.len() > 0 && win_moves[0].len() == 2 {
            return win_moves[0][0];
        } else if lose_moves.len() > 0 && lose_moves[0].len() == 3 {
            return lose_moves[0][1];
            
        } else if lose_moves.len() >= win_moves.len() {
            if let Some((min_key, _)) = self.map_moves(&lose_moves).iter().min_by_key(|(_, &v)| v) {
                    return *min_key;
            } else {}
         }  else if let Some((min_key, _)) = self.map_moves(&tie_moves).iter().max_by_key(|(_, &v)| v) {

            return *min_key;
         }

    return self.moves[0][0];

}


    fn map_moves(&self, list:&Vec<Vec<char>>) -> HashMap<char, i32>{
        let mut map = HashMap::new();
        for item in list {
            let count = map.entry(item[0]).or_insert(0);
            *count += 1;
        }
        map
    }       

    }
}

