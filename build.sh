#!/usr/bin/env bash

set -ex

mkdir -p android/app/src/main/jniLibs/x86_64
cargo build --target x86_64-linux-android --release
cp `pwd`/target/x86_64-linux-android/release/libseven.so `pwd`/android/app/src/main/jniLibs/x86_64/libseven.so

mkdir -p android/app/src/main/jniLibs/arm64-v8a
cargo build --target aarch64-linux-android --release
cp `pwd`/target/aarch64-linux-android/release/libseven.so `pwd`/android/app/src/main/jniLibs/arm64-v8a/libseven.so

mkdir -p android/app/src/main/jniLibs/x86
cargo build --target i686-linux-android --release
cp `pwd`/target/i686-linux-android/release/libseven.so `pwd`/android/app/src/main/jniLibs/x86/libseven.so

# mkdir -p android/app/src/main/jniLibs/armeabi-v7a
# cargo build --target armv7-linux-androideabi --release
# cp `pwd`/target/armv7-linux-androideabi/release/libseven.so `pwd`/android/app/src/main/jniLibs/armeabi-v7a/libseven.so

stat `pwd`/android/app/src/main/jniLibs/x86_64/libseven.so
stat `pwd`/android/app/src/main/jniLibs/arm64-v8a/libseven.so
stat `pwd`/android/app/src/main/jniLibs/x86/libseven.so
# stat `pwd`/android/app/src/main/jniLibs/armeabi-v7a/libseven.so
