#!/usr/bin/env bash
set -euo pipefail

for example in basic react-component vue-component responsive; do
  echo "Checking examples/$example"
  test -d "examples/$example"
done

echo "Examples skeleton validation passed"
