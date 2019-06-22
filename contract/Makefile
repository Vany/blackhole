WASM = target/wasm32-unknown-unknown/release/pgp_keyserver.wasm

PWD = `pwd`
PORT = 30000


$(WASM): src/keyserver.rs
	cargo +nightly build --lib --target wasm32-unknown-unknown --release


.PHONY: build debug clean

build: $(WASM)

clean:
	cargo clean

debug: build
	docker run --rm -v $(PWD)/$(WASM):/code/code.wasm -p $(PORT):$(PORT) fluencelabs/frun

test:
	curl 'http://localhost:$(PORT)/apps/0/tx' --data $$'sessionId/1\nHelloWorld' | jq -r .result.data | base64 -D
