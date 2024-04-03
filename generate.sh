#! /usr/bin/bash

set -ex

P=android-hardware-interfaces/power/stats/aidl/
aidl --structured --stability=vintf --lang=rust -I$P $P/android/hardware/power/stats/*.aidl -o src/

P=android-frameworks-base/core/java
aidl --lang=rust -I$P $P/android/os/Bundle.aidl $P/android/os/IPowerStatsService.aidl $P/com/android/internal/os/IResultReceiver.aidl -o src/
