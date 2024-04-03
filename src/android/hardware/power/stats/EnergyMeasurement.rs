#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct EnergyMeasurement {
  pub id: i32,
  pub timestampMs: i64,
  pub durationMs: i64,
  pub energyUWs: i64,
}
impl Default for EnergyMeasurement {
  fn default() -> Self {
    Self {
      id: 0,
      timestampMs: 0,
      durationMs: 0,
      energyUWs: 0,
    }
  }
}
impl binder::Parcelable for EnergyMeasurement {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.id)?;
      subparcel.write(&self.timestampMs)?;
      subparcel.write(&self.durationMs)?;
      subparcel.write(&self.energyUWs)?;
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
        self.durationMs = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.energyUWs = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(EnergyMeasurement);
binder::impl_deserialize_for_parcelable!(EnergyMeasurement);
impl binder::binder_impl::ParcelableMetadata for EnergyMeasurement {
  fn get_descriptor() -> &'static str { "android.hardware.power.stats.EnergyMeasurement" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::EnergyMeasurement as _7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement;
}
