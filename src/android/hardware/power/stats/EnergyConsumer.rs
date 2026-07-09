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
pub struct r#EnergyConsumer {
  pub r#id: i32,
  pub r#ordinal: i32,
  pub r#type: crate::mangled::_7_android_8_hardware_5_power_5_stats_18_EnergyConsumerType,
  pub r#name: alloc::string::String,
}
impl Default for r#EnergyConsumer {
  fn default() -> Self {
    Self {
      r#id: 0,
      r#ordinal: 0,
      r#type: crate::mangled::_7_android_8_hardware_5_power_5_stats_18_EnergyConsumerType::OTHER,
      r#name: Default::default(),
    }
  }
}
impl binder::Parcelable for r#EnergyConsumer {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> core::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#id)?;
      subparcel.write(&self.r#ordinal)?;
      subparcel.write(&self.r#type)?;
      subparcel.write(&self.r#name)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> core::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#id = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#ordinal = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#type = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#name = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#EnergyConsumer);
binder::impl_deserialize_for_parcelable!(r#EnergyConsumer);
impl binder::binder_impl::ParcelableMetadata for r#EnergyConsumer {
  fn get_descriptor() -> &'static str { "android.hardware.power.stats.EnergyConsumer" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#EnergyConsumer as _7_android_8_hardware_5_power_5_stats_14_EnergyConsumer;
}
