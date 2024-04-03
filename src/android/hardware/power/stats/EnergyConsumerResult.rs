#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct EnergyConsumerResult {
  pub id: i32,
  pub timestampMs: i64,
  pub energyUWs: i64,
  pub attribution: Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_25_EnergyConsumerAttribution>,
}
impl Default for EnergyConsumerResult {
  fn default() -> Self {
    Self {
      id: 0,
      timestampMs: 0,
      energyUWs: 0,
      attribution: Default::default(),
    }
  }
}
impl binder::Parcelable for EnergyConsumerResult {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.id)?;
      subparcel.write(&self.timestampMs)?;
      subparcel.write(&self.energyUWs)?;
      subparcel.write(&self.attribution)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.id = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.timestampMs = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.energyUWs = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.attribution = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(EnergyConsumerResult);
binder::impl_deserialize_for_parcelable!(EnergyConsumerResult);
impl binder::binder_impl::ParcelableMetadata for EnergyConsumerResult {
  fn get_descriptor() -> &'static str { "android.hardware.power.stats.EnergyConsumerResult" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::EnergyConsumerResult as _7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult;
}
