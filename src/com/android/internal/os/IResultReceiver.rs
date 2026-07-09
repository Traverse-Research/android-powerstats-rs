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
  IResultReceiver["com.android.internal.os.IResultReceiver"] {
    native: BnResultReceiver(on_transact),
    proxy: BpResultReceiver {
    },
    // async: IResultReceiverAsync(try_into_local_async),
  }
}
pub trait IResultReceiver: binder::Interface + Send {
  fn get_descriptor() -> &'static str where Self: Sized { "com.android.internal.os.IResultReceiver" }
  fn r#send<'a, 'l1, >(&'a self, _arg_resultCode: i32, _arg_resultData: &'l1 crate::mangled::_7_android_2_os_6_Bundle) -> binder::Result<()>;
  // fn try_as_async_server<'a>(&'a self) -> Option<&'a (dyn IResultReceiverAsyncServer + Send + Sync)> {
  //   None
  // }
}
// pub trait IResultReceiverAsync<P>: binder::Interface + Send {
//   fn get_descriptor() -> &'static str where Self: Sized { "com.android.internal.os.IResultReceiver" }
//   fn r#send<'a, >(&'a self, _arg_resultCode: i32, _arg_resultData: &'a crate::mangled::_7_android_2_os_6_Bundle) -> binder::BoxFuture<'a, binder::Result<()>>;
// }
// #[::async_trait::async_trait]
// pub trait IResultReceiverAsyncServer: binder::Interface + Send {
//   fn get_descriptor() -> &'static str where Self: Sized { "com.android.internal.os.IResultReceiver" }
//   async fn r#send<'a, 'l1, >(&'a self, _arg_resultCode: i32, _arg_resultData: &'l1 crate::mangled::_7_android_2_os_6_Bundle) -> binder::Result<()>;
// }
// impl BnResultReceiver {
//   /// Create a new async binder service.
//   pub fn new_async_binder<T, R>(inner: T, rt: R, features: binder::BinderFeatures) -> binder::Strong<dyn IResultReceiver>
//   where
//     T: IResultReceiverAsyncServer + binder::Interface + Send + Sync + 'static,
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
//     impl<T, R> IResultReceiver for Wrapper<T, R>
//     where
//       T: IResultReceiverAsyncServer + Send + Sync + 'static,
//       R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
//     {
//       fn r#send<'a, 'l1, >(&'a self, _arg_resultCode: i32, _arg_resultData: &'l1 crate::mangled::_7_android_2_os_6_Bundle) -> binder::Result<()> {
//         self._rt.block_on(self._inner.r#send(_arg_resultCode, _arg_resultData))
//       }
//       fn try_as_async_server(&self) -> Option<&(dyn IResultReceiverAsyncServer + Send + Sync)> {
//         Some(&self._inner)
//       }
//     }
//     let wrapped = Wrapper { _inner: inner, _rt: rt };
//     Self::new_binder(wrapped, features)
//   }
//   pub fn try_into_local_async<P: binder::BinderAsyncPool + 'static>(_native: binder::binder_impl::Binder<Self>) -> Option<binder::Strong<dyn IResultReceiverAsync<P>>> {
//     struct Wrapper {
//       _native: binder::binder_impl::Binder<BnResultReceiver>
//     }
//     impl binder::Interface for Wrapper {}
//     impl<P: binder::BinderAsyncPool> IResultReceiverAsync<P> for Wrapper {
//       fn r#send<'a, >(&'a self, _arg_resultCode: i32, _arg_resultData: &'a crate::mangled::_7_android_2_os_6_Bundle) -> binder::BoxFuture<'a, binder::Result<()>> {
//         Box::pin(self._native.try_as_async_server().unwrap().r#send(_arg_resultCode, _arg_resultData))
//       }
//     }
//     if _native.try_as_async_server().is_some() {
//       Some(binder::Strong::new(Box::new(Wrapper { _native }) as Box<dyn IResultReceiverAsync<P>>))
//     } else {
//       None
//     }
//   }
// }
pub mod transactions {
  pub const r#send: binder::binder_impl::TransactionCode = binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
}
impl BpResultReceiver {
  fn build_parcel_send(&self, _arg_resultCode: i32, _arg_resultData: &crate::mangled::_7_android_2_os_6_Bundle) -> binder::Result<binder::binder_impl::Parcel> {
    let mut aidl_data = self.binder.prepare_transact()?;
    aidl_data.write(&_arg_resultCode)?;
    aidl_data.write(_arg_resultData)?;
    Ok(aidl_data)
  }
  fn read_response_send(&self, _arg_resultCode: i32, _arg_resultData: &crate::mangled::_7_android_2_os_6_Bundle, _aidl_reply: core::result::Result<binder::binder_impl::Parcel, binder::StatusCode>) -> binder::Result<()> {
    let _aidl_reply = _aidl_reply?;
    Ok(())
  }
}
impl IResultReceiver for BpResultReceiver {
  fn r#send<'a, 'l1, >(&'a self, _arg_resultCode: i32, _arg_resultData: &'l1 crate::mangled::_7_android_2_os_6_Bundle) -> binder::Result<()> {
    let _aidl_data = self.build_parcel_send(_arg_resultCode, _arg_resultData)?;
    let _aidl_reply = self.binder.submit_transact(transactions::r#send, _aidl_data, binder::binder_impl::FLAG_ONEWAY | FLAG_PRIVATE_LOCAL);
    self.read_response_send(_arg_resultCode, _arg_resultData, _aidl_reply)
  }
}
// impl<P: binder::BinderAsyncPool> IResultReceiverAsync<P> for BpResultReceiver {
//   fn r#send<'a, >(&'a self, _arg_resultCode: i32, _arg_resultData: &'a crate::mangled::_7_android_2_os_6_Bundle) -> binder::BoxFuture<'a, binder::Result<()>> {
//     let _aidl_data = match self.build_parcel_send(_arg_resultCode, _arg_resultData) {
//       Ok(_aidl_data) => _aidl_data,
//       Err(err) => return Box::pin(core::future::ready(Err(err))),
//     };
//     let binder = self.binder.clone();
//     P::spawn(
//       move || binder.submit_transact(transactions::r#send, _aidl_data, binder::binder_impl::FLAG_ONEWAY | FLAG_PRIVATE_LOCAL),
//       move |_aidl_reply| async move {
//         self.read_response_send(_arg_resultCode, _arg_resultData, _aidl_reply)
//       }
//     )
//   }
// }
impl IResultReceiver for binder::binder_impl::Binder<BnResultReceiver> {
  fn r#send<'a, 'l1, >(&'a self, _arg_resultCode: i32, _arg_resultData: &'l1 crate::mangled::_7_android_2_os_6_Bundle) -> binder::Result<()> { self.0.r#send(_arg_resultCode, _arg_resultData) }
}
fn on_transact(_aidl_service: &dyn IResultReceiver, _aidl_code: binder::binder_impl::TransactionCode, _aidl_data: &binder::binder_impl::BorrowedParcel<'_>, _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>) -> core::result::Result<(), binder::StatusCode> {
  match _aidl_code {
    transactions::r#send => {
      let _arg_resultCode: i32 = _aidl_data.read()?;
      let _arg_resultData: crate::mangled::_7_android_2_os_6_Bundle = _aidl_data.read()?;
      let _aidl_return = _aidl_service.r#send(_arg_resultCode, &_arg_resultData);
      Ok(())
    }
    _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION)
  }
}
pub(crate) mod mangled {
 pub use super::r#IResultReceiver as _3_com_7_android_8_internal_2_os_15_IResultReceiver;
}
