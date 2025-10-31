#! /bin/bash -ex

cargo b --examples --target aarch64-linux-android
adb push target/aarch64-linux-android/debug/examples/sample_gpu_meters data/local/tmp
adb shell /data/local/tmp/sample_gpu_meters
