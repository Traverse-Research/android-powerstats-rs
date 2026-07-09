/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: aidl --structured --stability=vintf --lang=rust -Iandroid-hardware-interfaces/power/stats/aidl/ android-hardware-interfaces/power/stats/aidl/android/hardware/power/stats/Channel.aidl android-hardware-interfaces/power/stats/aidl/android/hardware/power/stats/EnergyConsumer.aidl android-hardware-interfaces/power/stats/aidl/android/hardware/power/stats/EnergyConsumerAttribution.aidl android-hardware-interfaces/power/stats/aidl/android/hardware/power/stats/EnergyConsumerResult.aidl android-hardware-interfaces/power/stats/aidl/android/hardware/power/stats/EnergyConsumerType.aidl android-hardware-interfaces/power/stats/aidl/android/hardware/power/stats/EnergyMeasurement.aidl android-hardware-interfaces/power/stats/aidl/android/hardware/power/stats/IPowerStats.aidl android-hardware-interfaces/power/stats/aidl/android/hardware/power/stats/PowerEntity.aidl android-hardware-interfaces/power/stats/aidl/android/hardware/power/stats/State.aidl android-hardware-interfaces/power/stats/aidl/android/hardware/power/stats/StateResidency.aidl android-hardware-interfaces/power/stats/aidl/android/hardware/power/stats/StateResidencyResult.aidl -o src/
 *
 * DO NOT CHECK THIS FILE INTO A CODE TREE (e.g. git, etc..).
 * ALWAYS GENERATE THIS FILE FROM UPDATED AIDL COMPILER
 * AS A BUILD INTERMEDIATE ONLY. THIS IS NOT SOURCE CODE.
 */
#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  #[repr(align(1))]
  r#EnergyConsumerType : [i8; 8] {
    r#OTHER = 0,
    r#BLUETOOTH = 1,
    r#CPU_CLUSTER = 2,
    r#DISPLAY = 3,
    r#GNSS = 4,
    r#MOBILE_RADIO = 5,
    r#WIFI = 6,
    r#CAMERA = 7,
  }
}
pub(crate) mod mangled {
 pub use super::r#EnergyConsumerType as _7_android_8_hardware_5_power_5_stats_18_EnergyConsumerType;
}
