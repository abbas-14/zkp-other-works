#!/bin/bash
#
# verifier verifies the proof provided verification keys for particular ckt
#
echo 'Verifying the proof..'
echo ''
zokrates verify -j proof.json -v verification.key
