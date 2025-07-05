mod four;
mod minmax;

use crate::minmax::GameState;
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
            four::Player::Player1 => 1,
            four::Player::Player2 => 2,
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
            four::GameResult::Win(four::Player::Player1) => 1,
            four::GameResult::Win(four::Player::Player2) => 2,
            four::GameResult::Draw => 3,
            four::GameResult::Ongoing => 0,
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
