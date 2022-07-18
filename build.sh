#!/usr/bin/env bash

set -ex

mkdir -p jniLibs/x86_64
cargo build --target x86_64-linux-android --release
cp target/x86_64-linux-android/release/libseven.so jniLibs/x86_64/

mkdir -p jniLibs/arm64-v8a
cargo build --target aarch64-linux-android --release
cp target/aarch64-linux-android/release/libseven.so jniLibs/arm64-v8a/

mkdir -p jniLibs/x86
cargo build --target i686-linux-android --release
cp target/i686-linux-android/release/libseven.so jniLibs/x86/

mkdir -p jniLibs/armeabi-v7a
cargo build --target armv7-linux-androideabi --release
cp target/armv7-linux-androideabi/release/libseven.so jniLibs/armeabi-v7a/
