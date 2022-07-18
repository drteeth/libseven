#!/usr/bin/env bash

if [ -z "$ANDROID_NDK_TOOLCHAINS" ]
then
  echo 'ANDROID_NDK_TOOLCHAINS must be exported and point to the bin folder of the NDK toolchains'
  echo ''
  echo 'macos:'
  echo 'export ANDROID_NDK_TOOLCHAINS=$HOME/Library/Android/sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/darwin-x86_64/bin'
  echo ''
  echo 'linux:'
  echo 'export ANDROID_NDK_TOOLCHAINS=$HOME/Android/Sdk/ndk/21.4.7075529/toolchains/llvm/prebuilt/linux-x86_64/bin'
fi

cat <<EOF > .cargo/config
[target.aarch64-linux-android]
linker = "$ANDROID_NDK_TOOLCHAINS/aarch64-linux-android21-clang"

[target.armv7-linux-androideabi]
linker = "$ANDROID_NDK_TOOLCHAINS/armv7a-linux-androideabi21-clang"

[target.i686-linux-android]
linker = "$ANDROID_NDK_TOOLCHAINS/i686-linux-android21-clang"

[target.x86_64-linux-android]
linker = "$ANDROID_NDK_TOOLCHAINS/x86_64-linux-android21-clang"
EOF

