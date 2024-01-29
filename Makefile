send: build
	./.send.sh

build: backend frontend common

backend: backend/src/*
	cd backend; CC_armv7_unknown_linux_musleabihf=arm-linux-gnueabihf-gcc CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=arm-linux-gnueabihf-gcc CAERPKG_CONFIG_ALLOW_CROSS=1 cargo build --target=armv7-unknown-linux-musleabihf

frontend: frontend/src/*
	cd frontend; trunk build

common: common/src/*
