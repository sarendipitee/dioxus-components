#!/usr/bin/env sh
set -eu

case "${1:-}" in
  build | serve)
    command="$1"
    shift
    ;;
  *)
    printf 'usage: %s <build|serve> [dx args...]\n' "$0" >&2
    exit 2
    ;;
esac

"$(dirname -- "$0")/clean-preview-dx-assets.sh"
exec dx "$command" -p preview --web "$@"
