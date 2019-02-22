

bindgen:
	cargo +nightly build --target wasm32-unknown-unknown
	@mkdir pkg 2> /dev/null || true
	wasm-bindgen target/wasm32-unknown-unknown/debug/mywasm.wasm --typescript --out-dir pkg/
	@tail -n +2 pkg/mywasm.d.ts | prettier --parser typescript > js/src/mywasm.ts


wasm:
	#RUST_LOG=error wasm-pack build --dev
	wasm-pack build --dev
	@tail -n +2 pkg/mywasm.d.ts | prettier --parser typescript > js/src/mywasm.ts

install:
	curl https://sh.rustup.rs -sSf | sh
	rustup toolchain install nightly
	rustup target add wasm32-unknown-unknown --toolchain nightly
	cargo +nightly install wasm-bindgen-cli
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh


# npm install -g prettier
pretty:
	@prettier pkg/mywasm.d.ts --parser typescript | diff -w pkg/mywasm.d.ts - | colordiff

# cargo install bat
view:
	@prettier pkg/mywasm.d.ts --parser typescript | bat -l typescript

js/src/mywasm.ts : pkg/mywasm.d.ts
	@prettier --parser typescript pkg/mywasm.d.ts > js/src/mywasm.ts

tsc:
	@cd js; tsc

run:
	@cargo run > junk.json
	@node js/dist/node2.js --file junk.json --all


pest:
	@cargo run -- --cmd "$cmd" | prettier --parser typescript | bat -l typescript

.PHONY: pretty install wasm bindgen tsc run pest
