II_CANISTER_ID = rdmx6-jaaaa-aaaaa-aaadq-cai
ISSUER_CANISTER_ID = bu5ax-5iaaa-aaaam-qbgcq-cai

create-canisters:
	@dfx canister create --all


deploy-demo-app:
	@dfx build rpdemo_backend
	@dfx generate rpdemo_backend
	@candid-extractor target/wasm32-unknown-unknown/release/rpdemo_backend.wasm > src/rpdemo_backend/rpdemo_backend.did
	# Use jq to extract and transform the root_key directly from the output of dfx ping.
	# jq will be used to parse JSON output, format the root_key array, and transform it into the required format.
	$(eval export ROOT_KEY=$(shell dfx ping \
		| jq -r '"{" + (.root_key | map(tostring + ":nat8") | join(";")) + "}"'))
	@dfx deploy rpdemo_backend --argument "( \
	    record { \
				ii_canister_id = principal \"$(II_CANISTER_ID)\"; \
				ic_root_key_der = vec $(ROOT_KEY); \
				issuer_canister_id = principal \"$(ISSUER_CANISTER_ID)\"; \
	    } \
	)"
	@find . -name '.DS_Store' -delete