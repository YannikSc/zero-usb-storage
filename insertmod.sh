#!/usr/bin/env bash

## Remove all modules blocking the OTG port

LOADED_GADGET_MODULES="$(lsmod | grep -E '^g_' | cut -d ' ' -f1)";

for MODULE in $LOADED_GADGET_MODULES; do
    modprobe -r "${MODULE}";
done

## Load our g_mass_storage

FILE="${1}";
READ_ONLY="${2}";
CDROM="${3}";

modprobe g_mass_storage "${FILE}" "${CDROM}" "${READ_ONLY}";
