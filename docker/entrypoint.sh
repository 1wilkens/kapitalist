#!/bin/bash

set -eox pipefail

# wait for postgres to be available
wait-for-port.sh "${KAPITALIST_DB_HOST:=db}:5432" -t 0 -q

# init database and start serving
# XXX: init should only be run on first launch, how can we manage that?
kapitalist init
kapitalist "$@"
