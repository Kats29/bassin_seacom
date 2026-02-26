send: build
	./.send.sh

build: frontend backend common

backend: backend/src/*
	@echo "Building backend ..."
	cd backend && \
	PKG_CONFIG_ALLOW_CROSS=1 \
	CC_armv7_unknown_linux_musleabihf=arm-linux-gnueabihf-gcc \
	CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-gcc \
	cargo build -r -p backend \
	--target=armv7-unknown-linux-musleabihf \
	--manifest-path=Cargo.toml

frontend: frontend/src/*
	@echo "Building frontend ..."
	cd frontend && \
	cargo check --package frontend --target wasm32-unknown-unknown && \
	trunk build --verbose

common: common/src/*

doc :
	cargo rustdoc -p backend --target=armv7-unknown-linux-musleabihf --open
	cargo rustdoc -p common --open
	cargo rustdoc -p frontend --target=wasm32-unknown-unknown --open
