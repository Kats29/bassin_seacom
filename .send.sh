#!/bin/sh

sftp debian@beaglebone.local << EOF
	put target/armv7-unknown-linux-musleabihf/debug/backend
	put -R frontend/dist
EOF
