#!/usr/bin/env bash

## Remove all modules blocking the OTG port

LOADED_GADGET_MODULES="$(lsmod | grep -E '^g_' | cut -d ' ' -f1)";

for MODULE in $LOADED_GADGET_MODULES; do
    modprobe -r "${MODULE}";
done

## Load our g_mass_storage

MODULE="${1}";
ARGUMENTS="${@:2}"
modprobe "${MODULE}" ${ARGUMENTS};
