#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#[derive(Debug)]
pub struct PowerEntity {
  pub id: i32,
  pub name: String,
  pub states: Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_5_State>,
}
impl Default for PowerEntity {
  fn default() -> Self {
    Self {
      id: 0,
      name: Default::default(),
      states: Default::default(),
    }
  }
}
impl binder::Parcelable for PowerEntity {
  fn write_to_parcel(&self, parcel: &mut binder::binder_impl::BorrowedParcel) -> std::result::Result<(), binder::StatusCode> {
    parcel.sized_write(|subparcel| {
      subparcel.write(&self.id)?;
      subparcel.write(&self.name)?;
      subparcel.write(&self.states)?;
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
      if subparcel.has_more_data() {
        self.states = subparcel.read()?;
      }
      Ok(())
    })
  }
}
binder::impl_serialize_for_parcelable!(PowerEntity);
binder::impl_deserialize_for_parcelable!(PowerEntity);
impl binder::binder_impl::ParcelableMetadata for PowerEntity {
  fn get_descriptor() -> &'static str { "android.hardware.power.stats.PowerEntity" }
  fn get_stability(&self) -> binder::binder_impl::Stability { binder::binder_impl::Stability::Vintf }
}
pub(crate) mod mangled {
 pub use super::PowerEntity as _7_android_8_hardware_5_power_5_stats_11_PowerEntity;
}
