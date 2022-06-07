#!/bin/bash

if [[ "${TARGET}" == "aarch64-apple-darwin" ]]; then
    echo "mac-aarch64"
elif [[ "${TARGET}" == "aarch64-pc-windows-msvc" ]]; then
    echo "win-aarch64"
elif [[ "${TARGET}" == "aarch64-unknown-linux-musl" ]]; then
    echo "linux-aarch64-musl"
elif [[ "${TARGET}" == "aarch64-unknown-linux-gnu" ]]; then
    echo "linux-aarch64"
elif [[ "${TARGET}" == "arm-unknown-linux-gnueabi" ]]; then
    echo "linux-arm"
elif [[ "${TARGET}" == "arm-unknown-linux-gnueabihf" ]]; then
    echo "linux-armhf"
elif [[ "${TARGET}" == "arm-unknown-linux-musleabi" ]]; then
    echo "linux-arm-musl"
elif [[ "${TARGET}" == "arm-unknown-linux-musleabihf" ]]; then
    echo "linux-armhf-musl"
elif [[ "${TARGET}" == "armv7-unknown-linux-gnueabi" ]]; then
    echo "linux-arm"
elif [[ "${TARGET}" == "armv7-unknown-linux-musleabi" ]]; then
    echo "linux-arm-musl"
elif [[ "${TARGET}" == "armv7-unknown-linux-musleabihf" ]]; then
    echo "linux-armhf-musl"
elif [[ "${TARGET}" == "armv7-unknown-linux-gnueabihf" ]]; then
    echo "linux-armhf"
elif [[ "${TARGET}" == "i686-unknown-linux-musl" ]]; then
    echo "linux-i686-musl"
elif [[ "${TARGET}" == "i686-pc-windows-gnu" ]]; then
    echo "win-i686"
elif [[ "${TARGET}" == "i686-pc-windows-msvc" ]]; then
    echo "win-i686-msvc"
elif [[ "${TARGET}" == "i686-unknown-linux-gnu" ]]; then
    echo "linux-i686"
elif [[ "${TARGET}" == "x86_64-apple-darwin" ]]; then
    echo "mac-x64"
elif [[ "${TARGET}" == "x86_64-pc-windows-gnu" ]]; then
    echo "win-x64"
elif [[ "${TARGET}" == "x86_64-pc-windows-msvc" ]]; then
    echo "win-x64-msvc"
elif [[ "${TARGET}" == "x86_64-unknown-linux-musl" ]]; then
    echo "linux-x64-musl"
elif [[ "${TARGET}" == "x86_64-unknown-linux-gnu" ]]; then
    echo "linux-x64"
fi
