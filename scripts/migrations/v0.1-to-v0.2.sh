#!/usr/bin/env sh
set -eu

if [ "$#" -lt 1 ]; then
  echo "usage: $0 <files...>" >&2
  exit 2
fi

for target in "$@"; do
  if [ ! -f "$target" ]; then
    echo "skip: $target is not a file" >&2
    continue
  fi

  tmp_file="${target}.tmp.svg2web"
  sed \
    -e 's/component_detection/components.detect/g' \
    -e 's/min_component_size/components.min_size/g' \
    -e 's/svg2web analyze/svg2web parse --analyze/g' \
    "$target" > "$tmp_file"
  mv "$tmp_file" "$target"
done
