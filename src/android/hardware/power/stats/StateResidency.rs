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
use alloc::boxed::Box;
#[derive(Debug)]
pub struct r#StateResidency {
  pub r#id: i32,
  pub r#totalTimeInStateMs: i64,
  pub r#totalStateEntryCount: i64,
  pub r#lastEntryTimestampMs: i64,
}
impl Default for r#StateResidency {
  fn default() -> Self {
    Self {
      r#id: 0,
      r#totalTimeInStateMs: 0,
      r#totalStateEntryCount: 0,
      r#lastEntryTimestampMs: 0,
    }
  }
}
impl binder::Parcelable for r#StateResidency {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> core::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#id)?;
      subparcel.write(&self.r#totalTimeInStateMs)?;
      subparcel.write(&self.r#totalStateEntryCount)?;
      subparcel.write(&self.r#lastEntryTimestampMs)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> core::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#id = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#totalTimeInStateMs = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#totalStateEntryCount = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#lastEntryTimestampMs = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#StateResidency);
binder::impl_deserialize_for_parcelable!(r#StateResidency);
impl binder::binder_impl::ParcelableMetadata for r#StateResidency {
  fn get_descriptor() -> &'static str { "android.hardware.power.stats.StateResidency" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#StateResidency as _7_android_8_hardware_5_power_5_stats_14_StateResidency;
}
