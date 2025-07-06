mod four;
mod minmax;
mod checkers;

use crate::minmax::{GameResult, GameState, Player};

fn main() {
    let mut state = checkers::CheckersBoard::new();

    while !state.is_terminal() {
        println!("{:?}", state);
        println!("Current Player: {:?}", state.current_player);

        if state.current_player == Player::Player1 {
            println!("AI 1 is thinking...");
            let mut explored_states = std::collections::HashMap::new();
            let game_tree = minmax::minmax(
                state,
                15,
                true,
                &mut explored_states,
                f32::NEG_INFINITY,
                f32::INFINITY,
            );
            state = game_tree
                .best_child(true, &explored_states)
                .expect("No valid moves found")
                .clone();
            println!("Evaluation: {}", game_tree.evaluation);
        } else {
            println!("AI 2 is thinking...");
            let mut explored_states = std::collections::HashMap::new();
            let game_tree = minmax::minmax(
                state,
                15,
                false,
                &mut explored_states,
                f32::NEG_INFINITY,
                f32::INFINITY,
            );
            state = game_tree
                .best_child(false, &explored_states)
                .expect("No valid moves found")
                .clone();
            println!("Evaluation: {}", game_tree.evaluation);
        }
    }

    println!("{:?}", state);
}
