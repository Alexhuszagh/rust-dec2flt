#!/bin/bash
# Test the binary sizes at different optimization levels.

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

# Delete our log if it exists and create a new, empty one.
mkdir -p log
if [ -f log/binsize.log ]; then
    rm log/binsize.log
fi
touch log/binsize.log
echo "opt-level,size,size(stripped)" >> log/binsize.log

# Create our binary levels.
levels=(
    "-O0"
    "-O1"
    "-O2"
    "-O3"
    "-Os"
    "-Oz"
)
for level in "${levels[@]}"; do
    # Determine if it's a debug or release builds.
    args="--bin dec2flt_example"
    case $level in
        -O2|-O3|-Os|-Oz)
        args="$args --release"
        dir="release"
        shift
        ;;
        *)
        dir="debug"
        # Debug build, do nothing.
        ;;
    esac

    scripts/manifest.sh $level
    cargo build $args 2>/dev/null
    size=$(ls -sh target/$dir/dec2flt_example | tr -s ' ' | cut -d ' ' -f 1)
    strip target/$dir/dec2flt_example
    stripped_size=$(ls -sh target/$dir/dec2flt_example | tr -s ' ' | cut -d ' ' -f 1)
    echo "${level:2:3},$size,$stripped_size" >> log/binsize.log
done
