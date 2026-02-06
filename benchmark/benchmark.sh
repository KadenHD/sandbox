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
    run_in_dir "c" gcc -O3 -march=native -std=c11 main.c -o main
    run_in_dir "cpp" g++ -O3 -march=native -std=c++17 main.cpp -o main
    run_in_dir "cs" csc -optimize+ -nologo main.cs
}

run_benchmark() {
    local P="$1"
    run_in_dir "python" python main.py "$P"
    run_in_dir "rust" cargo run --release --quiet -- "$P"
    run_in_dir "javascript" node main.js "$P"
    run_in_dir "typescript" node main.js "$P"
    run_in_dir "java" java Main "$P"
    run_in_dir "php" php main.php "$P"
    run_in_dir "c" ./main "$P"
    run_in_dir "cpp" ./main "$P"
    run_in_dir "cs" ./main "$P"
    
}

N="${1:-100000000}"

if ! [[ "$N" =~ ^[0-9]+$ ]] || [ "$N" -lt 3000000 ]; then
    echo "Error: N must be an integer >= 3,000,000"
    exit 1
fi

echo "<-------START------->"
pre_install
run_benchmark "$N"
echo "<--------END-------->"
