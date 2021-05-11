#!/bin/bash
# Auto-generate the correct `Cargo.toml` profile.

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

# Match our command line arguments, and get our profile, and our
# optimization level.
case $1 in
    -O0|O0)
    profile="dev"
    opt_level="0"
    shift
    ;;
    -O1|O1)
    profile="dev"
    opt_level="1"
    shift
    ;;
    -O2|O2)
    profile="release"
    opt_level="2"
    shift
    ;;
    -O3|O3)
    profile="release"
    opt_level="3"
    shift
    ;;
    -Os|Os)
    profile="release"
    opt_level="\"s\""
    shift
    ;;
    -Oz|Oz)
    profile="release"
    opt_level="\"z\""
    shift
    ;;
    *)
    profile="dev"
    opt_level="0"
    ;;
esac

# Delete our old manifest file, copy our "Cargo.toml.in" file.
if [ -f Cargo.toml ]; then
    rm Cargo.toml
fi
cp scripts/Cargo.toml.in Cargo.toml

# Match our profile and add our optimization settings.
case $profile in
    dev)
    echo "[profile.dev]" >> Cargo.toml
    echo "opt-level = $opt_level" >> Cargo.toml
    echo "debug = true" >> Cargo.toml
    echo "lto = false" >> Cargo.toml
    shift
    ;;
    release)
    echo "[profile.release]" >> Cargo.toml
    echo "opt-level = $opt_level" >> Cargo.toml
    echo "debug = false" >> Cargo.toml
    echo "debug-assertions = false" >> Cargo.toml
    echo "lto = true" >> Cargo.toml
    shift
    ;;
    *)
    echo "Invalid profile, got $profile"
    exit 1
    ;;
esac
