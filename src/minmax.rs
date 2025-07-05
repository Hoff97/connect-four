use std::{collections::HashMap, fmt::Debug, hash::Hash};

pub trait GameState {
    type Action;

    fn get_possible_actions(&self) -> Vec<Self::Action>;
    fn apply_action(&self, action: &Self::Action) -> Self;
    fn is_terminal(&self) -> bool;
    fn evaluate(&self) -> Option<f32>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct GameTree<T: GameState> {
    pub state: T,
    pub children: Vec<T>,
    pub evaluation: f32,
}

pub struct MainLineItem<'a, T: GameState> {
    pub state: &'a T,
    pub evaluation: f32,
}

impl<T: GameState + Eq + Hash> GameTree<T> {
    pub fn new(state: T) -> Self {
        GameTree {
            state,
            children: Vec::new(),
            evaluation: 0.0,
        }
    }

    pub fn get_children(&self) -> &Vec<T> {
        &self.children
    }

    pub fn get_evaluation(&self) -> f32 {
        self.evaluation
    }

    pub fn get_main_line<'a>(
        &'a self,
        explored_states: &'a HashMap<T, GameTree<T>>,
    ) -> Vec<MainLineItem<'a, T>> {
        let mut main_line = Vec::new();
        let current_node: &T = &self.state;
        main_line.push(MainLineItem {
            state: current_node,
            evaluation: explored_states
                .get(current_node)
                .map_or(0.0, |tree| tree.evaluation),
        });

        if self.children.is_empty() {
            return main_line;
        }

        for child in &self.children {
            if let Some(child_tree) = explored_states.get(child) {
                if child_tree.evaluation == self.evaluation {
                    main_line.extend(child_tree.get_main_line(explored_states));
                    break;
                }
            }
        }

        return main_line;
    }

    pub fn best_child(
        &self,
        maximizing_player: bool,
        explored_states: &HashMap<T, GameTree<T>>,
    ) -> Option<&T> {
        if self.children.is_empty() {
            return None;
        }

        let mut best_child = None;
        let mut best_evaluation = if maximizing_player {
            f32::NEG_INFINITY
        } else {
            f32::INFINITY
        };

        for child in &self.children {
            if let Some(child_tree) = explored_states.get(child) {
                if (maximizing_player && child_tree.evaluation > best_evaluation)
                    || (!maximizing_player && child_tree.evaluation < best_evaluation)
                {
                    best_evaluation = child_tree.evaluation;
                    best_child = Some(child);
                }
            }
        }

        best_child
    }
}

pub fn minmax<'a, T: GameState + Hash + Eq + Clone + Debug>(
    state: T,
    depth: u8,
    maximizing_player: bool,
    explored_states: &mut HashMap<T, GameTree<T>>,
    mut alpha: f32,
    mut beta: f32,
) -> GameTree<T> {
    // Min max algorithm with alpha-beta pruning
    let mut tree = GameTree {
        state: state,
        children: Vec::new(),
        evaluation: if maximizing_player {
            f32::NEG_INFINITY
        } else {
            f32::INFINITY
        },
    };

    if depth == 0 || tree.state.is_terminal() {
        tree.evaluation = tree.state.evaluate().unwrap_or(0.0);
        return tree;
    }

    let actions = tree.state.get_possible_actions();
    let mut next_states: Vec<_> = actions
        .iter()
        .map(|action| {
            let state = tree.state.apply_action(action);
            let eval = state.evaluate().unwrap_or(0.0);
            return (state, if maximizing_player { eval } else { -eval });
        })
        .collect();
    next_states.sort_by(|a, b| {
        a.1.partial_cmp(&b.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .reverse()
    });

    for (new_state, _) in next_states {
        if let Some(_) = explored_states.get(&new_state) {
            tree.children.push(new_state);
        } else {
            let child_tree = minmax(
                new_state,
                depth - 1,
                !maximizing_player,
                explored_states,
                alpha,
                beta,
            );
            tree.children.push(child_tree.state.clone());
            explored_states.insert(child_tree.state.clone(), child_tree);
        }

        tree.evaluation = if maximizing_player {
            tree.evaluation.max(
                explored_states
                    .get(tree.children.last().unwrap())
                    .unwrap()
                    .evaluation,
            )
        } else {
            tree.evaluation.min(
                explored_states
                    .get(tree.children.last().unwrap())
                    .unwrap()
                    .evaluation,
            )
        };

        if tree.evaluation >= beta && maximizing_player {
            break; // beta cutoff
        } else if tree.evaluation <= alpha && !maximizing_player {
            break; // alpha cutoff
        }
        if maximizing_player {
            alpha = alpha.max(tree.evaluation);
        } else {
            beta = beta.min(tree.evaluation);
        }
    }

    return tree;
}
