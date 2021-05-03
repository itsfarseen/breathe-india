#!/bin/sh
echo "Build completed"
ls .
ls -R dist
echo "Copying files from dist to nginx_volume"
rm -rf nginx_volume/*
cp -r dist/* nginx_volume
