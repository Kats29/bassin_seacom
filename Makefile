send: build
	./.send.sh

build: backend frontend common

backend: backend/src/*
	PKG_CONFIG_ALLOW_CROSS=1 CC_armv7_unknown_linux_musleabihf=arm-linux-gnueabihf-gcc CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-gcc cargo build -p backend --target=armv7-unknown-linux-musleabihf

frontend: frontend/src/*
	cd frontend; trunk build

common: common/src/*
