#![forbid(unsafe_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  IPowerStats["android.hardware.power.stats.IPowerStats"] {
    native: BnPowerStats(on_transact),
    proxy: BpPowerStats {
    },
    async: IPowerStatsAsync,
    stability: binder::binder_impl::Stability::Vintf,
  }
}
pub trait IPowerStats: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.power.stats.IPowerStats" }
  fn getPowerEntityInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>>;
  fn getStateResidency(&self, _arg_powerEntityIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>>;
  fn getEnergyConsumerInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>>;
  fn getEnergyConsumed(&self, _arg_energyConsumerIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>>;
  fn getEnergyMeterInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>>;
  fn readEnergyMeter(&self, _arg_channelIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>>;
  fn getDefaultImpl() -> IPowerStatsDefaultRef where Self: Sized {
    DEFAULT_IMPL.lock().unwrap().clone()
  }
  fn setDefaultImpl(d: IPowerStatsDefaultRef) -> IPowerStatsDefaultRef where Self: Sized {
    std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
  }
}
pub trait IPowerStatsAsync<P>: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.power.stats.IPowerStats" }
  fn getPowerEntityInfo<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>>>;
  fn getStateResidency<'a>(&'a self, _arg_powerEntityIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>>>;
  fn getEnergyConsumerInfo<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>>>;
  fn getEnergyConsumed<'a>(&'a self, _arg_energyConsumerIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>>>;
  fn getEnergyMeterInfo<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>>>;
  fn readEnergyMeter<'a>(&'a self, _arg_channelIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>>>;
}
// #[::async_trait::async_trait]
// pub trait IPowerStatsAsyncServer: binder::Interface + Send {
//   fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.power.stats.IPowerStats" }
//   async fn getPowerEntityInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>>;
//   async fn getStateResidency(&self, _arg_powerEntityIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>>;
//   async fn getEnergyConsumerInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>>;
//   async fn getEnergyConsumed(&self, _arg_energyConsumerIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>>;
//   async fn getEnergyMeterInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>>;
//   async fn readEnergyMeter(&self, _arg_channelIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>>;
// }
// impl BnPowerStats {
//   /// Create a new async binder service.
//   pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IPowerStats>
//   where
//     T: IPowerStatsAsyncServer + binder::Interface + Send + Sync + 'static,
//     R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
//   {
//     struct Wrapper<T, R> {
//       _inner: T,
//       _rt: R,
//     }
//     impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync {
//       fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
//       fn dump(&self, _file: &std::fs::File, _args: &[&std::ffi::CStr]) -> std::result::Result<(), binder::StatusCode> { self._inner.dump(_file, _args) }
//     }
//     impl<T, R> IPowerStats for Wrapper<T, R>
//     where
//       T: IPowerStatsAsyncServer + Send + Sync + 'static,
//       R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
//     {
//       fn getPowerEntityInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>> {
//         self._rt.block_on(self._inner.getPowerEntityInfo())
//       }
//       fn getStateResidency(&self, _arg_powerEntityIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>> {
//         self._rt.block_on(self._inner.getStateResidency(_arg_powerEntityIds))
//       }
//       fn getEnergyConsumerInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>> {
//         self._rt.block_on(self._inner.getEnergyConsumerInfo())
//       }
//       fn getEnergyConsumed(&self, _arg_energyConsumerIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>> {
//         self._rt.block_on(self._inner.getEnergyConsumed(_arg_energyConsumerIds))
//       }
//       fn getEnergyMeterInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>> {
//         self._rt.block_on(self._inner.getEnergyMeterInfo())
//       }
//       fn readEnergyMeter(&self, _arg_channelIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>> {
//         self._rt.block_on(self._inner.readEnergyMeter(_arg_channelIds))
//       }
//     }
//     let wrapped = Wrapper { _inner: inner, _rt: rt };
//     Self::new_binder(wrapped, features)
//   }
// }
pub trait IPowerStatsDefault: Send + Sync {
  fn getPowerEntityInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn getStateResidency(&self, _arg_powerEntityIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn getEnergyConsumerInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn getEnergyConsumed(&self, _arg_energyConsumerIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn getEnergyMeterInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
  fn readEnergyMeter(&self, _arg_channelIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>> {
    Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
  }
}
pub mod transactions {
  pub const getPowerEntityInfo: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const getStateResidency: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
  pub const getEnergyConsumerInfo: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
  pub const getEnergyConsumed: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
  pub const getEnergyMeterInfo: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
  pub const readEnergyMeter: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
}
pub type IPowerStatsDefaultRef = Option<std::sync::Arc<dyn IPowerStatsDefault>>;
use lazy_static::lazy_static;
lazy_static! {
  static ref DEFAULT_IMPL: std::sync::Mutex<IPowerStatsDefaultRef> = std::sync::Mutex::new(None);
}
impl BpPowerStats {
  fn build_parcel_getPowerEntityInfo(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getPowerEntityInfo(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IPowerStats>::getDefaultImpl() {
        return _aidl_default_impl.getPowerEntityInfo();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getStateResidency(&self, _arg_powerEntityIds: &[i32]) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_powerEntityIds)?;
    Ok(aidl_data)
  }
  fn read_response_getStateResidency(&self, _arg_powerEntityIds: &[i32], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IPowerStats>::getDefaultImpl() {
        return _aidl_default_impl.getStateResidency(_arg_powerEntityIds);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getEnergyConsumerInfo(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getEnergyConsumerInfo(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IPowerStats>::getDefaultImpl() {
        return _aidl_default_impl.getEnergyConsumerInfo();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getEnergyConsumed(&self, _arg_energyConsumerIds: &[i32]) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_energyConsumerIds)?;
    Ok(aidl_data)
  }
  fn read_response_getEnergyConsumed(&self, _arg_energyConsumerIds: &[i32], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IPowerStats>::getDefaultImpl() {
        return _aidl_default_impl.getEnergyConsumed(_arg_energyConsumerIds);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getEnergyMeterInfo(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getEnergyMeterInfo(&self, _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IPowerStats>::getDefaultImpl() {
        return _aidl_default_impl.getEnergyMeterInfo();
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_readEnergyMeter(&self, _arg_channelIds: &[i32]) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_channelIds)?;
    Ok(aidl_data)
  }
  fn read_response_readEnergyMeter(&self, _arg_channelIds: &[i32], _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>> {
    if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
      if let Some(_aidl_default_impl) = <Self as IPowerStats>::getDefaultImpl() {
        return _aidl_default_impl.readEnergyMeter(_arg_channelIds);
      }
    }
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
}
impl IPowerStats for BpPowerStats {
  fn getPowerEntityInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>> {
    let _aidl_data = self.build_parcel_getPowerEntityInfo()?;
    let _aidl_reply = self.binder.submit_transact(transactions::getPowerEntityInfo, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getPowerEntityInfo(_aidl_reply)
  }
  fn getStateResidency(&self, _arg_powerEntityIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>> {
    let _aidl_data = self.build_parcel_getStateResidency(_arg_powerEntityIds)?;
    let _aidl_reply = self.binder.submit_transact(transactions::getStateResidency, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getStateResidency(_arg_powerEntityIds, _aidl_reply)
  }
  fn getEnergyConsumerInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>> {
    let _aidl_data = self.build_parcel_getEnergyConsumerInfo()?;
    let _aidl_reply = self.binder.submit_transact(transactions::getEnergyConsumerInfo, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getEnergyConsumerInfo(_aidl_reply)
  }
  fn getEnergyConsumed(&self, _arg_energyConsumerIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>> {
    let _aidl_data = self.build_parcel_getEnergyConsumed(_arg_energyConsumerIds)?;
    let _aidl_reply = self.binder.submit_transact(transactions::getEnergyConsumed, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getEnergyConsumed(_arg_energyConsumerIds, _aidl_reply)
  }
  fn getEnergyMeterInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>> {
    let _aidl_data = self.build_parcel_getEnergyMeterInfo()?;
    let _aidl_reply = self.binder.submit_transact(transactions::getEnergyMeterInfo, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_getEnergyMeterInfo(_aidl_reply)
  }
  fn readEnergyMeter(&self, _arg_channelIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>> {
    let _aidl_data = self.build_parcel_readEnergyMeter(_arg_channelIds)?;
    let _aidl_reply = self.binder.submit_transact(transactions::readEnergyMeter, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL);
    self.read_response_readEnergyMeter(_arg_channelIds, _aidl_reply)
  }
}
impl<P: binder::BinderAsyncPool> IPowerStatsAsync<P> for BpPowerStats {
  fn getPowerEntityInfo<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>>> {
    let _aidl_data = match self.build_parcel_getPowerEntityInfo() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::getPowerEntityInfo, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getPowerEntityInfo(_aidl_reply)
      }
    )
  }
  fn getStateResidency<'a>(&'a self, _arg_powerEntityIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>>> {
    let _aidl_data = match self.build_parcel_getStateResidency(_arg_powerEntityIds) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::getStateResidency, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getStateResidency(_arg_powerEntityIds, _aidl_reply)
      }
    )
  }
  fn getEnergyConsumerInfo<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>>> {
    let _aidl_data = match self.build_parcel_getEnergyConsumerInfo() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::getEnergyConsumerInfo, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getEnergyConsumerInfo(_aidl_reply)
      }
    )
  }
  fn getEnergyConsumed<'a>(&'a self, _arg_energyConsumerIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>>> {
    let _aidl_data = match self.build_parcel_getEnergyConsumed(_arg_energyConsumerIds) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::getEnergyConsumed, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getEnergyConsumed(_arg_energyConsumerIds, _aidl_reply)
      }
    )
  }
  fn getEnergyMeterInfo<'a>(&'a self) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>>> {
    let _aidl_data = match self.build_parcel_getEnergyMeterInfo() {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::getEnergyMeterInfo, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_getEnergyMeterInfo(_aidl_reply)
      }
    )
  }
  fn readEnergyMeter<'a>(&'a self, _arg_channelIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>>> {
    let _aidl_data = match self.build_parcel_readEnergyMeter(_arg_channelIds) {
      Ok(_aidl_data) => _aidl_data,
      Err(err) => return Box::pin(std::future::ready(Err(err))),
    };
    let binder = self.binder.clone();
    P::spawn(
      move || binder.submit_transact(transactions::readEnergyMeter, _aidl_data, binder::binder_impl::FLAG_PRIVATE_LOCAL),
      move |_aidl_reply| async move {
        self.read_response_readEnergyMeter(_arg_channelIds, _aidl_reply)
      }
    )
  }
}
impl IPowerStats for binder::binder_impl::Binder<BnPowerStats> {
  fn getPowerEntityInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>> { self.0.getPowerEntityInfo() }
  fn getStateResidency(&self, _arg_powerEntityIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>> { self.0.getStateResidency(_arg_powerEntityIds) }
  fn getEnergyConsumerInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>> { self.0.getEnergyConsumerInfo() }
  fn getEnergyConsumed(&self, _arg_energyConsumerIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>> { self.0.getEnergyConsumed(_arg_energyConsumerIds) }
  fn getEnergyMeterInfo(&self) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>> { self.0.getEnergyMeterInfo() }
  fn readEnergyMeter(&self, _arg_channelIds: &[i32]) -> binder::Result<Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>> { self.0.readEnergyMeter(_arg_channelIds) }
}
fn on_transact(_aidl_service: &dyn IPowerStats, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> std::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::getPowerEntityInfo => {
      let _aidl_return = _aidl_service.getPowerEntityInfo();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::getStateResidency => {
      let _arg_powerEntityIds: Vec<i32> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.getStateResidency(&_arg_powerEntityIds);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::getEnergyConsumerInfo => {
      let _aidl_return = _aidl_service.getEnergyConsumerInfo();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::getEnergyConsumed => {
      let _arg_energyConsumerIds: Vec<i32> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.getEnergyConsumed(&_arg_energyConsumerIds);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::getEnergyMeterInfo => {
      let _aidl_return = _aidl_service.getEnergyMeterInfo();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::readEnergyMeter => {
      let _arg_channelIds: Vec<i32> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.readEnergyMeter(&_arg_channelIds);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::IPowerStats as _7_android_8_hardware_5_power_5_stats_11_IPowerStats;
}
