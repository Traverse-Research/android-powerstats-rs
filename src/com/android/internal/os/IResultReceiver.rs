#![forbid(unsafe_code)]
// #![rustfmt::skip]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)]
use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  IResultReceiver["com.android.internal.os.IResultReceiver"] {
    native: BnResultReceiver(on_transact),
    proxy: BpResultReceiver {
    },
    async: IResultReceiverAsync,
  }
}
pub trait IResultReceiver: binder::Interface + Send {
    fn get_descriptor() -> &'static str
    where
        Self: Sized,
    {
        "com.android.internal.os.IResultReceiver"
    }
    fn r#send(
        &self,
        _arg_resultCode: i32,
        _arg_resultData: &crate::mangled::_7_android_2_os_6_Bundle,
    ) -> binder::Result<()>;
    fn getDefaultImpl() -> IResultReceiverDefaultRef
    where
        Self: Sized,
    {
        DEFAULT_IMPL.lock().unwrap().clone()
    }
    fn setDefaultImpl(d: IResultReceiverDefaultRef) -> IResultReceiverDefaultRef
    where
        Self: Sized,
    {
        std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
    }
}
pub trait IResultReceiverAsync<P>: binder::Interface + Send {
    fn get_descriptor() -> &'static str
    where
        Self: Sized,
    {
        "com.android.internal.os.IResultReceiver"
    }
    fn r#send(
        &self,
        _arg_resultCode: i32,
        _arg_resultData: &crate::mangled::_7_android_2_os_6_Bundle,
    ) -> std::future::Ready<binder::Result<()>>;
}
// #[::async_trait::async_trait]
// pub trait IResultReceiverAsyncServer: binder::Interface + Send {
//     fn get_descriptor() -> &'static str
//     where
//         Self: Sized,
//     {
//         "com.android.internal.os.IResultReceiver"
//     }
//     async fn r#send(
//         &self,
//         _arg_resultCode: i32,
//         _arg_resultData: &crate::mangled::_7_android_2_os_6_Bundle,
//     ) -> binder::Result<()>;
// }
// impl BnResultReceiver {
//     /// Create a new async binder service.
//     pub fn new_async_binder<T, R>(
//         inner: T,
//         rt: R,
//         features: binder::BinderFeatures,
//     ) -> binder::Strong<dyn IResultReceiver>
//     where
//         T: IResultReceiverAsyncServer + binder::Interface + Send + Sync + 'static,
//         R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
//     {
//         struct Wrapper<T, R> {
//             _inner: T,
//             _rt: R,
//         }
//         impl<T, R> binder::Interface for Wrapper<T, R>
//         where
//             T: binder::Interface,
//             R: Send + Sync,
//         {
//             fn as_binder(&self) -> binder::SpIBinder {
//                 self._inner.as_binder()
//             }
//             fn dump(
//                 &self,
//                 _file: &std::fs::File,
//                 _args: &[&std::ffi::CStr],
//             ) -> std::result::Result<(), binder::StatusCode> {
//                 self._inner.dump(_file, _args)
//             }
//         }
//         impl<T, R> IResultReceiver for Wrapper<T, R>
//         where
//             T: IResultReceiverAsyncServer + Send + Sync + 'static,
//             R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
//         {
//             fn r#send(
//                 &self,
//                 _arg_resultCode: i32,
//                 _arg_resultData: &crate::mangled::_7_android_2_os_6_Bundle,
//             ) -> binder::Result<()> {
//                 self._rt
//                     .block_on(self._inner.r#send(_arg_resultCode, _arg_resultData))
//             }
//         }
//         let wrapped = Wrapper {
//             _inner: inner,
//             _rt: rt,
//         };
//         Self::new_binder(wrapped, features)
//     }
// }
pub trait IResultReceiverDefault: Send + Sync {
    fn r#send(
        &self,
        _arg_resultCode: i32,
        _arg_resultData: &crate::mangled::_7_android_2_os_6_Bundle,
    ) -> binder::Result<()> {
        Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
    }
}
pub mod transactions {
    pub const r#send: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
}
pub type IResultReceiverDefaultRef = Option<std::sync::Arc<dyn IResultReceiverDefault>>;
use lazy_static::lazy_static;
lazy_static! {
    static ref DEFAULT_IMPL: std::sync::Mutex<IResultReceiverDefaultRef> =
        std::sync::Mutex::new(None);
}
impl BpResultReceiver {
    fn build_parcel_send(
        &self,
        _arg_resultCode: i32,
        _arg_resultData: &crate::mangled::_7_android_2_os_6_Bundle,
    ) -> binder::Result<binder::binder_impl::Parcel> {
        let mut aidl_data = self.binder.prepare_transact()?;
        aidl_data.write(&_arg_resultCode)?;
        aidl_data.write(_arg_resultData)?;
        Ok(aidl_data)
    }
    fn read_response_send(
        &self,
        _arg_resultCode: i32,
        _arg_resultData: &crate::mangled::_7_android_2_os_6_Bundle,
        _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>,
    ) -> binder::Result<()> {
        if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
            if let Some(_aidl_default_impl) = <Self as IResultReceiver>::getDefaultImpl() {
                return _aidl_default_impl.r#send(_arg_resultCode, _arg_resultData);
            }
        }
        let _aidl_reply = _aidl_reply?;
        Ok(())
    }
}
impl IResultReceiver for BpResultReceiver {
    fn r#send(
        &self,
        _arg_resultCode: i32,
        _arg_resultData: &crate::mangled::_7_android_2_os_6_Bundle,
    ) -> binder::Result<()> {
        let _aidl_data = self.build_parcel_send(_arg_resultCode, _arg_resultData)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#send,
            _aidl_data,
            binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_send(_arg_resultCode, _arg_resultData, _aidl_reply)
    }
}
impl<P: binder::BinderAsyncPool> IResultReceiverAsync<P> for BpResultReceiver {
    fn r#send(
        &self,
        _arg_resultCode: i32,
        _arg_resultData: &crate::mangled::_7_android_2_os_6_Bundle,
    ) -> std::future::Ready<binder::Result<()>> {
        let _aidl_data = match self.build_parcel_send(_arg_resultCode, _arg_resultData) {
            Ok(_aidl_data) => _aidl_data,
            Err(err) => return std::future::ready(Err(err)),
        };
        let _aidl_reply = self.binder.submit_transact(
            transactions::r#send,
            _aidl_data,
            binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        std::future::ready(self.read_response_send(_arg_resultCode, _arg_resultData, _aidl_reply))
    }
}
impl IResultReceiver for binder::binder_impl::Binder<BnResultReceiver> {
    fn r#send(
        &self,
        _arg_resultCode: i32,
        _arg_resultData: &crate::mangled::_7_android_2_os_6_Bundle,
    ) -> binder::Result<()> {
        self.0.r#send(_arg_resultCode, _arg_resultData)
    }
}
fn on_transact(
    _aidl_service: &dyn IResultReceiver,
    _aidl_code: binder::binder_impl::TransactionCode,
    _aidl_data: &binder::binder_impl::BorrowedParcel<'_>,
    _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>,
) -> std::result::Result<(), binder::StatusCode> {
    match _aidl_code {
        transactions::r#send => {
            let _arg_resultCode: i32 = _aidl_data.read()?;
            let _arg_resultData: crate::mangled::_7_android_2_os_6_Bundle = _aidl_data.read()?;
            let _aidl_return = _aidl_service.r#send(_arg_resultCode, &_arg_resultData);
            Ok(())
        }
        _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION),
    }
}
// pub(crate) mod mangled {
//     pub use super::r#IResultReceiver as _3_com_7_android_8_internal_2_os_15_IResultReceiver;
// }
