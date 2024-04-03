#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct State {
  pub id: i32,
  pub name: String,
}
impl Default for State {
  fn default() -> Self {
    Self {
      id: 0,
      name: Default::default(),
    }
  }
}
impl binder::Parcelable for State {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.id)?;
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
        self.name = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(State);
binder::impl_deserialize_for_parcelable!(State);
impl binder::binder_impl::ParcelableMetadata for State {
  fn get_descriptor() -> &'static str { "android.hardware.power.stats.State" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::State as _7_android_8_hardware_5_power_5_stats_5_State;
}
