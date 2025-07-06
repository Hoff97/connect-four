//! Checkers game implementation using the GameState trait
//! 
//! This module implements a complete checkers game with the following rules:
//! - Checkers can move diagonally forward one square
//! - Checkers can capture opponent pieces by jumping over them (exactly 2 squares diagonally)
//! - Multiple captures in a single move are allowed by consecutive jumps
//! - All captures are mandatory (if a capture is possible, it must be taken)
//! - Checkers promote to queens when reaching the opposite end
//! - Queens can move diagonally in any direction
//! - Game ends when: one player has no pieces, no captures for 50 moves, or no moves available
//!
//! ## Action Format
//! 
//! Actions are represented as `Vec<(usize, usize, usize, usize)>` where each tuple represents
//! a single move from (from_row, from_col) to (to_row, to_col). For regular moves, the vector
//! contains one element. For multiple captures, the vector contains multiple elements representing
//! the sequence of jumps.

use crate::minmax::{GameState, Player};
use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CheckersTile {
    Empty,
    Checker(Player),
    Queen(Player),
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct CheckersBoard {
    pub board: [[CheckersTile; 8]; 8],
    pub current_player: Player,
    pub moves_without_capture: u32,
}

impl Debug for CheckersBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.board.iter().rev() {
            for tile in row {
                let symbol = match tile {
                    CheckersTile::Empty => ".",
                    CheckersTile::Checker(Player::Player1) => "X",
                    CheckersTile::Checker(Player::Player2) => "O",
                    CheckersTile::Queen(Player::Player1) => "Q",
                    CheckersTile::Queen(Player::Player2) => "q",
                };
                write!(f, "{} ", symbol)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "Current Player: {:?}", self.current_player)?;
        writeln!(f, "Moves without capture: {}", self.moves_without_capture)?;
        Ok(())
    }
}

impl CheckersBoard {
    pub fn new() -> Self {
        let mut board = [[CheckersTile::Empty; 8]; 8];
        for row in 0..8 {
            for col in ((row % 2)..8).step_by(2) {
                if row < 3 {
                    board[row][col] = CheckersTile::Checker(Player::Player1);
                } else if row > 4 {
                    board[row][col] = CheckersTile::Checker(Player::Player2);
                }
            }
        }
        CheckersBoard { board, current_player: Player::Player1, moves_without_capture: 0 }
    }

    fn is_valid_position(&self, row: usize, col: usize) -> bool {
        row < 8 && col < 8
    }

    fn get_piece_at(&self, row: usize, col: usize) -> CheckersTile {
        if self.is_valid_position(row, col) {
            self.board[row][col]
        } else {
            CheckersTile::Empty
        }
    }

    fn get_moves_for_piece(&self, row: usize, col: usize) -> Vec<Vec<(usize, usize, usize, usize)>> {
        let mut moves = Vec::new();
        let piece = self.get_piece_at(row, col);
        
        match piece {
            CheckersTile::Checker(player) | CheckersTile::Queen(player) => {
                if player != self.current_player {
                    return moves;
                }
                
                // Get capture moves first (mandatory)
                let capture_moves = self.get_capture_moves_for_piece(row, col);
                if !capture_moves.is_empty() {
                    return capture_moves;
                }
                
                // If no captures, get regular moves
                let regular_moves = self.get_regular_moves_for_piece(row, col);
                for mov in regular_moves {
                    moves.push(vec![mov]); // Single move as a sequence
                }
            }
            _ => {}
        }
        
        moves
    }

