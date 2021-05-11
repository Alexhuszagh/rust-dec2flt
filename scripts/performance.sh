#!/bin/bash

# Leave early if not on nightly.
version=$(rustc -V)
if [[ "$version" != *"nightly"* ]]; then
    # Error, not on nightly
    >&2 echo "Error: rust-dec2flt must be run on nightly."
    exit 1
fi

# Check if we're not running from the project root.
config=.git/config
if [ ! -f "$config" ]; then
    >&2 echo "Error: script must be run from project root."
    exit 1
fi

# Only create our new manifest if it doesn't exist.
if [ ! -f Cargo.toml ]; then
    cp scripts/Cargo.toml.in Cargo.toml
fi

# Need to ensure we're using the performance governor.
# Requires sudo.
sudo cpupower frequency-set --governor performance

# Build our benches first.
cargo bench --no-run

# Delete our log if it exists and create a new, empty one.
mkdir -p log
if [ -f log/perf.log ]; then
    rm log/perf.log
fi
touch log/perf.log

# Now need to bench and extract lines.
output=$(cargo bench)
lines=$(echo "$output" | grep "time:")

# Now need to process the lines, to extract the times
for line in "${lines[@]}"; do
    line=$(echo "$line" | tr -s ' ')
    time=$(echo "$line" | cut -d ' ' -f 5,6 | sed "s/ //g")
    echo "$time" >> log/perf.log
done
