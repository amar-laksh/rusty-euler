#!/bin/bash -
#===============================================================================
#
#          FILE: run.sh
#
#         USAGE: ./run.sh
#
#   DESCRIPTION: 
#
#       OPTIONS: ---
#  REQUIREMENTS: ---
#          BUGS: ---
#         NOTES: ---
#        AUTHOR: YOUR NAME (), 
#  ORGANIZATION: 
#       CREATED: Tuesday 04 December 2018 03:02:45  IST
#      REVISION:  ---
#===============================================================================



if [ $# -lt 1 ]
then
	echo "Usage: problem number"
	exit
fi

function run_problem() {
    cd "problem_$1" && cargo build --release -q
    cd ..
    START=$(($(date +%s%N)))
    actual_result=$("./problem_$1/target/release/problem_$1")
    expected_result=$(grep "Problem $1: .*" ./answers | cut -d ' ' -f3)
    if [[ "$actual_result" != "$expected_result"  ]]; then
        echo "problem $1 wrong result"
        echo "expected_result: $expected_result"
        echo "actual_result: $actual_result"
        exit
    fi
    echo -n "Problem $1: " && 
    END=$(($(date +%s%N)))
    echo "It took $(($END - $START)) nanoseconds"
}

if [ "$1" == "all" ]
then
    for n in $(ls | grep "problem_" | cut -d '_' -f2);do
        run_problem "$n"
    done
	exit
fi

run_problem "$1"
