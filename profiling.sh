#!/bin/sh
#https://shunyata.github.io/2015/10/01/profiling-rust/

cargo build
valgrind --tool=callgrind target/debug/ruga
callgrind_annotate callgrind.out.* | less
rm callgrind.out.*
