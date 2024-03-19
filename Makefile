release: OPT_FLAG = --release --no-default-features


.all: build


build:
	npm install
	cargo build $(OPT_FLAG)
	wasm-pack build --target web wlrs-wasm
	npm run build


release: build


install:
	@mkdir -p /opt/wlrs-wasm
	@mkdir -p /opt/wlrs-server

	rm -rf /opt/wlrs-wasm/*
	rm -rf /opt/wlrs-server/*

	cp -r dist /opt/wlrs/wlrs-wasm/
	cp -r node_modules /opt/wlrs/wlrs-wasm/
	cp -r wlrs-wasm/pkg /opt/wlrs/wlrs-wasm/
	cp package.json /opt/wlrs/wlrs-wasm/

	cp target/release/wlrs-server /usr/local/bin/

	cp .env /opt/wlrs/wlrs-wasm/
	cp .env /opt/wlrs/wlrs-server/


run:
	npm run dev


clean:
	cargo clean
	@rm -rf dist
	@rm -rf wlrs-wasm/pkg
	@rm -rf node_modules
	@rm -f package-lock.json
