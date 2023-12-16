#!/bin/bash

function exit_me() {
	echo "Directory exists, exiting"
	exit -1
}

echo "Preparing day $1"

mkdir -v src/bin/day$1 || exit_me

cat -v <<EOF > src/bin/day$1/main.rs
use std::fs;

fn main() {
    println!("AOC 2023 Day $1");

    let contents = fs::read_to_string("src/bin/day$1/input.txt").expect("Failed to read input");
}
EOF

mv -v ~/Downloads/input src/bin/day$1/input.txt

echo "Done"
