#!/usr/bin/env sh
set -eu

if [ "$#" -ne 1 ]; then
  echo "usage: $0 <commit-message>" >&2
  exit 2
fi

message="$1"
pattern='^(feat|fix|docs|test|refactor|chore)(\(.+\))?: .+'

printf '%s\n' "$message" | grep -Eq "$pattern"
