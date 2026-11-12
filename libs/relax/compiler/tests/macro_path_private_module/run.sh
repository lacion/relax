#!/bin/bash
# usage: run.sh <makepad binary>: an import of a name that is both a private module and an
# exported macro of another crate binds the macro alone, so the name stays that crate in paths;
# prints "macro_path_private_module: ok" or "macro_path_private_module: FAILED" (exit 1).
H=$(cd "$(dirname "$0")" && pwd)
export MAKEPAD_RUN_CACHE=off
ERR=$(mktemp)
OUT=$(cd "$H" && MAKEPAD_RUN_STD=$H/../../std "$1" run --quiet path-app 2>"$ERR")
echo "$OUT"
if [ "$OUT" = "private module path ok 7 42 1" ]; then rm -f "$ERR"; echo "macro_path_private_module: ok"; else tail -5 "$ERR"; rm -f "$ERR"; echo "macro_path_private_module: FAILED"; exit 1; fi
