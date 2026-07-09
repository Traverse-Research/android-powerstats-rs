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
pub struct r#PowerEntity {
  pub r#id: i32,
  pub r#name: alloc::string::String,
  pub r#states: alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_5_State>,
}
impl Default for r#PowerEntity {
  fn default() -> Self {
    Self {
      r#id: 0,
      r#name: Default::default(),
      r#states: Default::default(),
    }
  }
}
impl binder::Parcelable for r#PowerEntity {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> core::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.r#id)?;
      subparcel.write(&self.r#name)?;
      subparcel.write(&self.r#states)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> core::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.r#id = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#name = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#states = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(r#PowerEntity);
binder::impl_deserialize_for_parcelable!(r#PowerEntity);
impl binder::binder_impl::ParcelableMetadata for r#PowerEntity {
  fn get_descriptor() -> &'static str { "android.hardware.power.stats.PowerEntity" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::r#PowerEntity as _7_android_8_hardware_5_power_5_stats_11_PowerEntity;
}