    fn get_regular_moves_for_piece(&self, row: usize, col: usize) -> Vec<(usize, usize, usize, usize)> {
        let mut moves = Vec::new();
        let piece = self.get_piece_at(row, col);
        
        match piece {
            CheckersTile::Checker(player) => {
                let directions = if player == Player::Player1 {
                    vec![(1, -1), (1, 1)] // Forward diagonals for Player1
                } else {
                    vec![(-1, -1), (-1, 1)] // Forward diagonals for Player2
                };
                
                for (dr, dc) in directions {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;
                    
                    if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                        let new_row = new_row as usize;
                        let new_col = new_col as usize;
                        
                        if self.get_piece_at(new_row, new_col) == CheckersTile::Empty {
                            moves.push((row, col, new_row, new_col));
                        }
                    }
                }
            }
            CheckersTile::Queen(_) => {
                // Queens can move in all diagonal directions
                let directions = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];
                
                for (dr, dc) in directions {
                    let new_row = row as i32 + dr;
                    let new_col = col as i32 + dc;
                    
                    if new_row >= 0 && new_row < 8 && new_col >= 0 && new_col < 8 {
                        let new_row_u = new_row as usize;
                        let new_col_u = new_col as usize;
                        
                        if self.get_piece_at(new_row_u, new_col_u) == CheckersTile::Empty {
                            moves.push((row, col, new_row_u, new_col_u));
                        }
                    }
                }
            }
            _ => {}
        }
        
        moves
    }

    fn get_capture_moves_for_piece(&self, row: usize, col: usize) -> Vec<Vec<(usize, usize, usize, usize)>> {
        let mut all_capture_sequences = Vec::new();
        self.find_all_capture_sequences(row, col, vec![], &mut all_capture_sequences);
        all_capture_sequences
    }

    fn find_all_capture_sequences(&self, row: usize, col: usize, current_sequence: Vec<(usize, usize, usize, usize)>, all_sequences: &mut Vec<Vec<(usize, usize, usize, usize)>>) {
        let piece = self.get_piece_at(row, col);
        let mut found_capture = false;
        
        match piece {
            CheckersTile::Checker(player) => {
                let directions = if player == Player::Player1 {
                    vec![(1, -1), (1, 1)] // Forward diagonals for Player1
                } else {
                    vec![(-1, -1), (-1, 1)] // Forward diagonals for Player2
                };
                
                for (dr, dc) in directions {
                    if let Some(capture_move) = self.find_single_capture(row, col, dr, dc) {
                        found_capture = true;
                        let (_, _, to_row, to_col) = capture_move;
                        
                        // Create a new sequence with this capture
                        let mut new_sequence = current_sequence.clone();
                        new_sequence.push(capture_move);
                        
                        // Simulate the board state after this capture
                        let temp_board = self.apply_capture_sequence(&new_sequence);
                        
                        // Look for more captures from the new position
                        temp_board.find_all_capture_sequences(to_row, to_col, new_sequence, all_sequences);
                    }
                }
            }
            CheckersTile::Queen(_) => {
                let directions = vec![(-1, -1), (-1, 1), (1, -1), (1, 1)];
                
                for (dr, dc) in directions {
                    if let Some(capture_move) = self.find_single_capture(row, col, dr, dc) {
                        found_capture = true;
                        let (_, _, to_row, to_col) = capture_move;
                        
                        // Create a new sequence with this capture
                        let mut new_sequence = current_sequence.clone();
                        new_sequence.push(capture_move);
                        
                        // Simulate the board state after this capture
                        let temp_board = self.apply_capture_sequence(&new_sequence);
                        
                        // Look for more captures from the new position
                        temp_board.find_all_capture_sequences(to_row, to_col, new_sequence, all_sequences);
                    }
                }
            }
            _ => {}
        }
        
        // If we found no more captures and we have a sequence, add it to results
        if !found_capture && !current_sequence.is_empty() {
            all_sequences.push(current_sequence);
        }
    }

    fn find_single_capture(&self, row: usize, col: usize, dr: i32, dc: i32) -> Option<(usize, usize, usize, usize)> {
        let opponent_row = row as i32 + dr;
        let opponent_col = col as i32 + dc;
        
        // Check if opponent position is valid
        if opponent_row < 0 || opponent_row >= 8 || opponent_col < 0 || opponent_col >= 8 {
            return None;
        }
        
        let opponent_row = opponent_row as usize;
        let opponent_col = opponent_col as usize;
        
        // Check if there's an opponent piece at this position
        match self.get_piece_at(opponent_row, opponent_col) {
            CheckersTile::Checker(player) | CheckersTile::Queen(player) if player != self.current_player => {
                // Found opponent piece, check landing position
                let landing_row = opponent_row as i32 + dr;
                let landing_col = opponent_col as i32 + dc;
                
                if landing_row >= 0 && landing_row < 8 && landing_col >= 0 && landing_col < 8 {
                    let landing_row = landing_row as usize;
                    let landing_col = landing_col as usize;
                    
                    // Check if landing position is empty
                    if self.get_piece_at(landing_row, landing_col) == CheckersTile::Empty {
                        return Some((row, col, landing_row, landing_col));
                    }
                }
            }
            _ => {}
        }
        
        None
    }

    fn apply_capture_sequence(&self, sequence: &[(usize, usize, usize, usize)]) -> CheckersBoard {
        let mut board = self.clone();
        
        for &(from_row, from_col, to_row, to_col) in sequence {
            // Move the piece
            let piece = board.board[from_row][from_col];
            board.board[to_row][to_col] = piece;
            board.board[from_row][from_col] = CheckersTile::Empty;
            
            // Remove captured piece (always exactly one square diagonally between from and to)
            let captured_row = (from_row + to_row) / 2;
            let captured_col = (from_col + to_col) / 2;
            board.board[captured_row][captured_col] = CheckersTile::Empty;
            
            // Check for promotion
            match piece {
                CheckersTile::Checker(Player::Player1) if to_row == 7 => {
                    board.board[to_row][to_col] = CheckersTile::Queen(Player::Player1);
                }
                CheckersTile::Checker(Player::Player2) if to_row == 0 => {
                    board.board[to_row][to_col] = CheckersTile::Queen(Player::Player2);
                }
                _ => {}
            }
        }
        
        board
    }

    pub fn count_pieces(&self, player: Player) -> u32 {
        let mut count = 0;
        for row in 0..8 {
            for col in 0..8 {
                match self.board[row][col] {
                    CheckersTile::Checker(p) | CheckersTile::Queen(p) if p == player => count += 1,
                    _ => {}
                }
            }
        }
        count
    }

    fn has_any_moves(&self, player: Player) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                match self.board[row][col] {
                    CheckersTile::Checker(p) | CheckersTile::Queen(p) if p == player => {
                        if !self.get_moves_for_piece(row, col).is_empty() {
                            return true;
                        }
                    }
                    _ => {}
                }
            }
        }
        false
    }
}

