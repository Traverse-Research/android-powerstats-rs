#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct StateResidencyResult {
  pub id: i32,
  pub stateResidencyData: Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_StateResidency>,
}
impl Default for StateResidencyResult {
  fn default() -> Self {
    Self {
      id: 0,
      stateResidencyData: Default::default(),
    }
  }
}
impl binder::Parcelable for StateResidencyResult {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.id)?;
      subparcel.write(&self.stateResidencyData)?;
      Ok(())
    })
  }
  fn read_from_parcel(&mut self, parcel: &binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_read(|subparcel| {
      if subparcel.has_more_data() {
        self.id = subparcel.read()?;
      }
      if subparcel.has_more_data() {
        self.stateResidencyData = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(StateResidencyResult);
binder::impl_deserialize_for_parcelable!(StateResidencyResult);
impl binder::binder_impl::ParcelableMetadata for StateResidencyResult {
  fn get_descriptor() -> &'static str { "android.hardware.power.stats.StateResidencyResult" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::StateResidencyResult as _7_android_8_hardware_5_power_5_stats_20_StateResidencyResult;
}
