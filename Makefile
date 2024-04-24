release: OPT_FLAG = --release


.all: build


build:
	cargo build $(OPT_FLAG)
	trunk build $(OPT_FLAG)


release: build


install:
	@mkdir -p /opt/wlrs-wasm
	@mkdir -p /opt/wlrs-server

	rm -rf /opt/wlrs-wasm/*
	rm -rf /opt/wlrs-server/*

	cp -r dist /opt/wlrs/wlrs-wasm/
	cp -r target /opt/wlrs/wlrs-wasm/
	
	cp target/release/wlrs-server /usr/local/bin/

clean:
	cargo clean
	trunk clean