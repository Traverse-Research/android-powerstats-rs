#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
use binder::declare_binder_enum;
declare_binder_enum! {
  EnergyConsumerType : [i8; 8] {
    OTHER = 0,
    BLUETOOTH = 1,
    CPU_CLUSTER = 2,
    DISPLAY = 3,
    GNSS = 4,
    MOBILE_RADIO = 5,
    WIFI = 6,
    CAMERA = 7,
  }
}
pub(crate) mod mangled {
 pub use super::EnergyConsumerType as _7_android_8_hardware_5_power_5_stats_18_EnergyConsumerType;
}