impl GameState for CheckersBoard {
    type Action = Vec<(usize, usize, usize, usize)>; // Multiple jumps: from_row, from_col, to_row, to_col

    fn get_possible_actions(&self) -> Vec<Self::Action> {
        let mut all_moves = Vec::new();
        let mut capture_moves = Vec::new();
        
        // Check all pieces for the current player
        for row in 0..8 {
            for col in 0..8 {
                match self.board[row][col] {
                    CheckersTile::Checker(player) | CheckersTile::Queen(player) if player == self.current_player => {
                        let moves = self.get_moves_for_piece(row, col);
                        for mov in moves {
                            // Check if this is a capture sequence
                            if mov.len() > 1 || (mov.len() == 1 && {
                                let (from_row, from_col, to_row, to_col) = mov[0];
                                let row_diff = (to_row as i32 - from_row as i32).abs();
                                let col_diff = (to_col as i32 - from_col as i32).abs();
                                row_diff == 2 && col_diff == 2 // Capture move
                            }) {
                                capture_moves.push(mov);
                            } else {
                                all_moves.push(mov);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        
        // If there are capture moves, they are mandatory
        if !capture_moves.is_empty() {
            capture_moves
        } else {
            all_moves
        }
    }

    fn apply_action(&self, action: &Self::Action) -> Self {
        if action.is_empty() {
            return self.clone();
        }
        
        let mut new_board = self.clone();
        let mut captured_any = false;
        
        for &(from_row, from_col, to_row, to_col) in action {
            // Move the piece
            let piece = new_board.board[from_row][from_col];
            new_board.board[to_row][to_col] = piece;
            new_board.board[from_row][from_col] = CheckersTile::Empty;
            
            // Check if this is a capture move
            let row_diff = (to_row as i32 - from_row as i32).abs();
            let col_diff = (to_col as i32 - from_col as i32).abs();
            
            if row_diff == 2 && col_diff == 2 {
                // This is a capture move - remove the captured piece
                let captured_row = (from_row + to_row) / 2;
                let captured_col = (from_col + to_col) / 2;
                new_board.board[captured_row][captured_col] = CheckersTile::Empty;
                captured_any = true;
            }
            
            // Check for promotion
            match piece {
                CheckersTile::Checker(Player::Player1) if to_row == 7 => {
                    new_board.board[to_row][to_col] = CheckersTile::Queen(Player::Player1);
                }
                CheckersTile::Checker(Player::Player2) if to_row == 0 => {
                    new_board.board[to_row][to_col] = CheckersTile::Queen(Player::Player2);
                }
                _ => {}
            }
        }
        
        // Update move counter
        if captured_any {
            new_board.moves_without_capture = 0;
        } else {
            new_board.moves_without_capture += 1;
        }
        
        new_board.current_player = new_board.current_player.switch();
        new_board
    }

    fn is_terminal(&self) -> bool {
        // Game is over if:
        // 1. One player has no pieces left
        // 2. No captures for more than 50 moves
        // 3. Current player has no moves
        
        let player1_pieces = self.count_pieces(Player::Player1);
        let player2_pieces = self.count_pieces(Player::Player2);
        
        if player1_pieces == 0 || player2_pieces == 0 {
            return true;
        }
        
        if self.moves_without_capture >= 50 {
            return true;
        }
        
        if !self.has_any_moves(self.current_player) {
            return true;
        }
        
        false
    }

    fn evaluate(&self) -> Option<f32> {
        if self.is_terminal() {
            let player1_pieces = self.count_pieces(Player::Player1);
            let player2_pieces = self.count_pieces(Player::Player2);
            
            if player1_pieces == 0 {
                return Some(-1000.0); // Player 2 wins
            }
            if player2_pieces == 0 {
                return Some(1000.0); // Player 1 wins
            }
            if self.moves_without_capture >= 50 || !self.has_any_moves(self.current_player) {
                return Some(0.0); // Draw
            }
        }
        
        // Heuristic evaluation based on piece count and position
        let mut score = 0.0;
        
        for row in 0..8 {
            for col in 0..8 {
                match self.board[row][col] {
                    CheckersTile::Checker(Player::Player1) => {
                        score += 10.0;
                        // Bonus for advancing pieces
                        score += row as f32 * 0.5;
                    }
                    CheckersTile::Checker(Player::Player2) => {
                        score -= 10.0;
                        // Bonus for advancing pieces
                        score -= (7 - row) as f32 * 0.5;
                    }
                    CheckersTile::Queen(Player::Player1) => {
                        score += 30.0; // Queens are more valuable
                    }
                    CheckersTile::Queen(Player::Player2) => {
                        score -= 30.0;
                    }
                    _ => {}
                }
            }
        }
        
        Some(score)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkers_initialization() {
        let board = CheckersBoard::new();
        assert_eq!(board.current_player, Player::Player1);
        assert_eq!(board.moves_without_capture, 0);
        
        // Check that Player1 pieces are in the first 3 rows
        let mut player1_count = 0;
        let mut player2_count = 0;
        
        for row in 0..8 {
            for col in 0..8 {
                match board.board[row][col] {
                    CheckersTile::Checker(Player::Player1) => player1_count += 1,
                    CheckersTile::Checker(Player::Player2) => player2_count += 1,
                    _ => {}
                }
            }
        }
        
        assert_eq!(player1_count, 12);
        assert_eq!(player2_count, 12);
    }

    #[test]
    fn test_get_possible_actions() {
        let board = CheckersBoard::new();
        let actions = board.get_possible_actions();
        
        // At the start, Player1 should have 7 possible moves (4 edge pieces can move)
        assert_eq!(actions.len(), 7);
        
        // All initial moves should be forward diagonal moves
        for action in actions {
            assert_eq!(action.len(), 1); // Single move actions
            let (from_row, from_col, to_row, to_col) = action[0];
            assert!(to_row == from_row + 1);
            assert!((to_col as i32 - from_col as i32).abs() == 1);
        }
    }

    #[test]
    fn test_terminal_conditions() {
        let board = CheckersBoard::new();
        assert!(!board.is_terminal()); // Game should not be terminal at start
        
        // Test 50 move rule
        let mut long_game = board.clone();
        long_game.moves_without_capture = 50;
        assert!(long_game.is_terminal());
    }

    #[test]
    fn test_game_flow() {
        let mut board = CheckersBoard::new();
        
        // Get initial actions for Player1
        let actions = board.get_possible_actions();
        assert!(!actions.is_empty());
        
        // Make a move
        let first_action = &actions[0];
        board = board.apply_action(first_action);
        
        // Now it should be Player2's turn
        assert_eq!(board.current_player, Player::Player2);
        
        // Player2 should also have moves
        let player2_actions = board.get_possible_actions();
        assert!(!player2_actions.is_empty());
        
        // Test evaluation
        let evaluation = board.evaluate();
        assert!(evaluation.is_some());
        
        // Should be close to 0 at the start (balanced)
        let eval_value = evaluation.unwrap();
        assert!(eval_value.abs() < 50.0); // Should be reasonably balanced
    }

    #[test]
    fn test_piece_promotion() {
        // Create a board with a piece near promotion
        let mut board = CheckersBoard::new();
        
        // Manually set up a scenario where a piece can be promoted
        board.board[6][1] = CheckersTile::Checker(Player::Player1);
        board.board[7][2] = CheckersTile::Empty;
        
        // Apply a move that should promote the piece
        let action = vec![(6, 1, 7, 2)];
        let promoted_board = board.apply_action(&action);
        
        // Check that the piece was promoted to a queen
        assert_eq!(promoted_board.board[7][2], CheckersTile::Queen(Player::Player1));
    }

    #[test]
    fn test_multiple_captures() {
        let mut board = CheckersBoard::new();
        
        // Set up a scenario with multiple captures possible
        board.board[2][1] = CheckersTile::Checker(Player::Player1);
        board.board[3][2] = CheckersTile::Checker(Player::Player2);
        board.board[5][4] = CheckersTile::Checker(Player::Player2);
        board.board[4][3] = CheckersTile::Empty;
        board.board[6][5] = CheckersTile::Empty;
        
        // Player1 should be able to capture both pieces in one move
        let actions = board.get_possible_actions();
        
        // Look for a multi-capture sequence
        let multi_capture = actions.iter().find(|action| action.len() > 1);
        assert!(multi_capture.is_some(), "Should find a multi-capture sequence");
        
        if let Some(sequence) = multi_capture {
            // Apply the multi-capture
            let new_board = board.apply_action(sequence);
            
            // Both opponent pieces should be removed
            assert_eq!(new_board.board[3][2], CheckersTile::Empty);
            assert_eq!(new_board.board[5][4], CheckersTile::Empty);
            
            // The capturing piece should be at the final position
            let final_move = sequence.last().unwrap();
            let (_, _, final_row, final_col) = *final_move;
            assert_eq!(new_board.board[final_row][final_col], CheckersTile::Checker(Player::Player1));
        }
    }

    #[test]
    fn test_single_capture() {
        let mut board = CheckersBoard::new();
        
        // Set up a simple capture scenario
        board.board[2][1] = CheckersTile::Checker(Player::Player1);
        board.board[3][2] = CheckersTile::Checker(Player::Player2);
        board.board[4][3] = CheckersTile::Empty;
        
        // Clear other pieces to avoid interference
        for row in 0..8 {
            for col in 0..8 {
                if (row, col) != (2, 1) && (row, col) != (3, 2) {
                    board.board[row][col] = CheckersTile::Empty;
                }
            }
        }
        
        let actions = board.get_possible_actions();
        
        // Should have exactly one capture action
        assert_eq!(actions.len(), 1);
        let capture_action = &actions[0];
        assert_eq!(capture_action.len(), 1); // Single capture
        
        let (from_row, from_col, to_row, to_col) = capture_action[0];
        assert_eq!(from_row, 2);
        assert_eq!(from_col, 1);
        assert_eq!(to_row, 4);
        assert_eq!(to_col, 3);
        
        // Apply the capture
        let new_board = board.apply_action(capture_action);
        
        // Check that the opponent piece was captured
        assert_eq!(new_board.board[3][2], CheckersTile::Empty);
        assert_eq!(new_board.board[4][3], CheckersTile::Checker(Player::Player1));
        assert_eq!(new_board.board[2][1], CheckersTile::Empty);
    }
}