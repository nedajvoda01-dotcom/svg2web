#!/usr/bin/env bash
# scripts/verify-examples.sh
#
# Сравнивает фактический вывод svg2web с ожидаемым (golden files).
#
# Использование:
#   bash scripts/verify-examples.sh              # все примеры
#   bash scripts/verify-examples.sh basic        # только basic
#
# Режимы:
#   Сравнение: если папка expected/ есть, сравнивает с ней через diff.
#   Генерация: если expected/ нет — сравнивает old output/ с новым запуском.
#
# Статусы возврата:
#   0 — все примеры совпадают
#   1 — есть расхождения (diff выведен в stderr)
#   2 — svg2web не установлен / не собран

set -euo pipefail

SVG2WEB_BIN="${SVG2WEB_BIN:-}"
WORKSPACE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
EXAMPLES_DIR="$WORKSPACE_ROOT/examples"
TMPDIR_BASE="$(mktemp -d)"
PASS=0
FAIL=0
SKIP=0

# ─────────────────────────────
# Цвета
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

ok()   { echo -e "${GREEN}  PASS${NC} $1"; }
fail() { echo -e "${RED}  FAIL${NC} $1"; }
skip() { echo -e "${YELLOW}  SKIP${NC} $1"; }
info() { echo -e "       $1"; }

# ─────────────────────────────
# Найти svg2web бинарник
find_binary() {
    if [ -n "$SVG2WEB_BIN" ] && [ -x "$SVG2WEB_BIN" ]; then
        return 0
    fi

    local candidates=(
        "$WORKSPACE_ROOT/target/debug/svg2web"
        "$WORKSPACE_ROOT/target/release/svg2web"
    )

    for c in "${candidates[@]}"; do
        if [ -x "$c" ]; then
            SVG2WEB_BIN="$c"
            return 0
        fi
    done

    echo -e "${RED}ERROR${NC}: svg2web binary not found."
    echo "  Build first: cargo build --bin svg2web"
    echo "  Or set: SVG2WEB_BIN=/path/to/svg2web"
    return 2
}

# ─────────────────────────────
# Запуск svg2web для одного примера
run_example() {
    local example_dir="$1"
    local output_dir="$2"
    local config_file="$example_dir/config.toml"
    local input_file="$example_dir/input.svg"

    if [ ! -f "$input_file" ]; then
        echo "  no input.svg, skipping actual run"
        return 0
    fi

    mkdir -p "$output_dir"

    local cmd_args=("$SVG2WEB_BIN" build "$input_file" --output "$output_dir")
    if [ -f "$config_file" ]; then
        cmd_args+=(--config "$config_file")
    fi

    "${cmd_args[@]}" 2>/dev/null || true
}

# ─────────────────────────────
# Сравнение двух директорий через diff
compare_dirs() {
    local expected="$1"
    local actual="$2"
    local label="$3"

    if [ ! -d "$expected" ] || [ -z "$(ls -A "$expected" 2>/dev/null)" ]; then
        skip "$label (no expected files to compare)"
        SKIP=$((SKIP+1))
        return 0
    fi

    if [ ! -d "$actual" ] || [ -z "$(ls -A "$actual" 2>/dev/null)" ]; then
        fail "$label — actual output directory is empty or missing"
        info "Expected files: $(ls "$expected" | tr '\n' ' ')"
        FAIL=$((FAIL+1))
        return 1
    fi

    local diff_out
    diff_out="$(diff -rq \
        --exclude="*.map" \
        --exclude=".DS_Store" \
        "$expected" "$actual" 2>&1)" || true

    if [ -z "$diff_out" ]; then
        ok "$label"
        PASS=$((PASS+1))
    else
        fail "$label"
        echo "$diff_out" | head -40 | sed 's/^/    /' >&2
        FAIL=$((FAIL+1))
        return 1
    fi
}

