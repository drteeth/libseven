#!/bin/bash

echo y | $ANDROID_HOME/tools/bin/sdkmanager "ndk;21.4.7075529" --channel=3;
echo y | $ANDROID_HOME/cmdline-tools/latest/bin/sdkmanager "cmake;3.18.1"
