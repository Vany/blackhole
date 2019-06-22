WASM = target/wasm32-unknown-unknown/release/pgp_keyserver.wasm

PWD = `pwd`
PORT = 30000


$(WASM): Cargo.toml src/keyserver.rs
	cargo +nightly build --lib --target wasm32-unknown-unknown --release


.PHONY: build debug clean

build: $(WASM)

clean:
	cargo clean

debug: build
	docker run --rm -v $(PWD)/$(WASM):/code/code.wasm -p $(PORT):$(PORT) fluencelabs/frun:newline



SERIAL = 0
TESTPAYLOAD ?= GET / HTTP/1.1\r\nHost: urh.ru\r\n\r\n{"hello":"world"}


test:
	curl 'http://localhost:$(PORT)/apps/1/tx' --data $$'sessionId/$(SERIAL)\n$(TESTPAYLOAD)' | jq -r .result.data | base64 -D
