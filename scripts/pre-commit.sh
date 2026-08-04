#!/usr/bin/env sh
set -eu

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

printf '%s\n' 'Running pre-commit checks...'

cargo fmt --all -- --check
scripts/check-class-usage.py --staged

has_staged_css=false
while IFS= read -r path; do
  case "$path" in
    preview/*.css|preview/**/*.css)
      has_staged_css=true
      break
      ;;
  esac
done <<EOF
$(git diff --cached --name-only --diff-filter=ACMR)
EOF

if [ "$has_staged_css" = true ]; then
  printf '%s\n' 'Checking staged preview CSS...'
  (
    cd preview
    npx stylelint "**/*.css" --max-warnings=0
  )
fi

printf '%s\n' 'Pre-commit checks passed.'
