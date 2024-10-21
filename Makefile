II_CANISTER_ID = rdmx6-jaaaa-aaaaa-aaadq-cai
ISSUER_CANISTER_ID = bu5ax-5iaaa-aaaam-qbgcq-cai
rootkey_did := $(shell dfx ping $(DFX_NETWORK) \
	| sed -n 's/.*"root_key": \[\(.*\)\].*/{\1}/p' \
	| sed 's/\([0-9][0-9]*\)/\1:nat8/g' \
	| sed 's/,/;/g')


create-canisters:
	@dfx canister create --all

deploy-demo-app:
	@echo "Root key from shell:!!! $(rootkey_did)!!!!"
	@dfx build rpdemo_backend
	@dfx generate rpdemo_backend
	@candid-extractor target/wasm32-unknown-unknown/release/rpdemo_backend.wasm > src/rpdemo_backend/rpdemo_backend.did
	# Use jq to extract and transform the root_key directly from the output of dfx ping.
	# jq will be used to parse JSON output, format the root_key array, and transform it into the required format.
	# $(eval export ROOT_KEY=$(shell dfx ping ic \
	# 	| jq -r '"{" + (.root_key | map(tostring + ":nat8") | join(";")) + "}"'))

	# @echo "Root key from NATIVE!!!!!! $(ROOT_KEY)!!!!!!"
	@dfx deploy rpdemo_backend --argument "( \
	    record { \
				ii_canister_id = principal \"$(II_CANISTER_ID)\"; \
				ic_root_key_der = vec $(rootkey_did); \
				issuer_canister_id = principal \"$(ISSUER_CANISTER_ID)\"; \
	    } \
	)"
	@find . -name '.DS_Store' -delete