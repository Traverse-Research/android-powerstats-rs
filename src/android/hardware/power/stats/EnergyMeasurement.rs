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
pub struct r#EnergyMeasurement {
  pub r#id: i32,
  pub r#timestampMs: i64,
  pub r#durationMs: i64,
  pub r#energyUWs: i64,
}
impl Default for r#EnergyMeasurement {
  fn default() -> Self {
    Self {
      r#id: 0,
      r#timestampMs: 0,
      r#durationMs: 0,
      r#energyUWs: 0,
    }
  }
}
impl binder::Parcelable for r#EnergyMeasurement {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> core::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#id)?;
      subparcel.write(&self.r#timestampMs)?;
      subparcel.write(&self.r#durationMs)?;
      subparcel.write(&self.r#energyUWs)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> core::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#id = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#timestampMs = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#durationMs = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#energyUWs = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#EnergyMeasurement);
binder::impl_deserialize_for_parcelable!(r#EnergyMeasurement);
impl binder::binder_impl::ParcelableMetadata for r#EnergyMeasurement {
  fn get_descriptor() -> &'static str { "android.hardware.power.stats.EnergyMeasurement" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#EnergyMeasurement as _7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement;
}
