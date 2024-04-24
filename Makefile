release: OPT_FLAG = --release


.all: build


build:
	cargo build $(OPT_FLAG)
	trunk build $(OPT_FLAG)


release: build


install:
	rm -rf /opt/wlrs/*
	@mkdir -p /opt/wlrs/wlrs-wasm
	@mkdir -p /opt/wlrs/wlrs-server

	cp wlrs-wasm/index.html /opt/wlrs/wlrs-wasm/
	cp -r wlrs-wasm/dist /opt/wlrs/wlrs-wasm/
	
	cp target/release/wlrs-server /usr/local/bin/

clean:
	cargo clean
	trunk clean