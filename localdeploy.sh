#!/usr/bin/env bash

II_CANISTER_ID=rdmx6-jaaaa-aaaaa-aaadq-cai
ISSUER_CANISTER_ID=bu5ax-5iaaa-aaaam-qbgcq-cai

# Create and build the canister
dfx canister create rpdemo_backend
dfx build rpdemo_backend

# Generate Candid interface
candid-extractor target/wasm32-unknown-unknown/release/rpdemo_backend.wasm > src/rpdemo_backend/rpdemo_backend.did

# Generate JavaScript bindings
dfx generate rpdemo_backend

# Extract the root key using dfx ping and proper command substitution
rootkey_did=$(dfx ping local \
    | sed -n 's/.*"root_key": \[\(.*\)\].*/{\1}/p' \
    | sed 's/\([0-9][0-9]*\)/\1:nat8/g' \
    | sed 's/,/;/g')

echo "Public key: ${rootkey_did}"
# Deploy the canister
dfx deploy rpdemo_backend --network local --argument "( \
    record { \
        ii_canister_id = principal \"${II_CANISTER_ID}\"; \
        ic_root_key_der = vec ${rootkey_did}; \
        issuer_canister_id = principal \"${ISSUER_CANISTER_ID}\"; \
    } \
)"
