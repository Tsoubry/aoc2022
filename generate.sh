#!/usr/bin/env bash

[[ $# -eq 0 ]] && echo "Usage: ./generate.sh day"

echo "🦀 DAY $1"

cargo new --vcs none --bin --edition 2021 --name day$1 day$1


