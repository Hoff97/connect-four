use std::fmt::Debug;

use crate::minmax::{GameState, Player, Tile, GameResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameAction {
    pub column: u8, // Column index where the disc is dropped
}

pub const BOARD_WIDTH: usize = 7;
pub const BOARD_HEIGHT: usize = 6;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FourRow {
    pub board: [[Tile; BOARD_WIDTH]; BOARD_HEIGHT],
    pub current_player: Player,
}

impl Debug for FourRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter().rev() {
            for tile in row {
                let symbol = match tile {
                    Tile::Empty => ".",
                    Tile::Player1 => "X",
                    Tile::Player2 => "O",
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }
        for col in 0..BOARD_WIDTH {
            write!(f, "{} ", col)?;
        }
        Ok(())
    }
}

impl FourRow {
    pub fn new() -> Self {
        FourRow {
            board: [[Tile::Empty; BOARD_WIDTH]; BOARD_HEIGHT],
            current_player: Player::Player1,
        }
    }

    pub fn heuristic(&self) -> f32 {
        let mut score = 0.0;

        // Column position scores
        let column_scores = [0.0005, 0.002, 0.004, 0.005, 0.004, 0.002, 0.0005];

        // Score for tile positions
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                match self.board[row][col] {
                    Tile::Player1 => score += column_scores[col],
                    Tile::Player2 => score -= column_scores[col],
                    Tile::Empty => {}
                }
            }
        }

        // Score for rows of 3
        score += self.count_rows_of_three();

        score
    }

    fn count_rows_of_three(&self) -> f32 {
        let mut score = 0.0;

        // Check horizontal rows of 3
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH - 2 {
                if let Some(player_score) =
                    self.check_three_in_line([(row, col), (row, col + 1), (row, col + 2)])
                {
                    score += player_score;
                }
            }
        }

        // Check vertical rows of 3
        for row in 0..BOARD_HEIGHT - 2 {
            for col in 0..BOARD_WIDTH {
                if let Some(player_score) =
                    self.check_three_in_line([(row, col), (row + 1, col), (row + 2, col)])
                {
                    score += player_score;
                }
            }
        }

        // Check diagonal rows of 3 (bottom-left to top-right)
        for row in 0..BOARD_HEIGHT - 2 {
            for col in 0..BOARD_WIDTH - 2 {
                if let Some(player_score) =
                    self.check_three_in_line([(row, col), (row + 1, col + 1), (row + 2, col + 2)])
                {
                    score += player_score;
                }
            }
        }

        // Check diagonal rows of 3 (top-left to bottom-right)
        for row in 2..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH - 2 {
                if let Some(player_score) =
                    self.check_three_in_line([(row, col), (row - 1, col + 1), (row - 2, col + 2)])
                {
                    score += player_score;
                }
            }
        }

        score
    }

    fn check_three_in_line(&self, positions: [(usize, usize); 3]) -> Option<f32> {
        let tiles: Vec<Tile> = positions
            .iter()
            .map(|&(row, col)| self.board[row][col])
            .collect();

        // Check if all three tiles are the same and not empty
        if tiles[0] != Tile::Empty && tiles[0] == tiles[1] && tiles[1] == tiles[2] {
            // Check how many sides this line of 3 can be extended on
            let extension_sides = self.count_extension_sides(&positions);

            if extension_sides > 0 {
                let score = if extension_sides == 2 { 0.08 } else { 0.05 };
                match tiles[0] {
                    Tile::Player1 => Some(score),
                    Tile::Player2 => Some(-score),
                    Tile::Empty => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn count_extension_sides(&self, positions: &[(usize, usize); 3]) -> usize {
        // Calculate the direction vector
        let (row_diff, col_diff) = (
            positions[1].0 as i32 - positions[0].0 as i32,
            positions[1].1 as i32 - positions[0].1 as i32,
        );

        // Check position before the first tile
        let before_pos = (
            positions[0].0 as i32 - row_diff,
            positions[0].1 as i32 - col_diff,
        );

        // Check position after the last tile
        let after_pos = (
            positions[2].0 as i32 + row_diff,
            positions[2].1 as i32 + col_diff,
        );

        let can_extend_before = self.is_valid_extension_position(before_pos);
        let can_extend_after = self.is_valid_extension_position(after_pos);

        (can_extend_before as usize) + (can_extend_after as usize)
    }

    fn is_valid_extension_position(&self, pos: (i32, i32)) -> bool {
        let (row, col) = pos;

        // Check if position is within bounds
        if row < 0 || row >= BOARD_HEIGHT as i32 || col < 0 || col >= BOARD_WIDTH as i32 {
            return false;
        }

        let row = row as usize;
        let col = col as usize;

        // Position must be empty
        if self.board[row][col] != Tile::Empty {
            return false;
        }

        true
    }

    pub fn get_terminal(&self) -> GameResult {
        // Check for a win condition
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                if self.board[row][col] == Tile::Empty {
                    continue;
                }
                let tile = self.board[row][col];

                // Check horizontal
                if col + 3 < BOARD_WIDTH
                    && tile == self.board[row][col + 1]
                    && tile == self.board[row][col + 2]
                    && tile == self.board[row][col + 3]
                {
                    return if tile == Tile::Player1 {
                        GameResult::Win(Player::Player1)
                    } else {
                        GameResult::Win(Player::Player2)
                    };
                }

                // Check vertical
                if row + 3 < BOARD_HEIGHT
                    && tile == self.board[row + 1][col]
                    && tile == self.board[row + 2][col]
                    && tile == self.board[row + 3][col]
                {
                    return if tile == Tile::Player1 {
                        GameResult::Win(Player::Player1)
                    } else {
                        GameResult::Win(Player::Player2)
                    };
                }

                // Check diagonal (bottom-left to top-right)
                if row + 3 < BOARD_HEIGHT
                    && col + 3 < BOARD_WIDTH
                    && tile == self.board[row + 1][col + 1]
                    && tile == self.board[row + 2][col + 2]
                    && tile == self.board[row + 3][col + 3]
                {
                    return if tile == Tile::Player1 {
                        GameResult::Win(Player::Player1)
                    } else {
                        GameResult::Win(Player::Player2)
                    };
                }

                // Check diagonal (top-left to bottom-right)
                if row >= 3
                    && col + 3 < BOARD_WIDTH
                    && tile == self.board[row - 1][col + 1]
                    && tile == self.board[row - 2][col + 2]
                    && tile == self.board[row - 3][col + 3]
                {
                    return if tile == Tile::Player1 {
                        GameResult::Win(Player::Player1)
                    } else {
                        GameResult::Win(Player::Player2)
                    };
                }
            }
        }

        // Check for a draw (no empty tiles left)
        if self
            .board
            .iter()
            .all(|row| row.iter().all(|&tile| tile != Tile::Empty))
        {
            return GameResult::Draw;
        }

        return GameResult::Ongoing;
    }
}

impl GameState for FourRow {
    type Action = GameAction;

    fn get_possible_actions(&self) -> Vec<Self::Action> {
        (0..BOARD_WIDTH)
            .filter(|&col| self.board[BOARD_HEIGHT - 1][col] == Tile::Empty)
            .map(|x| GameAction { column: x as u8 })
            .collect()
    }

    fn apply_action(&self, action: &Self::Action) -> Self {
        let mut new_board = self.board;
        for row in 0..BOARD_HEIGHT {
            if new_board[row][action.column as usize] == Tile::Empty {
                new_board[row][action.column as usize] = self.current_player.tile();
                break;
            }
        }
        FourRow {
            board: new_board,
            current_player: self.current_player.switch(),
        }
    }

    fn is_terminal(&self) -> bool {
        return self.get_terminal() != GameResult::Ongoing;
    }

    fn evaluate(&self) -> Option<f32> {
        match self.get_terminal() {
            GameResult::Win(Player::Player1) => return Some(100.0),
            GameResult::Win(Player::Player2) => return Some(-100.0),
            GameResult::Draw => return Some(0.0),
            GameResult::Ongoing => {}
        }
        // Game is still ongoing
        return Some(self.heuristic());
    }
}
