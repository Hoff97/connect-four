use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use std::collections::HashMap;

// We need to include the modules since this is a benchmark
mod four {
    include!("../src/four.rs");
}

mod minmax {
    include!("../src/minmax.rs");
}

use four::{BOARD_WIDTH, FourRow, GameAction, GameResult, Player};
use minmax::{GameState, minmax};

fn create_empty_game() -> FourRow {
    FourRow::new()
}

fn create_early_game() -> FourRow {
    let mut game = FourRow::new();
    let moves = vec![3, 3, 2]; // Few opening moves

    for column in moves {
        let action = GameAction { column };
        game = game.apply_action(&action);
    }
    game
}

fn create_mid_game() -> FourRow {
    let mut game = FourRow::new();
    let moves = vec![3, 3, 2, 4, 2, 4, 1, 5, 1, 5, 0]; // More developed position

    for column in moves {
        let action = GameAction { column };
        game = game.apply_action(&action);
    }
    game
}

fn create_complex_game() -> FourRow {
    let mut game = FourRow::new();
    let moves = vec![3, 3, 2, 4, 2, 4, 1, 5, 1, 5, 0, 6, 0, 6, 3, 4, 2, 5]; // Complex position

    for column in moves {
        let action = GameAction { column };
        if game
            .get_possible_actions()
            .iter()
            .any(|a| a.column == column)
        {
            game = game.apply_action(&action);
        }
    }
    game
}

fn bench_game_creation(c: &mut Criterion) {
    c.bench_function("create_empty_game", |b| {
        b.iter(|| black_box(create_empty_game()))
    });
}

fn bench_move_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_generation");

    let empty_game = create_empty_game();
    let early_game = create_early_game();
    let mid_game = create_mid_game();
    let complex_game = create_complex_game();

    group.bench_with_input(
        BenchmarkId::new("empty_game", ""),
        &empty_game,
        |b, game| b.iter(|| black_box(game.get_possible_actions())),
    );

    group.bench_with_input(
        BenchmarkId::new("early_game", ""),
        &early_game,
        |b, game| b.iter(|| black_box(game.get_possible_actions())),
    );

    group.bench_with_input(BenchmarkId::new("mid_game", ""), &mid_game, |b, game| {
        b.iter(|| black_box(game.get_possible_actions()))
    });

    group.bench_with_input(
        BenchmarkId::new("complex_game", ""),
        &complex_game,
        |b, game| b.iter(|| black_box(game.get_possible_actions())),
    );

    group.finish();
}

fn bench_move_application(c: &mut Criterion) {
    let mut group = c.benchmark_group("move_application");

    let game = create_early_game();
    let action = GameAction { column: 4 };

    group.bench_function("apply_action", |b| {
        b.iter(|| black_box(game.apply_action(&action)))
    });

    group.finish();
}

fn bench_terminal_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("terminal_check");

    let empty_game = create_empty_game();
    let mid_game = create_mid_game();
    let complex_game = create_complex_game();

    group.bench_with_input(
        BenchmarkId::new("empty_game", ""),
        &empty_game,
        |b, game| b.iter(|| black_box(game.get_terminal())),
    );

    group.bench_with_input(BenchmarkId::new("mid_game", ""), &mid_game, |b, game| {
        b.iter(|| black_box(game.get_terminal()))
    });

    group.bench_with_input(
        BenchmarkId::new("complex_game", ""),
        &complex_game,
        |b, game| b.iter(|| black_box(game.get_terminal())),
    );

    group.finish();
}

fn bench_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("evaluation");

    let empty_game = create_empty_game();
    let mid_game = create_mid_game();
    let complex_game = create_complex_game();

    group.bench_with_input(
        BenchmarkId::new("empty_game", ""),
        &empty_game,
        |b, game| b.iter(|| black_box(game.evaluate())),
    );

    group.bench_with_input(BenchmarkId::new("mid_game", ""), &mid_game, |b, game| {
        b.iter(|| black_box(game.evaluate()))
    });

    group.bench_with_input(
        BenchmarkId::new("complex_game", ""),
        &complex_game,
        |b, game| b.iter(|| black_box(game.evaluate())),
    );

    group.finish();
}

