# Performance Comparison Script

This script helps you compare the performance of different optimization strategies.

## Quick Benchmark Run

To run a quick benchmark:

```bash
cargo bench --bench game_benchmark
```

## Detailed Analysis

For detailed HTML reports:

```bash
cargo bench --bench game_benchmark
# Open target/criterion/report/index.html in your browser
```

## Key Metrics to Watch

1. **Minimax Depth Performance**: 
   - Depth 1-3: Should be microseconds
   - Depth 4-5: Should be under 1ms
   - Depth 6: Should be under 2ms

2. **Move Generation**: Should be consistent ~25ns regardless of position

3. **Position Evaluation**: Should be under 500ns for most positions

4. **Memory Operations**: Clone/hash should be under 20ns

## Optimization Strategies

### Current Implementation Strengths:
- Fast basic operations (move generation, application)
- Consistent performance across game states
- Efficient memory usage
- Good minimax scaling

### Potential Improvements:
- Bitboard representation for faster operations
- Better move ordering for alpha-beta pruning
- Transposition table optimizations
- Iterative deepening for better time management

## Running Custom Benchmarks

You can modify `benches/game_benchmark.rs` to test specific scenarios:

```rust
// Add custom game positions
fn create_custom_position() -> FourRow {
    let mut game = FourRow::new();
    // Add your specific moves here
    game
}

// Add custom benchmarks
group.bench_function("custom_benchmark", |b| {
    let game = create_custom_position();
    b.iter(|| {
        // Your benchmark code here
        black_box(game.evaluate())
    })
});
```
