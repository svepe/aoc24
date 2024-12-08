#!/bin/sh

DAY=${1:?Day number is required}

YEAR=2024

[ -f "token" ] || { echo "Save your session cookie in a file named 'token'" && exit 1; }

curl "https://adventofcode.com/${YEAR}/day/${DAY:?}/input" \
    --cookie "session=$(cat token)" \
    --output day-$(printf "%02d" ${DAY})/input.txt
