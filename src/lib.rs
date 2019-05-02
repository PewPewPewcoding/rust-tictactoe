enum TicTacToe {
    X,
    O,
    Empty,
}

struct GameState {
    board: [TicTacToe; 9],
}

impl GameState {
    pub fn new(input: String) -> GameState {
        let mut board = [TicTacToe::Empty; 9];

        for i in input {
            match i {
                'X' => board[i] = TicTacToe::X,
                'O' => board[i] = TicTacToe::O,
                _ => (),
            }
        }

        GameState { board }
    }

    pub fn possible_moves(&self) -> Vec<GameState> {
        let mut (x, o, e) = (0, 0, 0);
        let mut result = Vec::new();

        for i in self.board {
            match i {
                TicTacToe::X => x += 1,
                TicTacToe::O => o += 1,
                TicTacToe::Empty => e += 1,
            }
        }
        if e == 0 {
            return result;

        } else if x == o {
            for i in self.board {
                if i == TicTacToe::Empty {
                    let tmp = self.clone();
                    tmp[i] = TicTacToe::X;
                    result.push(tmp);
                }
            }
        } else {
            for i in self.board {
                if i == TicTacToe::Empty {
                    let tmp = self.clone();
                    tmp[i] = TicTacToe::O;
                    result.push(tmp);
                }
            }
        }
        result
    }
}
