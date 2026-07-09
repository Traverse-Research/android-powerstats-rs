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
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)] use binder::binder_impl::IBinderInternal;
use alloc::boxed::Box;
#[cfg(any(android_vndk, not(android_ndk)))]
const FLAG_PRIVATE_LOCAL: binder::binder_impl::TransactionFlags = binder::binder_impl::FLAG_PRIVATE_LOCAL;
#[cfg(not(any(android_vndk, not(android_ndk))))]
const FLAG_PRIVATE_LOCAL: binder::binder_impl::TransactionFlags = 0;
use binder::declare_binder_interface;
declare_binder_interface! {
  IPowerStats["android.hardware.power.stats.IPowerStats"] {
    native: BnPowerStats(on_transact),
    proxy: BpPowerStats {
    },
    // async: IPowerStatsAsync(try_into_local_async),
    stability: binder::binder_impl::Stability::Vintf,
  }
}
pub trait IPowerStats: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.power.stats.IPowerStats" }
  fn r#getPowerEntityInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>>;
  fn r#getStateResidency<'a, 'l1, >(&'a self, _arg_powerEntityIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>>;
  fn r#getEnergyConsumerInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>>;
  fn r#getEnergyConsumed<'a, 'l1, >(&'a self, _arg_energyConsumerIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>>;
  fn r#getEnergyMeterInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>>;
  fn r#readEnergyMeter<'a, 'l1, >(&'a self, _arg_channelIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>>;
  // fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IPowerStatsAsyncServer + Send + Sync)> {
  //   None
  // }
}
// pub trait IPowerStatsAsync<P>: binder::Interface + Send {
//   fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.power.stats.IPowerStats" }
//   fn r#getPowerEntityInfo<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>>>;
//   fn r#getStateResidency<'a, >(&'a self, _arg_powerEntityIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>>>;
//   fn r#getEnergyConsumerInfo<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>>>;
//   fn r#getEnergyConsumed<'a, >(&'a self, _arg_energyConsumerIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>>>;
//   fn r#getEnergyMeterInfo<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>>>;
//   fn r#readEnergyMeter<'a, >(&'a self, _arg_channelIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>>>;
// }
// #[::async_trait::async_trait]
// pub trait IPowerStatsAsyncServer: binder::Interface + Send {
//   fn get_descriptor() -> &'static str where Self: Sized { "android.hardware.power.stats.IPowerStats" }
//   async fn r#getPowerEntityInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>>;
//   async fn r#getStateResidency<'a, 'l1, >(&'a self, _arg_powerEntityIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>>;
//   async fn r#getEnergyConsumerInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>>;
//   async fn r#getEnergyConsumed<'a, 'l1, >(&'a self, _arg_energyConsumerIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>>;
//   async fn r#getEnergyMeterInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>>;
//   async fn r#readEnergyMeter<'a, 'l1, >(&'a self, _arg_channelIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>>;
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
//     impl<T, R> binder::Interface for Wrapper<T, R> where T: binder::Interface, R: Send + Sync + 'static {
//       fn as_binder(&self) -> binder::SpIBinder { self._inner.as_binder() }
//       #[cfg(feature = "std")]
//       fn dump(&self, _writer: &mut dyn std::io::Write, _args: &[&std::ffi::CStr]) -> core::result::Result<(), binder::StatusCode> { self._inner.dump(_writer, _args) }
//     }
//     impl<T, R> IPowerStats for Wrapper<T, R>
//     where
//       T: IPowerStatsAsyncServer + Send + Sync + 'static,
//       R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
//     {
//       fn r#getPowerEntityInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>> {
//         self._rt.block_on(self._inner.r#getPowerEntityInfo())
//       }
//       fn r#getStateResidency<'a, 'l1, >(&'a self, _arg_powerEntityIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>> {
//         self._rt.block_on(self._inner.r#getStateResidency(_arg_powerEntityIds))
//       }
//       fn r#getEnergyConsumerInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>> {
//         self._rt.block_on(self._inner.r#getEnergyConsumerInfo())
//       }
//       fn r#getEnergyConsumed<'a, 'l1, >(&'a self, _arg_energyConsumerIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>> {
//         self._rt.block_on(self._inner.r#getEnergyConsumed(_arg_energyConsumerIds))
//       }
//       fn r#getEnergyMeterInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>> {
//         self._rt.block_on(self._inner.r#getEnergyMeterInfo())
//       }
//       fn r#readEnergyMeter<'a, 'l1, >(&'a self, _arg_channelIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>> {
//         self._rt.block_on(self._inner.r#readEnergyMeter(_arg_channelIds))
//       }
//       fn try_as_async_server(&self) -> Option<&(dyn IPowerStatsAsyncServer + Send + Sync)> {
//         Some(&self._inner)
//       }
//     }
//     let wrapped = Wrapper { _inner: inner, _rt: rt };
//     Self::new_binder(wrapped, features)
//   }
//   pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IPowerStatsAsync<P>>> {
//     struct Wrapper {
//       _native: binder::binder_impl::Binder<BnPowerStats>
//     }
//     impl binder::Interface for Wrapper {}
//     impl<P: binder::BinderAsyncPool> IPowerStatsAsync<P> for Wrapper {
//       fn r#getPowerEntityInfo<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>>> {
//         Box::pin(self._native.try_as_async_server().unwrap().r#getPowerEntityInfo())
//       }
//       fn r#getStateResidency<'a, >(&'a self, _arg_powerEntityIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>>> {
//         Box::pin(self._native.try_as_async_server().unwrap().r#getStateResidency(_arg_powerEntityIds))
//       }
//       fn r#getEnergyConsumerInfo<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>>> {
//         Box::pin(self._native.try_as_async_server().unwrap().r#getEnergyConsumerInfo())
//       }
//       fn r#getEnergyConsumed<'a, >(&'a self, _arg_energyConsumerIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>>> {
//         Box::pin(self._native.try_as_async_server().unwrap().r#getEnergyConsumed(_arg_energyConsumerIds))
//       }
//       fn r#getEnergyMeterInfo<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>>> {
//         Box::pin(self._native.try_as_async_server().unwrap().r#getEnergyMeterInfo())
//       }
//       fn r#readEnergyMeter<'a, >(&'a self, _arg_channelIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>>> {
//         Box::pin(self._native.try_as_async_server().unwrap().r#readEnergyMeter(_arg_channelIds))
//       }
//     }
//     if _native.try_as_async_server().is_some() {
//       Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IPowerStatsAsync<P>>))
//     } else {
//       None
//     }
//   }
// }
pub mod transactions {
  pub const r#getPowerEntityInfo: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const r#getStateResidency: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
  pub const r#getEnergyConsumerInfo: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 2;
  pub const r#getEnergyConsumed: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 3;
  pub const r#getEnergyMeterInfo: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 4;
  pub const r#readEnergyMeter: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 5;
}
impl BpPowerStats {
  fn build_parcel_getPowerEntityInfo(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getPowerEntityInfo(&self, _aidl_reply: core::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>> {
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getStateResidency(&self, _arg_powerEntityIds: &[i32]) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_powerEntityIds)?;
    Ok(aidl_data)
  }
  fn read_response_getStateResidency(&self, _arg_powerEntityIds: &[i32], _aidl_reply: core::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>> {
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getEnergyConsumerInfo(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getEnergyConsumerInfo(&self, _aidl_reply: core::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>> {
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getEnergyConsumed(&self, _arg_energyConsumerIds: &[i32]) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_energyConsumerIds)?;
    Ok(aidl_data)
  }
  fn read_response_getEnergyConsumed(&self, _arg_energyConsumerIds: &[i32], _aidl_reply: core::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>> {
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_getEnergyMeterInfo(&self) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    Ok(aidl_data)
  }
  fn read_response_getEnergyMeterInfo(&self, _aidl_reply: core::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>> {
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
  fn build_parcel_readEnergyMeter(&self, _arg_channelIds: &[i32]) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_channelIds)?;
    Ok(aidl_data)
  }
  fn read_response_readEnergyMeter(&self, _arg_channelIds: &[i32], _aidl_reply: core::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>> {
    let _aidl_reply = _aidl_reply?;
    let _aidl_status: binder::Status = _aidl_reply.read()?;
    if !_aidl_status.is_ok() { return Err(_aidl_status); }
    let _aidl_return: alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement> = _aidl_reply.read()?;
    Ok(_aidl_return)
  }
}
impl IPowerStats for BpPowerStats {
  fn r#getPowerEntityInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>> {
    let _aidl_data = self.build_parcel_getPowerEntityInfo()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getPowerEntityInfo, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getPowerEntityInfo(_aidl_reply)
  }
  fn r#getStateResidency<'a, 'l1, >(&'a self, _arg_powerEntityIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>> {
    let _aidl_data = self.build_parcel_getStateResidency(_arg_powerEntityIds)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getStateResidency, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getStateResidency(_arg_powerEntityIds, _aidl_reply)
  }
  fn r#getEnergyConsumerInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>> {
    let _aidl_data = self.build_parcel_getEnergyConsumerInfo()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getEnergyConsumerInfo, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getEnergyConsumerInfo(_aidl_reply)
  }
  fn r#getEnergyConsumed<'a, 'l1, >(&'a self, _arg_energyConsumerIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>> {
    let _aidl_data = self.build_parcel_getEnergyConsumed(_arg_energyConsumerIds)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getEnergyConsumed, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getEnergyConsumed(_arg_energyConsumerIds, _aidl_reply)
  }
  fn r#getEnergyMeterInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>> {
    let _aidl_data = self.build_parcel_getEnergyMeterInfo()?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getEnergyMeterInfo, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_getEnergyMeterInfo(_aidl_reply)
  }
  fn r#readEnergyMeter<'a, 'l1, >(&'a self, _arg_channelIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>> {
    let _aidl_data = self.build_parcel_readEnergyMeter(_arg_channelIds)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#readEnergyMeter, _aidl_data, FLAG_PRIVATE_LOCAL);
    self.read_response_readEnergyMeter(_arg_channelIds, _aidl_reply)
  }
}
// impl<P: binder::BinderAsyncPool> IPowerStatsAsync<P> for BpPowerStats {
//   fn r#getPowerEntityInfo<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>>> {
//     let _aidl_data = match self.build_parcel_getPowerEntityInfo() {
//       Ok(_aidl_data) => _aidl_data,
//       Err(err) => return Box::pin(core::future::ready(Err(err))),
//     };
//     let binder = self.binder.clone();
//     P::spawn(
//       move || binder.submit_transact(transactions::r#getPowerEntityInfo, _aidl_data, FLAG_PRIVATE_LOCAL),
//       move |_aidl_reply| async move {
//         self.read_response_getPowerEntityInfo(_aidl_reply)
//       }
//     )
//   }
//   fn r#getStateResidency<'a, >(&'a self, _arg_powerEntityIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>>> {
//     let _aidl_data = match self.build_parcel_getStateResidency(_arg_powerEntityIds) {
//       Ok(_aidl_data) => _aidl_data,
//       Err(err) => return Box::pin(core::future::ready(Err(err))),
//     };
//     let binder = self.binder.clone();
//     P::spawn(
//       move || binder.submit_transact(transactions::r#getStateResidency, _aidl_data, FLAG_PRIVATE_LOCAL),
//       move |_aidl_reply| async move {
//         self.read_response_getStateResidency(_arg_powerEntityIds, _aidl_reply)
//       }
//     )
//   }
//   fn r#getEnergyConsumerInfo<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>>> {
//     let _aidl_data = match self.build_parcel_getEnergyConsumerInfo() {
//       Ok(_aidl_data) => _aidl_data,
//       Err(err) => return Box::pin(core::future::ready(Err(err))),
//     };
//     let binder = self.binder.clone();
//     P::spawn(
//       move || binder.submit_transact(transactions::r#getEnergyConsumerInfo, _aidl_data, FLAG_PRIVATE_LOCAL),
//       move |_aidl_reply| async move {
//         self.read_response_getEnergyConsumerInfo(_aidl_reply)
//       }
//     )
//   }
//   fn r#getEnergyConsumed<'a, >(&'a self, _arg_energyConsumerIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>>> {
//     let _aidl_data = match self.build_parcel_getEnergyConsumed(_arg_energyConsumerIds) {
//       Ok(_aidl_data) => _aidl_data,
//       Err(err) => return Box::pin(core::future::ready(Err(err))),
//     };
//     let binder = self.binder.clone();
//     P::spawn(
//       move || binder.submit_transact(transactions::r#getEnergyConsumed, _aidl_data, FLAG_PRIVATE_LOCAL),
//       move |_aidl_reply| async move {
//         self.read_response_getEnergyConsumed(_arg_energyConsumerIds, _aidl_reply)
//       }
//     )
//   }
//   fn r#getEnergyMeterInfo<'a, >(&'a self) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>>> {
//     let _aidl_data = match self.build_parcel_getEnergyMeterInfo() {
//       Ok(_aidl_data) => _aidl_data,
//       Err(err) => return Box::pin(core::future::ready(Err(err))),
//     };
//     let binder = self.binder.clone();
//     P::spawn(
//       move || binder.submit_transact(transactions::r#getEnergyMeterInfo, _aidl_data, FLAG_PRIVATE_LOCAL),
//       move |_aidl_reply| async move {
//         self.read_response_getEnergyMeterInfo(_aidl_reply)
//       }
//     )
//   }
//   fn r#readEnergyMeter<'a, >(&'a self, _arg_channelIds: &'a [i32]) -> binder::BoxFuture<'a, binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>>> {
//     let _aidl_data = match self.build_parcel_readEnergyMeter(_arg_channelIds) {
//       Ok(_aidl_data) => _aidl_data,
//       Err(err) => return Box::pin(core::future::ready(Err(err))),
//     };
//     let binder = self.binder.clone();
//     P::spawn(
//       move || binder.submit_transact(transactions::r#readEnergyMeter, _aidl_data, FLAG_PRIVATE_LOCAL),
//       move |_aidl_reply| async move {
//         self.read_response_readEnergyMeter(_arg_channelIds, _aidl_reply)
//       }
//     )
//   }
// }
impl IPowerStats for binder::binder_impl::Binder<BnPowerStats> {
  fn r#getPowerEntityInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_11_PowerEntity>> { self.0.r#getPowerEntityInfo() }
  fn r#getStateResidency<'a, 'l1, >(&'a self, _arg_powerEntityIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_StateResidencyResult>> { self.0.r#getStateResidency(_arg_powerEntityIds) }
  fn r#getEnergyConsumerInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_14_EnergyConsumer>> { self.0.r#getEnergyConsumerInfo() }
  fn r#getEnergyConsumed<'a, 'l1, >(&'a self, _arg_energyConsumerIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_20_EnergyConsumerResult>> { self.0.r#getEnergyConsumed(_arg_energyConsumerIds) }
  fn r#getEnergyMeterInfo<'a, >(&'a self) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_7_Channel>> { self.0.r#getEnergyMeterInfo() }
  fn r#readEnergyMeter<'a, 'l1, >(&'a self, _arg_channelIds: &'l1 [i32]) -> binder::Result<alloc::vec::Vec<crate::mangled::_7_android_8_hardware_5_power_5_stats_17_EnergyMeasurement>> { self.0.r#readEnergyMeter(_arg_channelIds) }
}
fn on_transact(_aidl_service: &dyn IPowerStats, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> core::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#getPowerEntityInfo => {
      let _aidl_return = _aidl_service.r#getPowerEntityInfo();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getStateResidency => {
      let _arg_powerEntityIds: alloc::vec::Vec<i32> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getStateResidency(&_arg_powerEntityIds);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getEnergyConsumerInfo => {
      let _aidl_return = _aidl_service.r#getEnergyConsumerInfo();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getEnergyConsumed => {
      let _arg_energyConsumerIds: alloc::vec::Vec<i32> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getEnergyConsumed(&_arg_energyConsumerIds);
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#getEnergyMeterInfo => {
      let _aidl_return = _aidl_service.r#getEnergyMeterInfo();
      match &_aidl_return {
        Ok(_aidl_return) => {
          _aidl_reply.write(&binder::Status::from(binder::StatusCode::OK))?;
          _aidl_reply.write(_aidl_return)?;
        }
        Err(_aidl_status) => _aidl_reply.write(_aidl_status)?
      }
      Ok(())
    }
    transactions::r#readEnergyMeter => {
      let _arg_channelIds: alloc::vec::Vec<i32> = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#readEnergyMeter(&_arg_channelIds);
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
 pub use super::r#IPowerStats as _7_android_8_hardware_5_power_5_stats_11_IPowerStats;
}
