#!/bin/sh
# Cases of scripts/check-mutation.sh, each a planted defect it must name. `task check:scripts`
# runs it; each case audits one class (`--class`) to stay fast.
#   1. studio/ without its authored scenarios: the `and` of CallTool's dry-run guard turned into
#      `or` survives, and the check fails naming that mutant;
#   2. studio/ with one line missing from its known list: the check fails naming that mutant.
set -eu

root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
work=$root/target/check-mutation-test
rm -rf "$work"
mkdir -p "$work"
fail=0

# expect_named <case> <mutant id> <dir> [args]: the check exits 1, names the mutant and prints the
# counts of the sites it did not mutate.
expect_named() {
  case_=$1 id=$2 dir=$3
  shift 3
  status=0
  sh "$root/scripts/check-mutation.sh" "$dir" "$@" > "$work/$case_.txt" 2>&1 || status=$?
  if [ "$status" != 1 ] || ! grep -q -F "not killed, not known: $id " "$work/$case_.txt" \
    || ! grep -q -E "[0-9]+ unmutated sites.*, [0-9]+ unavailable" "$work/$case_.txt"; then
    echo "check-mutation.test: $case_: expected exit 1 naming $id, got exit $status:" >&2
    cat "$work/$case_.txt" >&2
    fail=1
  fi
}

cp -R "$root/studio" "$work/studio"
sed -i '/^scenarios:/,$d' "$work/studio/ess-inputs.yaml"
echo 'scenarios: []' >> "$work/studio/ess-inputs.yaml"
expect_named no-scenarios guard-connective/studio.session.CallTool/dry-run-outside-apply/0 \
  "$work/studio" --known "$root/studio/mutation-known.txt" --class guard-connective

grep -v -F 'error-swap/studio.evaluation.FailEvaluation/over ' "$root/studio/mutation-known.txt" > "$work/known.txt"
expect_named known-line-missing error-swap/studio.evaluation.FailEvaluation/over \
  "$root/studio" --known "$work/known.txt" --class error-swap

[ "$fail" = 0 ] && echo "check-mutation.test: 2 planted defects named"
exit "$fail"
