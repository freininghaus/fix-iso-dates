#!/bin/bash

set -e
set -o pipefail

function run()
{
  args=$1
  input=$2
  expected=$3

  result=$(echo "$input" | cargo run -- $args)
  diff -u <(echo $result) <(echo $expected) || echo "$args, $input, $expected: $result" && exit 1
}

run "--min-date 2023-01-02" "2023-01-01 2023-01-02 2023-01-03 2023-01-02 2023-01-01" "2023-01-02 2023-01-02 2023-01-03 2023-01-02 2023-01-02"
