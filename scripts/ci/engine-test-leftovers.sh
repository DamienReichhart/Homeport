#!/usr/bin/env bash
# Lists the homeport-test-* resources the Engine integration tests left behind,
# removes them, and fails if there were any: every test must clean up after itself.
set -euo pipefail

containers=$(docker ps --all --quiet --filter name=homeport-test-)
images=$(docker image ls --format '{{.Repository}}:{{.Tag}}' | grep -E '(^|/)homeport-test-' || true)
volumes=$(docker volume ls --quiet --filter name=homeport-test-)
networks=$(docker network ls --quiet --filter name=homeport-test-)

left=0
report() {
  if [ -n "$2" ]; then
    echo "::error::Engine tests left $1 behind:"
    echo "$2"
    left=1
  fi
}
report containers "$containers"
report images "$images"
report volumes "$volumes"
report networks "$networks"

[ -n "$containers" ] && docker rm --force --volumes $containers >/dev/null
[ -n "$images" ] && docker image rm --force $images >/dev/null
[ -n "$volumes" ] && docker volume rm --force $volumes >/dev/null
[ -n "$networks" ] && docker network rm $networks >/dev/null

if [ "$left" -eq 0 ]; then
  echo "No homeport-test-* resources were left behind."
fi
exit "$left"