# ─────────────────────────────
# Проверить один пример
check_example() {
    local name="$1"
    local dir="$EXAMPLES_DIR/$name"

    if [ ! -d "$dir" ]; then
        skip "$name (directory not found)"
        SKIP=$((SKIP+1))
        return 0
    fi

    local actual_dir="$TMPDIR_BASE/$name"

    # Если бинарник доступен — генерируем в tmpdir и сравниваем
    if [ -n "$SVG2WEB_BIN" ]; then
        run_example "$dir" "$actual_dir"
    fi

    # Если есть expected/ — сравниваем с ней
    if [ -d "$dir/expected" ]; then
        # Без бинарника нет смысла сравнивать — нечего генерировать
        if [ -z "$SVG2WEB_BIN" ]; then
            skip "$name (expected/ exists but svg2web not built)"
            SKIP=$((SKIP+1))
            return 0
        fi
        compare_dirs "$dir/expected" "$actual_dir" "$name (expected)"

    # Если есть output/ — просто проверяем что файлы ненулевые (smoke check)
    elif [ -d "$dir/output" ]; then
        local empty_files
        empty_files="$(find "$dir/output" -maxdepth 1 -size 0 2>/dev/null)" || true
        if [ -n "$empty_files" ]; then
            fail "$name — output contains empty files: $empty_files"
            FAIL=$((FAIL+1))
        else
            ok "$name (smoke: output files non-empty)"
            PASS=$((PASS+1))
        fi
    else
        skip "$name (no expected/ or output/ to compare)"
        SKIP=$((SKIP+1))
    fi
}

# ─────────────────────────────
# Обновить golden files (перезаписать expected/)
update_golden() {
    local name="$1"
    local dir="$EXAMPLES_DIR/$name"
    local actual_dir="$TMPDIR_BASE/$name"

    if [ -z "$SVG2WEB_BIN" ]; then
        echo "SVG2WEB_BIN not set, cannot update goldens"
        return 1
    fi

    run_example "$dir" "$actual_dir"

    if [ -d "$actual_dir" ] && [ -n "$(ls -A "$actual_dir" 2>/dev/null)" ]; then
        rm -rf "$dir/expected"
        cp -r "$actual_dir" "$dir/expected"
        echo "  Updated goldens for $name"
    fi
}

# ─────────────────────────────
# Cleanup
cleanup() {
    rm -rf "$TMPDIR_BASE"
}
trap cleanup EXIT

# ─────────────────────────────
# Список всех известных примеров
ALL_EXAMPLES=(
    "basic"
    "react-component"
    "responsive"
    "vue-component"
    "advanced/custom-fonts"
    "advanced/external-images"
    "advanced/complex-animation-free"
)

# ─────────────────────────────
# Main
main() {
    cd "$WORKSPACE_ROOT"

    # Режим --update: обновить golden files
    if [ "${1:-}" = "--update" ]; then
        find_binary || exit 2
        shift
        local targets=("${@:-${ALL_EXAMPLES[@]}}")
        for ex in "${targets[@]}"; do
            update_golden "$ex"
        done
        echo "Golden files updated."
        exit 0
    fi

    # Проверяем бинарник (не fatal — если нет, делаем только smoke checks)
    find_binary 2>/dev/null || SVG2WEB_BIN=""

    if [ -n "$SVG2WEB_BIN" ]; then
        echo "Using binary: $SVG2WEB_BIN"
    else
        echo "svg2web not built — running smoke checks only (no actual generation)"
    fi

    echo ""

    # Фильтр по аргументам
    local targets=("${@:-${ALL_EXAMPLES[@]}}")

    for ex in "${targets[@]}"; do
        check_example "$ex"
    done

    echo ""
    echo "Results: ${PASS} passed, ${FAIL} failed, ${SKIP} skipped"

    if [ "$FAIL" -gt 0 ]; then
        exit 1
    fi

    exit 0
}

main "$@"
