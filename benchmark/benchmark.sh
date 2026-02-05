#!/usr/bin/env bash

set -e

run_in_dir() {
    local dir="$1"
    shift                       # remove the first argument so $@ is the command
    pushd "$dir" > /dev/null    # cd into directory
    "$@"                        # run the command with all remaining args
    popd > /dev/null            # return to original directory
}

pre_install() {
    echo "Pre-install / build steps..."
    run_in_dir "rust" cargo build --release --quiet
    run_in_dir "typescript" npm install --silent
    run_in_dir "typescript" tsc main.ts
    run_in_dir "java" javac Main.java
}

run_benchmark() {
    N="$1"
    run_in_dir "python" python main.py "$N"
    run_in_dir "rust" cargo run --release --quiet -- "$N"
    run_in_dir "javascript" node main.js "$N"
    run_in_dir "typescript" node main.js "$N"
    run_in_dir "java" java Main "$N"
    run_in_dir "php" php main.php "$N"
}

if [ -z "$1" ]; then
    echo "Usage: $0 <N>"
    exit 1
fi

echo "<-------START------->"
pre_install
run_benchmark "$1"
echo "<--------END-------->"
