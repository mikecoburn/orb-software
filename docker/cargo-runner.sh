#!/usr/bin/env bash
# Allows running tests inside docker. Takes care of linux-only dependencies and avoids
# the need to emulate the architecture.


set -Eeuo pipefail

SELF=$0;
PROGRAM=$1; shift

main() {
	local -r absolute_path="$(realpath ${PROGRAM})"
	local -r docker_dir="$(dirname ${SELF})"

	docker build -t cargo-runner ${docker_dir}
	docker run \
		--rm \
		-v "${absolute_path}:/mnt/program" \
		-w /mnt \
		-e RUST_BACKTRACE \
		-it cargo-runner:latest \
		/mnt/program $@
}

main $@
