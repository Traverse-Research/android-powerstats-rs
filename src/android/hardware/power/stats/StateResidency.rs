#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct StateResidency {
  pub id: i32,
  pub totalTimeInStateMs: i64,
  pub totalStateEntryCount: i64,
  pub lastEntryTimestampMs: i64,
}
impl Default for StateResidency {
  fn default() -> Self {
    Self {
      id: 0,
      totalTimeInStateMs: 0,
      totalStateEntryCount: 0,
      lastEntryTimestampMs: 0,
    }
  }
}
impl binder::Parcelable for StateResidency {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.id)?;
      subparcel.write(&self.totalTimeInStateMs)?;
      subparcel.write(&self.totalStateEntryCount)?;
      subparcel.write(&self.lastEntryTimestampMs)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.id = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.totalTimeInStateMs = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.totalStateEntryCount = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.lastEntryTimestampMs = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(StateResidency);
binder::impl_deserialize_for_parcelable!(StateResidency);
impl binder::binder_impl::ParcelableMetadata for StateResidency {
  fn get_descriptor() -> &'static str { "android.hardware.power.stats.StateResidency" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::StateResidency as _7_android_8_hardware_5_power_5_stats_14_StateResidency;
}
