#!/bin/sh

set -eu

: "${EXEC_TOOL:=gosu}"
: "${MIONOTE_HOST:=0.0.0.0}"
: "${MIONOTE_PORT:=4233}"

echo "\
======================================
======== Welcome to MioNote ========
======================================

Thank you for using MioNote.

──────────────────────────────────────
"

if [ "$(id -u)" -eq 0 ] && [ "$(id -g)" -eq 0 ]; then
    echo Setting file permissions...
    chown -R ${PUID}:${PGID} ${MIONOTE_PATH}

    echo Starting MioNote as user ${PUID}...
    exec "${EXEC_TOOL}" "${PUID}:${PGID}" mionote

else
    echo "A user was set by docker, skipping file permission changes."
    echo Starting MioNote as user $(id -u)...
    exec mionote
fi
