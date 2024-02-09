#!/bin/sh

sftp debian@bassin.local << EOF
	put target/armv7-unknown-linux-musleabihf/debug/backend
	put -R frontend/dist
EOF
