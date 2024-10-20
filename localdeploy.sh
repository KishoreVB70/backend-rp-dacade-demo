#!/usr/bin/env bash

# At the time of writing dfx outputs incorrect JSON with dfx ping (commas between object
# entries are missing).
# In order to read the root key we grab the array from the '"root_key": [...]' bit, the brackets
# to match what candid expects ({}), replace the commas between array entries to match
# what candid expects (semicolon) and annotate the numbers with their type (otherwise dfx assumes 'nat'
# instead of 'nat8').

DFX_NETWORK="local"
# URL used by II-issuer in the id_alias-verifiable credentials (hard-coded in II)
II_VC_URL="https://identity.ic0.app/"
# URL used by meta-issuer in the issued verifiable credentials (hard-coded in meta-issuer)
ISSUER_VC_URL="https://dacade.org/"
ISSUER_CANISTER_ID="bu5ax-5iaaa-aaaam-qbgcq-cai"
II_CANISTER_ID="rdmx6-jaaaa-aaaaa-aaadq-cai"


rootkey_did=$(dfx ping "$DFX_NETWORK" \
    | sed -n 's/.*"root_key": \[\(.*\)\].*/{\1}/p' \
    | sed 's/\([0-9][0-9]*\)/\1:nat8/g' \
    | sed 's/,/;/g')

echo "Parsed rootkey: ${rootkey_did:0:20}..." >&2

dfx deploy bot_backend --network "$DFX_NETWORK" --argument '(opt record { issuers = vec{ record{ vc_url = "'"$ISSUER_VC_URL"'"; canister_id = principal "'"$ISSUER_CANISTER_ID"'" }}; ic_root_key_der = vec '"$rootkey_did"'; ii_vc_url = "'"$II_VC_URL"'"; ii_canister_id = principal"'"$II_CANISTER_ID"'"; })'