fn bench_minimax_depth(c: &mut Criterion) {
    let mut group = c.benchmark_group("minimax_depth");

    let game = create_early_game();

    // Test different depths
    for depth in 1..=6 {
        group.bench_with_input(BenchmarkId::new("depth", depth), &depth, |b, &depth| {
            b.iter(|| {
                let mut explored_states = HashMap::new();
                black_box(minmax(
                    game.clone(),
                    depth,
                    true,
                    &mut explored_states,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                ))
            })
        });
    }

    group.bench_with_input(BenchmarkId::new("depth", 12), &12, |b, &depth| {
        b.iter(|| {
            let mut explored_states = HashMap::new();
            black_box(minmax(
                game.clone(),
                depth,
                true,
                &mut explored_states,
                f32::NEG_INFINITY,
                f32::INFINITY,
            ))
        })
    });

    group.finish();
}

fn bench_minimax_positions(c: &mut Criterion) {
    let mut group = c.benchmark_group("minimax_positions");

    let empty_game = create_empty_game();
    let early_game = create_early_game();
    let mid_game = create_mid_game();

    let depth = 4; // Fixed depth for position comparison

    group.bench_with_input(
        BenchmarkId::new("empty_game", depth),
        &empty_game,
        |b, game| {
            b.iter(|| {
                let mut explored_states = HashMap::new();
                black_box(minmax(
                    game.clone(),
                    depth,
                    true,
                    &mut explored_states,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                ))
            })
        },
    );

    group.bench_with_input(
        BenchmarkId::new("early_game", depth),
        &early_game,
        |b, game| {
            b.iter(|| {
                let mut explored_states = HashMap::new();
                black_box(minmax(
                    game.clone(),
                    depth,
                    true,
                    &mut explored_states,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                ))
            })
        },
    );

    group.bench_with_input(BenchmarkId::new("mid_game", depth), &mid_game, |b, game| {
        b.iter(|| {
            let mut explored_states = HashMap::new();
            black_box(minmax(
                game.clone(),
                depth,
                true,
                &mut explored_states,
                f32::NEG_INFINITY,
                f32::INFINITY,
            ))
        })
    });

    group.finish();
}

fn bench_game_clone(c: &mut Criterion) {
    let mut group = c.benchmark_group("game_clone");

    let game = create_complex_game();

    group.bench_function("clone_game", |b| b.iter(|| black_box(game.clone())));

    group.finish();
}

fn bench_hash_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_performance");

    let games = vec![
        create_empty_game(),
        create_early_game(),
        create_mid_game(),
        create_complex_game(),
    ];

    group.bench_function("hash_games", |b| {
        b.iter(|| {
            let mut map = HashMap::new();
            for (i, game) in games.iter().enumerate() {
                map.insert(black_box(game.clone()), i);
            }
            black_box(map)
        })
    });

    group.finish();
}

fn bench_full_game_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_game_simulation");

    group.bench_function("ai_vs_ai_depth_3", |b| {
        b.iter(|| {
            let mut game = create_empty_game();
            let mut move_count = 0;

            while game.get_terminal() == GameResult::Ongoing && move_count < 42 {
                let mut explored_states = HashMap::new();
                let is_maximizing = game.current_player == Player::Player1;

                let result = minmax(
                    game.clone(),
                    3,
                    is_maximizing,
                    &mut explored_states,
                    f32::NEG_INFINITY,
                    f32::INFINITY,
                );

                if let Some(best_state) = result.best_child(is_maximizing, &explored_states) {
                    game = best_state.clone();
                    move_count += 1;
                } else {
                    break;
                }
            }
            black_box(game)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_game_creation,
    bench_move_generation,
    bench_move_application,
    bench_terminal_check,
    bench_evaluation,
    bench_minimax_depth,
    bench_minimax_positions,
    bench_game_clone,
    bench_hash_performance,
    bench_full_game_simulation
);

criterion_main!(benches);
