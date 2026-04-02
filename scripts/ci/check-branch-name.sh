#!/usr/bin/env sh
set -eu

if [ "$#" -ne 1 ]; then
  echo "usage: $0 <branch-name>" >&2
  exit 2
fi

branch="$1"
pattern='^(feature|fix|docs)\/.+'

printf '%s\n' "$branch" | grep -Eq "$pattern"
