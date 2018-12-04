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
cd "problem_$1" && cargo run --release && cd ..

