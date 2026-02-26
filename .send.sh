#!/bin/sh

sftp debian@192.168.7.3 << EOF
	put target/armv7-unknown-linux-musleabihf/release/backend
	put -R frontend/dist
EOF
