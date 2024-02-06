#!/bin/bash

# Run Clippy
echo "Running Clippy..."
cargo clippy -- -D warnings
if [ $? -ne 0 ]; then
    echo "Clippy found errors. Exiting..."
    exit 1
fi

# Run tests
echo "Running tests..."
cargo test
if [ $? -ne 0 ]; then
    echo "Tests failed. Exiting..."
    exit 1
fi

# Run the application
echo "Running the application..."
cargo run
if [ $? -ne 0 ]; then
    echo "Application failed. Exiting..."
    exit 1
fi
