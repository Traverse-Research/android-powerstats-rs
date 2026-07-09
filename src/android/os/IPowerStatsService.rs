/*
 * This file is auto-generated.  DO NOT MODIFY.
 * Using: aidl --structured --lang=rust -Iandroid-frameworks-base/core/java android-frameworks-base/core/java/android/os/IPowerStatsService.aidl android-frameworks-base/core/java/com/android/internal/os/IResultReceiver.aidl -o src/
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
  IPowerStatsService["android.os.IPowerStatsService"] {
    native: BnPowerStatsService(on_transact),
    proxy: BpPowerStatsService {
    },
    // async: IPowerStatsServiceAsync(try_into_local_async),
  }
}
pub trait IPowerStatsService: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "android.os.IPowerStatsService" }
  fn r#getSupportedPowerMonitors<'a, 'l1, >(&'a self, _arg_resultReceiver: &'l1 crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::Result<()>;
  fn r#getPowerMonitorReadings<'a, 'l1, 'l2, >(&'a self, _arg_powerMonitorIndices: &'l1 [i32], _arg_resultReceiver: &'l2 crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::Result<()>;
  // fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IPowerStatsServiceAsyncServer + Send + Sync)> {
  //   None
  // }
}
// pub trait IPowerStatsServiceAsync<P>: binder::Interface + Send {
//   fn get_descriptor() -> &'static str where Self: Sized { "android.os.IPowerStatsService" }
//   fn r#getSupportedPowerMonitors<'a, >(&'a self, _arg_resultReceiver: &'a crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::BoxFuture<'a, binder::Result<()>>;
//   fn r#getPowerMonitorReadings<'a, >(&'a self, _arg_powerMonitorIndices: &'a [i32], _arg_resultReceiver: &'a crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::BoxFuture<'a, binder::Result<()>>;
// }
// #[::async_trait::async_trait]
// pub trait IPowerStatsServiceAsyncServer: binder::Interface + Send {
//   fn get_descriptor() -> &'static str where Self: Sized { "android.os.IPowerStatsService" }
//   async fn r#getSupportedPowerMonitors<'a, 'l1, >(&'a self, _arg_resultReceiver: &'l1 crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::Result<()>;
//   async fn r#getPowerMonitorReadings<'a, 'l1, 'l2, >(&'a self, _arg_powerMonitorIndices: &'l1 [i32], _arg_resultReceiver: &'l2 crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::Result<()>;
// }
// impl BnPowerStatsService {
//   /// Create a new async binder service.
//   pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IPowerStatsService>
//   where
//     T: IPowerStatsServiceAsyncServer + binder::Interface + Send + Sync + 'static,
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
//     impl<T, R> IPowerStatsService for Wrapper<T, R>
//     where
//       T: IPowerStatsServiceAsyncServer + Send + Sync + 'static,
//       R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
//     {
//       fn r#getSupportedPowerMonitors<'a, 'l1, >(&'a self, _arg_resultReceiver: &'l1 crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::Result<()> {
//         self._rt.block_on(self._inner.r#getSupportedPowerMonitors(_arg_resultReceiver))
//       }
//       fn r#getPowerMonitorReadings<'a, 'l1, 'l2, >(&'a self, _arg_powerMonitorIndices: &'l1 [i32], _arg_resultReceiver: &'l2 crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::Result<()> {
//         self._rt.block_on(self._inner.r#getPowerMonitorReadings(_arg_powerMonitorIndices, _arg_resultReceiver))
//       }
//       fn try_as_async_server(&self) -> Option<&(dyn IPowerStatsServiceAsyncServer + Send + Sync)> {
//         Some(&self._inner)
//       }
//     }
//     let wrapped = Wrapper { _inner: inner, _rt: rt };
//     Self::new_binder(wrapped, features)
//   }
//   pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IPowerStatsServiceAsync<P>>> {
//     struct Wrapper {
//       _native: binder::binder_impl::Binder<BnPowerStatsService>
//     }
//     impl binder::Interface for Wrapper {}
//     impl<P: binder::BinderAsyncPool> IPowerStatsServiceAsync<P> for Wrapper {
//       fn r#getSupportedPowerMonitors<'a, >(&'a self, _arg_resultReceiver: &'a crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::BoxFuture<'a, binder::Result<()>> {
//         Box::pin(self._native.try_as_async_server().unwrap().r#getSupportedPowerMonitors(_arg_resultReceiver))
//       }
//       fn r#getPowerMonitorReadings<'a, >(&'a self, _arg_powerMonitorIndices: &'a [i32], _arg_resultReceiver: &'a crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::BoxFuture<'a, binder::Result<()>> {
//         Box::pin(self._native.try_as_async_server().unwrap().r#getPowerMonitorReadings(_arg_powerMonitorIndices, _arg_resultReceiver))
//       }
//     }
//     if _native.try_as_async_server().is_some() {
//       Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IPowerStatsServiceAsync<P>>))
//     } else {
//       None
//     }
//   }
// }
pub mod transactions {
  pub const r#getSupportedPowerMonitors: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
  pub const r#getPowerMonitorReadings: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
}
pub const r#KEY_MONITORS: &str = "monitors";
pub const r#KEY_ENERGY: &str = "energy";
pub const r#KEY_TIMESTAMPS: &str = "timestamps";
pub const r#KEY_GRANULARITY: &str = "granularity";
pub const r#RESULT_SUCCESS: i32 = 0;
pub const r#RESULT_UNSUPPORTED_POWER_MONITOR: i32 = 1;
impl BpPowerStatsService {
  fn build_parcel_getSupportedPowerMonitors(&self, _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_resultReceiver)?;
    Ok(aidl_data)
  }
  fn read_response_getSupportedPowerMonitors(&self, _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver, _aidl_reply: core::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    let _aidl_reply = _aidl_reply?;
    Ok(())
  }
  fn build_parcel_getPowerMonitorReadings(&self, _arg_powerMonitorIndices: &[i32], _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(_arg_powerMonitorIndices)?;
    aidl_data.write(_arg_resultReceiver)?;
    Ok(aidl_data)
  }
  fn read_response_getPowerMonitorReadings(&self, _arg_powerMonitorIndices: &[i32], _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver, _aidl_reply: core::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    let _aidl_reply = _aidl_reply?;
    Ok(())
  }
}
impl IPowerStatsService for BpPowerStatsService {
  fn r#getSupportedPowerMonitors<'a, 'l1, >(&'a self, _arg_resultReceiver: &'l1 crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_getSupportedPowerMonitors(_arg_resultReceiver)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getSupportedPowerMonitors, _aidl_data, binder::binder_impl::FLAG_ONEWAY | FLAG_PRIVATE_LOCAL);
    self.read_response_getSupportedPowerMonitors(_arg_resultReceiver, _aidl_reply)
  }
  fn r#getPowerMonitorReadings<'a, 'l1, 'l2, >(&'a self, _arg_powerMonitorIndices: &'l1 [i32], _arg_resultReceiver: &'l2 crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_getPowerMonitorReadings(_arg_powerMonitorIndices, _arg_resultReceiver)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#getPowerMonitorReadings, _aidl_data, binder::binder_impl::FLAG_ONEWAY | FLAG_PRIVATE_LOCAL);
    self.read_response_getPowerMonitorReadings(_arg_powerMonitorIndices, _arg_resultReceiver, _aidl_reply)
  }
}
// impl<P: binder::BinderAsyncPool> IPowerStatsServiceAsync<P> for BpPowerStatsService {
//   fn r#getSupportedPowerMonitors<'a, >(&'a self, _arg_resultReceiver: &'a crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::BoxFuture<'a, binder::Result<()>> {
//     let _aidl_data = match self.build_parcel_getSupportedPowerMonitors(_arg_resultReceiver) {
//       Ok(_aidl_data) => _aidl_data,
//       Err(err) => return Box::pin(core::future::ready(Err(err))),
//     };
//     let binder = self.binder.clone();
//     P::spawn(
//       move || binder.submit_transact(transactions::r#getSupportedPowerMonitors, _aidl_data, binder::binder_impl::FLAG_ONEWAY | FLAG_PRIVATE_LOCAL),
//       move |_aidl_reply| async move {
//         self.read_response_getSupportedPowerMonitors(_arg_resultReceiver, _aidl_reply)
//       }
//     )
//   }
//   fn r#getPowerMonitorReadings<'a, >(&'a self, _arg_powerMonitorIndices: &'a [i32], _arg_resultReceiver: &'a crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::BoxFuture<'a, binder::Result<()>> {
//     let _aidl_data = match self.build_parcel_getPowerMonitorReadings(_arg_powerMonitorIndices, _arg_resultReceiver) {
//       Ok(_aidl_data) => _aidl_data,
//       Err(err) => return Box::pin(core::future::ready(Err(err))),
//     };
//     let binder = self.binder.clone();
//     P::spawn(
//       move || binder.submit_transact(transactions::r#getPowerMonitorReadings, _aidl_data, binder::binder_impl::FLAG_ONEWAY | FLAG_PRIVATE_LOCAL),
//       move |_aidl_reply| async move {
//         self.read_response_getPowerMonitorReadings(_arg_powerMonitorIndices, _arg_resultReceiver, _aidl_reply)
//       }
//     )
//   }
// }
impl IPowerStatsService for binder::binder_impl::Binder<BnPowerStatsService> {
  fn r#getSupportedPowerMonitors<'a, 'l1, >(&'a self, _arg_resultReceiver: &'l1 crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::Result<()> { self.0.r#getSupportedPowerMonitors(_arg_resultReceiver) }
  fn r#getPowerMonitorReadings<'a, 'l1, 'l2, >(&'a self, _arg_powerMonitorIndices: &'l1 [i32], _arg_resultReceiver: &'l2 crate::mangled::_7_android_2_os_14_ResultReceiver) -> binder::Result<()> { self.0.r#getPowerMonitorReadings(_arg_powerMonitorIndices, _arg_resultReceiver) }
}
fn on_transact(_aidl_service: &dyn IPowerStatsService, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> core::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#getSupportedPowerMonitors => {
      let _arg_resultReceiver: crate::mangled::_7_android_2_os_14_ResultReceiver = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getSupportedPowerMonitors(&_arg_resultReceiver);
      Ok(())
    }
    transactions::r#getPowerMonitorReadings => {
      let _arg_powerMonitorIndices: alloc::vec::Vec<i32> = _aidl_data.read()?;
      let _arg_resultReceiver: crate::mangled::_7_android_2_os_14_ResultReceiver = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#getPowerMonitorReadings(&_arg_powerMonitorIndices, &_arg_resultReceiver);
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IPowerStatsService as _7_android_2_os_18_IPowerStatsService;
}
