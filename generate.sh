#!/usr/bin/env bash

# requires cargo-generate, install with cargo install cargo-generate
# Create cookie file, fill in with cookie from the network tools once logged in on adventofcode.com

[[ $# -eq 0 ]] && echo "Usage: ./generate.sh day"

echo "🦀 DAY $1"

DAY_NUMBER=$1
AOCYEAR=2022
URL=https://adventofcode.com/$AOCYEAR/day/$DAY_NUMBER
DAY_FORMATTED=$DAY_NUMBER

if (($DAY_FORMATTED < 10)); then
  DAY_FORMATTED="0$DAY_FORMATTED"
fi

cargo generate --path template --name day$DAY_FORMATTED

curl -f $URL/input -H "cookie: $(cat cookie)" > day$DAY_FORMATTED/input.txt 2> /dev/null
