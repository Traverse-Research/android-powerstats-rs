#![forbid(unsafe_code)]
// #![rustfmt::skip]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#[allow(unused_imports)]
use binder::binder_impl::IBinderInternal;
use binder::declare_binder_interface;
declare_binder_interface! {
  IPowerStatsService["android.os.IPowerStatsService"] {
    native: BnPowerStatsService(on_transact),
    proxy: BpPowerStatsService {
    },
    async: IPowerStatsServiceAsync,
  }
}
pub trait IPowerStatsService: binder::Interface + Send {
    fn get_descriptor() -> &'static str
    where
        Self: Sized,
    {
        "android.os.IPowerStatsService"
    }
    fn getSupportedPowerMonitors(
        &self,
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> binder::Result<()>;
    fn getPowerMonitorReadings(
        &self,
        _arg_powerMonitorIndices: &[i32],
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> binder::Result<()>;
    fn getDefaultImpl() -> IPowerStatsServiceDefaultRef
    where
        Self: Sized,
    {
        DEFAULT_IMPL.lock().unwrap().clone()
    }
    fn setDefaultImpl(d: IPowerStatsServiceDefaultRef) -> IPowerStatsServiceDefaultRef
    where
        Self: Sized,
    {
        std::mem::replace(&mut *DEFAULT_IMPL.lock().unwrap(), d)
    }
}
pub trait IPowerStatsServiceAsync<P>: binder::Interface + Send {
    fn get_descriptor() -> &'static str
    where
        Self: Sized,
    {
        "android.os.IPowerStatsService"
    }
    fn getSupportedPowerMonitors(
        &self,
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> std::future::Ready<binder::Result<()>>;
    fn getPowerMonitorReadings(
        &self,
        _arg_powerMonitorIndices: &[i32],
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> std::future::Ready<binder::Result<()>>;
}
// #[::async_trait::async_trait]
// pub trait IPowerStatsServiceAsyncServer: binder::Interface + Send {
//     fn get_descriptor() -> &'static str
//     where
//         Self: Sized,
//     {
//         "android.os.IPowerStatsService"
//     }
//     async fn getSupportedPowerMonitors(
//         &self,
//         _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
//     ) -> binder::Result<()>;
//     async fn getPowerMonitorReadings(
//         &self,
//         _arg_powerMonitorIndices: &[i32],
//         _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
//     ) -> binder::Result<()>;
// }
// impl BnPowerStatsService {
//     /// Create a new async binder service.
//     pub fn new_async_binder<T, R>(
//         inner: T,
//         rt: R,
//         features: binder::BinderFeatures,
//     ) -> binder::Strong<dyn IPowerStatsService>
//     where
//         T: IPowerStatsServiceAsyncServer + binder::Interface + Send + Sync + 'static,
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
//         impl<T, R> IPowerStatsService for Wrapper<T, R>
//         where
//             T: IPowerStatsServiceAsyncServer + Send + Sync + 'static,
//             R: binder::binder_impl::BinderAsyncRuntime + Send + Sync + 'static,
//         {
//             fn getSupportedPowerMonitors(
//                 &self,
//                 _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
//             ) -> binder::Result<()> {
//                 self._rt
//                     .block_on(self._inner.getSupportedPowerMonitors(_arg_resultReceiver))
//             }
//             fn getPowerMonitorReadings(
//                 &self,
//                 _arg_powerMonitorIndices: &[i32],
//                 _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
//             ) -> binder::Result<()> {
//                 self._rt.block_on(
//                     self._inner
//                         .getPowerMonitorReadings(_arg_powerMonitorIndices, _arg_resultReceiver),
//                 )
//             }
//         }
//         let wrapped = Wrapper {
//             _inner: inner,
//             _rt: rt,
//         };
//         Self::new_binder(wrapped, features)
//     }
// }
pub trait IPowerStatsServiceDefault: Send + Sync {
    fn getSupportedPowerMonitors(
        &self,
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> binder::Result<()> {
        Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
    }
    fn getPowerMonitorReadings(
        &self,
        _arg_powerMonitorIndices: &[i32],
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> binder::Result<()> {
        Err(binder::StatusCode::UNKNOWN_TRANSACTION.into())
    }
}
pub mod transactions {
    pub const getSupportedPowerMonitors: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 0;
    pub const getPowerMonitorReadings: binder::binder_impl::TransactionCode =
        binder::binder_impl::FIRST_CALL_TRANSACTION + 1;
}
pub type IPowerStatsServiceDefaultRef = Option<std::sync::Arc<dyn IPowerStatsServiceDefault>>;
use lazy_static::lazy_static;
lazy_static! {
    static ref DEFAULT_IMPL: std::sync::Mutex<IPowerStatsServiceDefaultRef> =
        std::sync::Mutex::new(None);
}
pub const KEY_MONITORS: &str = "monitors";
pub const KEY_ENERGY: &str = "energy";
pub const KEY_TIMESTAMPS: &str = "timestamps";
pub const RESULT_SUCCESS: i32 = 0;
pub const RESULT_UNSUPPORTED_POWER_MONITOR: i32 = 1;
impl BpPowerStatsService {
    fn build_parcel_getSupportedPowerMonitors(
        &self,
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> binder::Result<binder::binder_impl::Parcel> {
        let mut aidl_data = self.binder.prepare_transact()?;
        // TODO: This should have written 0/1 first for nullability via writeTypedObject:
        // https://cs.android.com/android/platform/superproject/main/+/main:out/soong/.intermediates/frameworks/base/framework-minus-apex-intdefs/android_common/e18b8e8d84cb9f664aa09a397b08c165/xref50/srcjars.xref/android/os/IPowerStatsService.java;l=135;drc=190beaa49a35da1d9dcf66be9cfccfd23b0eb467
        aidl_data.write(_arg_resultReceiver)?;
        Ok(aidl_data)
    }
    fn read_response_getSupportedPowerMonitors(
        &self,
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
        _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>,
    ) -> binder::Result<()> {
        if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
            if let Some(_aidl_default_impl) = <Self as IPowerStatsService>::getDefaultImpl() {
                return _aidl_default_impl.getSupportedPowerMonitors(_arg_resultReceiver);
            }
        }
        let _aidl_reply = _aidl_reply?;
        Ok(())
    }
    fn build_parcel_getPowerMonitorReadings(
        &self,
        _arg_powerMonitorIndices: &[i32],
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> binder::Result<binder::binder_impl::Parcel> {
        let mut aidl_data = self.binder.prepare_transact()?;
        aidl_data.write(_arg_powerMonitorIndices)?;
        aidl_data.write(_arg_resultReceiver)?;
        Ok(aidl_data)
    }
    fn read_response_getPowerMonitorReadings(
        &self,
        _arg_powerMonitorIndices: &[i32],
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
        _aidl_reply: std::result::Result<binder::binder_impl::Parcel, binder::StatusCode>,
    ) -> binder::Result<()> {
        if let Err(binder::StatusCode::UNKNOWN_TRANSACTION) = _aidl_reply {
            if let Some(_aidl_default_impl) = <Self as IPowerStatsService>::getDefaultImpl() {
                return _aidl_default_impl
                    .getPowerMonitorReadings(_arg_powerMonitorIndices, _arg_resultReceiver);
            }
        }
        let _aidl_reply = _aidl_reply?;
        Ok(())
    }
}
impl IPowerStatsService for BpPowerStatsService {
    fn getSupportedPowerMonitors(
        &self,
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> binder::Result<()> {
        let _aidl_data = self.build_parcel_getSupportedPowerMonitors(_arg_resultReceiver)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::getSupportedPowerMonitors,
            _aidl_data,
            binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_getSupportedPowerMonitors(_arg_resultReceiver, _aidl_reply)
    }
    fn getPowerMonitorReadings(
        &self,
        _arg_powerMonitorIndices: &[i32],
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> binder::Result<()> {
        let _aidl_data = self
            .build_parcel_getPowerMonitorReadings(_arg_powerMonitorIndices, _arg_resultReceiver)?;
        let _aidl_reply = self.binder.submit_transact(
            transactions::getPowerMonitorReadings,
            _aidl_data,
            binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        self.read_response_getPowerMonitorReadings(
            _arg_powerMonitorIndices,
            _arg_resultReceiver,
            _aidl_reply,
        )
    }
}
impl<P: binder::BinderAsyncPool> IPowerStatsServiceAsync<P> for BpPowerStatsService {
    fn getSupportedPowerMonitors(
        &self,
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> std::future::Ready<binder::Result<()>> {
        let _aidl_data = match self.build_parcel_getSupportedPowerMonitors(_arg_resultReceiver) {
            Ok(_aidl_data) => _aidl_data,
            Err(err) => return std::future::ready(Err(err)),
        };
        let _aidl_reply = self.binder.submit_transact(
            transactions::getSupportedPowerMonitors,
            _aidl_data,
            binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        std::future::ready(
            self.read_response_getSupportedPowerMonitors(_arg_resultReceiver, _aidl_reply),
        )
    }
    fn getPowerMonitorReadings(
        &self,
        _arg_powerMonitorIndices: &[i32],
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> std::future::Ready<binder::Result<()>> {
        let _aidl_data = match self
            .build_parcel_getPowerMonitorReadings(_arg_powerMonitorIndices, _arg_resultReceiver)
        {
            Ok(_aidl_data) => _aidl_data,
            Err(err) => return std::future::ready(Err(err)),
        };
        let _aidl_reply = self.binder.submit_transact(
            transactions::getPowerMonitorReadings,
            _aidl_data,
            binder::binder_impl::FLAG_ONEWAY | binder::binder_impl::FLAG_PRIVATE_LOCAL,
        );
        std::future::ready(self.read_response_getPowerMonitorReadings(
            _arg_powerMonitorIndices,
            _arg_resultReceiver,
            _aidl_reply,
        ))
    }
}
impl IPowerStatsService for binder::binder_impl::Binder<BnPowerStatsService> {
    fn getSupportedPowerMonitors(
        &self,
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> binder::Result<()> {
        self.0.getSupportedPowerMonitors(_arg_resultReceiver)
    }
    fn getPowerMonitorReadings(
        &self,
        _arg_powerMonitorIndices: &[i32],
        _arg_resultReceiver: &crate::mangled::_7_android_2_os_14_ResultReceiver,
    ) -> binder::Result<()> {
        self.0
            .getPowerMonitorReadings(_arg_powerMonitorIndices, _arg_resultReceiver)
    }
}
fn on_transact(
    _aidl_service: &dyn IPowerStatsService,
    _aidl_code: binder::binder_impl::TransactionCode,
    _aidl_data: &binder::binder_impl::BorrowedParcel<'_>,
    _aidl_reply: &mut binder::binder_impl::BorrowedParcel<'_>,
) -> std::result::Result<(), binder::StatusCode> {
    match _aidl_code {
        transactions::getSupportedPowerMonitors => {
            let _arg_resultReceiver: crate::mangled::_7_android_2_os_14_ResultReceiver =
                _aidl_data.read()?;
            let _aidl_return = _aidl_service.getSupportedPowerMonitors(&_arg_resultReceiver);
            Ok(())
        }
        transactions::getPowerMonitorReadings => {
            let _arg_powerMonitorIndices: Vec<i32> = _aidl_data.read()?;
            let _arg_resultReceiver: crate::mangled::_7_android_2_os_14_ResultReceiver =
                _aidl_data.read()?;
            let _aidl_return = _aidl_service
                .getPowerMonitorReadings(&_arg_powerMonitorIndices, &_arg_resultReceiver);
            Ok(())
        }
        _ => Err(binder::StatusCode::UNKNOWN_TRANSACTION),
    }
}
pub(crate) mod mangled {
    pub use super::IPowerStatsService as _7_android_2_os_18_IPowerStatsService;
}
