#!/usr/bin/env bash

# Create cookie file, fill in with cookie from the network tools once logged in on adventofcode.com

[[ $# -eq 0 ]] && echo "Usage: ./generate.sh day"

echo "🦀 DAY $1"

AOCYEAR=2022
URL=https://adventofcode.com/$AOCYEAR/day/$1

cargo generate --path template --name day$1

curl -f $URL/input -H "cookie: $(cat cookie)" > day$1/input.txt 2> /dev/null
