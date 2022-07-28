#!/bin/bash

dirname=$(dirname $0)

$dirname/target/release/glium_test &

gol=$!

i3lock -c 00000000 -n

kill $gol
