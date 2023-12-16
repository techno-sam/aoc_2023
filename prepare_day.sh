#!/bin/bash

function exit_me() {
	echo "Directory exists, exiting"
	exit -1
}

echo "Preparing day $1"

mkdir -v src/bin/day$1 || exit_me

cat -v <<EOF > src/bin/day$1/main.rs
fn main() {
    println!("AOC 2023 Day $1");
}
EOF

mv -v ~/Downloads/input src/bin/day$1/input.txt

echo "Done"
