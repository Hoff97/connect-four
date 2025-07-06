mod four;
mod minmax;
mod checkers;

use crate::minmax::{GameResult, GameState, Player};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::console;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct ConnectFourGame {
    state: four::FourRow,
    last_ai_evaluation: Option<f32>,
}

#[wasm_bindgen]
impl ConnectFourGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ConnectFourGame {
        ConnectFourGame {
            state: four::FourRow::new(),
            last_ai_evaluation: None,
        }
    }

    #[wasm_bindgen]
    pub fn get_board(&self) -> String {
        serde_json::to_string(&self.state.board).unwrap_or_else(|_| "[]".to_string())
    }

    #[wasm_bindgen]
    pub fn get_current_player(&self) -> u8 {
        match self.state.current_player {
            Player::Player1 => 1,
            Player::Player2 => 2,
        }
    }

    #[wasm_bindgen]
    pub fn get_possible_moves(&self) -> Vec<u8> {
        self.state
            .get_possible_actions()
            .iter()
            .map(|a| a.column)
            .collect()
    }

    #[wasm_bindgen]
    pub fn make_move(&mut self, column: u8) -> bool {
        let action = four::GameAction { column };
        if self.state.get_possible_actions().contains(&action) {
            self.state = self.state.apply_action(&action);
            true
        } else {
            false
        }
    }

    #[wasm_bindgen]
    pub fn get_game_result(&self) -> u8 {
        match self.state.get_terminal() {
            GameResult::Win(Player::Player1) => 1,
            GameResult::Win(Player::Player2) => 2,
            GameResult::Draw => 3,
            GameResult::Ongoing => 0,
        }
    }

    #[wasm_bindgen]
    pub fn get_ai_move(&mut self, depth: u8, ai_is_player1: bool) -> Option<u8> {
        log!("AI is thinking with depth {}...", depth);

        let mut explored_states = HashMap::new();
        let maximizing_player = ai_is_player1;

        let game_tree = minmax::minmax(
            self.state.clone(),
            depth,
            maximizing_player,
            &mut explored_states,
            f32::NEG_INFINITY,
            f32::INFINITY,
        );

        // Store the evaluation for later retrieval
        self.last_ai_evaluation = Some(game_tree.evaluation);

        if let Some(best_state) = game_tree.best_child(maximizing_player, &explored_states) {
            // Find which column was played by comparing the states
            for action in self.state.get_possible_actions() {
                let new_state = self.state.apply_action(&action);
                if new_state == *best_state {
                    log!(
                        "AI chose column {} with evaluation {}",
                        action.column,
                        game_tree.evaluation
                    );
                    return Some(action.column);
                }
            }
        }

        None
    }

    #[wasm_bindgen]
    pub fn get_last_ai_evaluation(&self) -> Option<f32> {
        self.last_ai_evaluation
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.state = four::FourRow::new();
        self.last_ai_evaluation = None;
    }

    #[wasm_bindgen]
    pub fn get_board_dimensions(&self) -> Vec<usize> {
        vec![four::BOARD_HEIGHT, four::BOARD_WIDTH]
    }
}

#[wasm_bindgen]
pub struct CheckersGame {
    state: checkers::CheckersBoard,
    last_ai_evaluation: Option<f32>,
}

#[wasm_bindgen]
impl CheckersGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> CheckersGame {
        CheckersGame {
            state: checkers::CheckersBoard::new(),
            last_ai_evaluation: None,
        }
    }

    #[wasm_bindgen]
    pub fn get_board(&self) -> String {
        // Convert board to JSON format for web display
        let mut board_json = Vec::new();
        for row in 0..8 {
            let mut row_data = Vec::new();
            for col in 0..8 {
                let tile = match self.state.board[row][col] {
                    checkers::CheckersTile::Empty => "Empty",
                    checkers::CheckersTile::Checker(Player::Player1) => "Player1",
                    checkers::CheckersTile::Checker(Player::Player2) => "Player2",
                    checkers::CheckersTile::Queen(Player::Player1) => "Queen1",
                    checkers::CheckersTile::Queen(Player::Player2) => "Queen2",
                };
                row_data.push(tile);
            }
            board_json.push(row_data);
        }
        serde_json::to_string(&board_json).unwrap_or_else(|_| "[]".to_string())
    }

    #[wasm_bindgen]
    pub fn get_current_player(&self) -> u8 {
        match self.state.current_player {
            Player::Player1 => 1,
            Player::Player2 => 2,
        }
    }

    #[wasm_bindgen]
    pub fn get_possible_moves(&self) -> String {
        let actions = self.state.get_possible_actions();
        serde_json::to_string(&actions).unwrap_or_else(|_| "[]".to_string())
    }

    #[wasm_bindgen]
    pub fn make_move(&mut self, moves_json: &str) -> bool {
        if let Ok(action) = serde_json::from_str::<Vec<(usize, usize, usize, usize)>>(moves_json) {
            let possible_actions = self.state.get_possible_actions();
            if possible_actions.contains(&action) {
                self.state = self.state.apply_action(&action);
                return true;
            }
        }
        false
    }

    #[wasm_bindgen]
    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    #[wasm_bindgen]
    pub fn get_game_result(&self) -> u8 {
        if !self.state.is_terminal() {
            return 0; // Game ongoing
        }

        let player1_pieces = self.state.count_pieces(Player::Player1);
        let player2_pieces = self.state.count_pieces(Player::Player2);

        if player1_pieces == 0 {
            return 2; // Player 2 wins
        }
        if player2_pieces == 0 {
            return 1; // Player 1 wins
        }

        // Draw (50 move rule or no moves)
        return 3;
    }

    #[wasm_bindgen]
    pub fn get_ai_move(&mut self, depth: u8, ai_is_player1: bool) -> String {
        log!("AI is thinking with depth {}...", depth);

        let mut explored_states = HashMap::new();
        let maximizing_player = ai_is_player1;

        let game_tree = minmax::minmax(
            self.state.clone(),
            depth,
            maximizing_player,
            &mut explored_states,
            f32::NEG_INFINITY,
            f32::INFINITY,
        );

        // Store the evaluation for later retrieval
        self.last_ai_evaluation = Some(game_tree.evaluation);

        if let Some(best_state) = game_tree.best_child(maximizing_player, &explored_states) {
            // Find which action was taken by comparing the states
            for action in self.state.get_possible_actions() {
                let new_state = self.state.apply_action(&action);
                if new_state == *best_state {
                    log!("AI chose move with evaluation {}", game_tree.evaluation);
                    return serde_json::to_string(&action).unwrap_or_else(|_| "[]".to_string());
                }
            }
        }

        "[]".to_string()
    }

    #[wasm_bindgen]
    pub fn get_last_ai_evaluation(&self) -> Option<f32> {
        self.last_ai_evaluation
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.state = checkers::CheckersBoard::new();
        self.last_ai_evaluation = None;
    }

    #[wasm_bindgen]
    pub fn get_moves_without_capture(&self) -> u32 {
        self.state.moves_without_capture
    }
}
