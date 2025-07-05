mod four;
mod minmax;

use crate::minmax::GameState;

fn main() {
    let mut state = four::FourRow::new();

    while state.get_terminal() == four::GameResult::Ongoing {
        println!("{:?}", state);
        println!("Current Player: {:?}", state.current_player);

        if state.current_player == four::Player::Player1 {
            /*println!("Your turn, possible actions:");
            println!("{:?}", state.get_possible_actions().iter().map(|a| a.column).collect::<Vec<_>>());

            loop {
                let mut input = String::new();
                println!("Enter your action (column number to drop a tile): ");
                std::io::stdin().read_line(&mut input).expect("Failed to read line");
                let action: Result<u8, _> = input.trim().parse();

                if let Ok(action) = action {
                    let act = four::GameAction{column: action};
                    if state.get_possible_actions().contains(&act) {
                        state = state.apply_action(&act);
                        break;
                    } else {
                        println!("Invalid action, try again.");
                    }
                } else {
                    println!("Invalid input, please enter a number.");
                }
            }*/
            println!("AI 1 is thinking...");
            let mut explored_states = std::collections::HashMap::new();
            let game_tree = minmax::minmax(
                state,
                5,
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
                5,
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
    match state.get_terminal() {
        four::GameResult::Win(four::Player::Player1) => println!("AI 1 wins!"),
        four::GameResult::Win(four::Player::Player2) => println!("AI 2 wins!"),
        four::GameResult::Draw => println!("It's a draw!"),
        four::GameResult::Ongoing => println!("Game is still ongoing!"),
    }
}
