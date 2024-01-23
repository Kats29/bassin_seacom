all: backend frontend common

backend: backend/src/*
	cd backend; PKG_CONFIG_ALLOW_CROSS=1 cargo build --target=armv7-unknown-linux-gnueabihf

frontend: frontend/src/*
	cd frontend; trunk build

common: common/src/*
