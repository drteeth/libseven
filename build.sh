#!/usr/bin/env bash

set -ex

cargo +nightly build --target i686-linux-android -Zbuild-std

# cargo tly build -t arm64-v8a build -Zbuild-std
# cargo build --target aarch64-linux-android --release
# cargo build --target armv7-linux-androidabi --release
# cargo build --target i686-linux-android --release

mkdir -p android/app/src/main/jniLibs/arm64
mkdir -p android/app/src/main/jniLibs/armeabi
mkdir -p android/app/src/main/jniLibs/x86

ln -s `pwd`/target/release/libseven.so `pwd`/android/app/src/main/jniLibs/x86/libseven.so
