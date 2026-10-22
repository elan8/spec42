#!/usr/bin/env bash
# Draft the CHANGELOG.md section for the next release from the commits since the last release.
#
# Commits carry their user-visible note as git trailers (see AGENTS.md):
#   Changelog: <one user-facing sentence>
#   Breaking:  <what a consumer must change>
# This prints the Breaking and Changelog trailers, then the subjects of the remaining commits for
# triage. Release preparation edits the result into the `## [x.y.z]` section; feature and fix
# changes never edit CHANGELOG.md.
#
# Usage: scripts/changelog-draft.sh [<since-ref>]   (default: the latest v* tag)
set -euo pipefail

since="${1:-$(git describe --tags --abbrev=0 --match 'v[0-9]*')}"
range="${since}..HEAD"

# One line per trailer value, as a list item, oldest commit first.
trailer_items() {
  git log --no-merges --reverse --format="%(trailers:key=$1,valueonly,unfold,separator=%x1e)" "$range" |
    tr '\036' '\n' | sed '/^[[:space:]]*$/d; s/^[[:space:]]*/- /'
}

# Subjects of the commits that carry neither trailer, so nothing user-visible is missed.
other_items() {
  git log --no-merges --reverse \
    --format='%(trailers:key=Changelog,valueonly,separator=%x20)%(trailers:key=Breaking,valueonly,separator=%x20)%x1f%s (%h)' \
    "$range" |
    awk -F'\037' '$1 == "" { print "- " $2 }'
}

print_section() {
  local title="$1" items="$2"
  if [ -n "$items" ]; then
    printf '### %s\n\n%s\n\n' "$title" "$items"
  fi
}

printf '<!-- Draft from %s; edit into the release section of CHANGELOG.md. -->\n\n' "$range"
print_section "Breaking" "$(trailer_items Breaking)"
print_section "Changes" "$(trailer_items Changelog)"
print_section "Other commits (triage)" "$(other_items)"
