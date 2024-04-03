#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct EnergyConsumerAttribution {
  pub uid: i32,
  pub energyUWs: i64,
}
impl Default for EnergyConsumerAttribution {
  fn default() -> Self {
    Self {
      uid: 0,
      energyUWs: 0,
    }
  }
}
impl binder::Parcelable for EnergyConsumerAttribution {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.uid)?;
      subparcel.write(&self.energyUWs)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.uid = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.energyUWs = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(EnergyConsumerAttribution);
binder::impl_deserialize_for_parcelable!(EnergyConsumerAttribution);
impl binder::binder_impl::ParcelableMetadata for EnergyConsumerAttribution {
  fn get_descriptor() -> &'static str { "android.hardware.power.stats.EnergyConsumerAttribution" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::EnergyConsumerAttribution as _7_android_8_hardware_5_power_5_stats_25_EnergyConsumerAttribution;
}
