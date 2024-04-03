#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct EnergyConsumer {
  pub id: i32,
  pub ordinal: i32,
  pub r#type: crate::mangled::_7_android_8_hardware_5_power_5_stats_18_EnergyConsumerType,
  pub name: String,
}
impl Default for EnergyConsumer {
  fn default() -> Self {
    Self {
      id: 0,
      ordinal: 0,
      r#type: crate::mangled::_7_android_8_hardware_5_power_5_stats_18_EnergyConsumerType::OTHER,
      name: Default::default(),
    }
  }
}
impl binder::Parcelable for EnergyConsumer {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.id)?;
      subparcel.write(&self.ordinal)?;
      subparcel.write(&self.r#type)?;
      subparcel.write(&self.name)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.id = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.ordinal = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.r#type = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.name = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(EnergyConsumer);
binder::impl_deserialize_for_parcelable!(EnergyConsumer);
impl binder::binder_impl::ParcelableMetadata for EnergyConsumer {
  fn get_descriptor() -> &'static str { "android.hardware.power.stats.EnergyConsumer" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::EnergyConsumer as _7_android_8_hardware_5_power_5_stats_14_EnergyConsumer;
}
