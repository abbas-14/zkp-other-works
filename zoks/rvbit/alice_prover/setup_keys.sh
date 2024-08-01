#!/bin/bash

# prover needs to compile the circuit and create the proving and verifying keys
# then prover sends the verification keys to the verifier side
# then generate the witness provided with all the inputs to the circuit
# then generate proof
# then prover sends the proof to the verifier side
#
# lets compile

CKT_PREF="magic_sq_mod"
IPS="31 73 7 13 37 61 67 0 43 111"
# CKT_PREF="reveal_bit"
# IPS="0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 510"
ZOK_FILE="${CKT_PREF}.zok"
CKT_BIN_FILE="${CKT_PREF}_compiled_ckt"

echo ''
echo 'Compiling the circuit..'
echo ''
zokrates compile -i $ZOK_FILE -o $CKT_BIN_FILE
# lets generate the keys
echo ''
echo 'Generating keys..'
echo ''
zokrates setup -i $CKT_BIN_FILE -p proving.key -v  verification.key
