#!/bin/bash
# Benchmark runner script

echo "Running Connect Four Rust implementation benchmarks..."
echo "This will generate HTML reports in target/criterion/"
echo ""

# Run the benchmarks
cargo bench

echo ""
echo "Benchmarks completed!"
echo "HTML reports available in: target/criterion/"
echo ""
echo "To view results, open target/criterion/report/index.html in your browser"
