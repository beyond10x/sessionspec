#!/bin/sh
# Mutation audit of a small ESS specification of this repository (spec/ or studio/): would its
# conformance suite notice if a declared rule broke? `ess verify conform mutate` derives one
# mutant per declared rule (a dropped `from` state, a moved guard boundary, a dropped `sets`
# entry, a swapped error, ...), synthesizes each mutant's suite and runs it on the interpreter of
# the unchanged specification, with the authored scenarios of <dir>/ess-inputs.yaml run against
# the interpreter of each mutant. A mutant no scenario tells apart is a declared rule the suite
# does not pin down.
#
# <dir>/mutation-known.txt lists the mutants known not to be killed, one per line, the mutant id
# first and the reason after it. The run passes when the mutants that are neither killed,
# equivalent nor stillborn are exactly those: a new survivor, or a listed mutant that is now
# killed, fails it. A survivor is answered by an authored scenario under <dir>/scenarios/ (one
# that fails against the mutant's interpreter kills it) or by declaring what makes the rule
# observable, never by listing it without a reason.
#
# Every class runs, `grant-add` included. A synthesized suite sends each command as one actor it
# does not grant (`<command>/grant/denied`), so a grant added to another actor is told apart only
# by an authored scenario that sends the command as that actor and expects `refused: not_granted`
# (<dir>/scenarios/grants/).
#
# A site the audit does not mutate is not a failure, but its count is printed with every run so a
# drop in coverage shows: the stillborn mutants, the unmutated sites (`unmutated_sites`, by reason)
# and the unavailable ones (`unavailable_sites`).
#
# spec/ holds Binary64 fields (the costs and shares of the session view), which ESS 0.99.2's
# mutation audit refuses to carry. They are view, event and entity fields no guard reads, so the
# audit runs on a copy under target/mutation/ with Binary64 read as Decimal; every mutant of a
# rule is the same as on spec/ itself.
#
# Takes about 7 s for studio/ and 13 s for spec/.
#
#   check-mutation.sh <dir> [--known FILE] [--class CLASS]...
#
# `--class` audits only the classes named, against the lines of the known list in those classes;
# scripts/check-mutation.test.sh uses it to stay fast. The gate names none: every class runs.
#
# Exit 0: the non-killed mutants are exactly the listed ones. 1: they differ (each difference is
# named), or the suite failed on the unchanged specification. 2: a usage error, or the audit did
# not run.
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)

dir=
known=
only=
while [ $# -gt 0 ]; do
  case $1 in
    --known) known=$2; shift 2 ;;
    --class) only="$only $2"; shift 2 ;;
    -*) echo "check-mutation: unknown argument $1" >&2; exit 2 ;;
    *) dir=${1%/}; shift ;;
  esac
done
[ -n "$dir" ] && [ -f "$dir/ess-inputs.yaml" ] || {
  echo "check-mutation: usage: check-mutation.sh <dir with ess-inputs.yaml> [--known FILE] [--class CLASS]..." >&2
  exit 2
}
[ -n "$known" ] || known=$dir/mutation-known.txt
[ -f "$known" ] || { echo "check-mutation: no $known" >&2; exit 2; }

name=$(basename "$(CDPATH= cd -- "$dir" && pwd)")
work=$root/target/mutation/$name
rm -rf "$work"
mkdir -p "$work"

src=$dir
if grep -r -q -w Binary64 "$dir" --include='*.yaml'; then
  src=$work/src
  cp -R "$dir" "$src"
  find "$src" -name '*.yaml' -exec sed -i 's/\bBinary64\b/Decimal/g' {} +
fi

scenarios=
if awk '/^scenarios:/ { on = 1; next } /^[a-z_]+:/ { on = 0 } on && /^  - / { found = 1 } END { exit !found }' "$src/ess-inputs.yaml"; then
  scenarios="--scenarios $src"
fi

narrow=$only
[ -n "$only" ] || only="from-drop transition-to guard-boundary sets-retarget guard-negate \
  guard-connective error-swap emit-drop order-flip sets-drop precedence-swap emit-swap grant-drop \
  grant-add"
classes=
for c in $only; do
  classes="$classes --class $c"
done

status=0
# shellcheck disable=SC2086 # $scenarios and $classes are lists of arguments
ess verify conform mutate --path "$src" --target interpreted $scenarios $classes \
  --format json --report-out "$work/report.json" > "$work/mutate.txt" 2>&1 || status=$?
case $status in
  0 | 1) ;;
  3)
    if grep -q 'ESS-MUTATE-001' "$work/mutate.txt"; then
      grep -v '^note:' "$work/mutate.txt" | tail -n 20 >&2
      echo "check-mutation: $dir: the suite failed on the unchanged specification (ESS-MUTATE-001)" >&2
      exit 1
    fi
    ;;
  *)
    grep -v '^note:' "$work/mutate.txt" | tail -n 20 >&2
    echo "check-mutation: $dir: the audit did not run (exit $status)" >&2
    exit 2
    ;;
esac
[ -s "$work/report.json" ] || { echo "check-mutation: $dir: no report" >&2; exit 2; }

jq -r '.mutants[] | select(.verdict != "killed" and .verdict != "equivalent" and .verdict != "stillborn") | .id' \
  "$work/report.json" | LC_ALL=C sort > "$work/open.txt"
grep -v -e '^#' -e '^$' "$known" | cut -d' ' -f1 | while IFS= read -r id; do
  [ -n "$narrow" ] || { echo "$id"; continue; }
  for c in $narrow; do
    case $id in "$c"/*) echo "$id" ;; esac
  done
done | LC_ALL=C sort > "$work/known.txt"
counts=$(jq -r '.counts | "\(.mutants) mutants: \(.killed) killed, \(.survived) survived, \(.unwitnessed) unwitnessed, \(.inconclusive) inconclusive, \(.equivalent) equivalent, \(.stillborn) stillborn"' "$work/report.json")
sites=$(jq -r '(.unmutated_sites // []) as $u | (.unavailable_sites // []) as $a
  | "\($u | length) unmutated sites"
  + (if ($u | length) > 0 then " (" + ($u | group_by(.reason) | map("\(.[0].reason) \(length)") | join(", ")) + ")" else "" end)
  + ", \($a | length) unavailable"' "$work/report.json")
counts="$counts; $sites"

if cmp -s "$work/open.txt" "$work/known.txt"; then
  echo "check-mutation: $dir: $counts; the $(wc -l < "$work/known.txt" | tr -d ' ') not killed are exactly those of $known"
  exit 0
fi
LC_ALL=C comm -23 "$work/open.txt" "$work/known.txt" | while IFS= read -r id; do
  verdict=$(jq -r --arg id "$id" '.mutants[] | select(.id == $id) | "\(.verdict): \(.change)"' "$work/report.json")
  echo "  not killed, not known: $id ($verdict)"
done
LC_ALL=C comm -13 "$work/open.txt" "$work/known.txt" | sed "s#^#  known, now killed (remove it from $known): #"
echo "check-mutation: $dir: $counts; report $work/report.json" >&2
exit 1
