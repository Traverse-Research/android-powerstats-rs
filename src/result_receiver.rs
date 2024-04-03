use binder::{
    binder_impl::{Deserialize, Serialize},
    FromIBinder, SpIBinder, Strong,
};

#[path = "com/android/internal/os/IResultReceiver.rs"]
#[allow(unused_qualifications, clippy::identity_op)]
mod iresultreceiver;

pub use iresultreceiver::*;

pub(crate) mod mangled {
    // pub(crate) use super::iresultreceiver::mangled::*;

    #[allow(non_camel_case_types)]
    pub(crate) type _7_android_2_os_14_ResultReceiver = super::ResultReceiver;
}

// https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/core/java/android/os/ResultReceiver.aidl;l=20;drc=b741c646c69ebdcfbc3287297a312a4ee1aeb5fe
// ERROR: android-frameworks-base/core/java/android/os/ResultReceiver.aidl:20.37-52: Refusing to generate code with unstructured parcelables. Declared parcelables should be in their own file and/or cannot be used with --structured interfaces.
// Unstructured parcel only has a definition in Java:
// https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/core/java/android/os/ResultReceiver.java
#[derive(Debug)]
pub(crate) struct ResultReceiver {
    // binder: Box<dyn IResultReceiver>,
    binder: Strong<dyn IResultReceiver>,
}

impl ResultReceiver {
    pub fn new<T: IResultReceiver + Send + Sync + 'static>(receiver: T) -> Self {
        let binder = BnResultReceiver::new_binder(
            // TODO: how to get back this inner? Or do we perform a deref trick?
            receiver,
            binder::BinderFeatures {
                set_requesting_sid: true,
                _non_exhaustive: (),
            },
        );
        Self { binder }
    }
}

impl Serialize for ResultReceiver {
    fn serialize(
        &self,
        parcel: &mut binder::binder_impl::BorrowedParcel<'_>,
    ) -> Result<(), binder::StatusCode> {
        // TODO: the service implementation should have called writeTypedObject(), which first writes an integer 0 or 1 to describe nullability:
        // https://cs.android.com/android/platform/superproject/main/+/main:out/soong/.intermediates/frameworks/base/framework-minus-apex-intdefs/android_common/e18b8e8d84cb9f664aa09a397b08c165/xref50/srcjars.xref/android/os/IPowerStatsService.java;l=92;drc=190beaa49a35da1d9dcf66be9cfccfd23b0eb467
        // TODO: The Rust codegen counterpart seems to go through `SerializeOption` _inside_ the struct serializer, though
        parcel.write(&1i32)?;
        // Only after that the contents of ResultReceiver should be passed:
        // https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/core/java/android/os/ResultReceiver.java;l=125;drc=9e8f83db6d969f1e1f47ffa0b0390d867491235b
        self.binder.as_binder().serialize(parcel)
    }
}

impl Deserialize for ResultReceiver {
    fn deserialize(
        parcel: &binder::binder_impl::BorrowedParcel<'_>,
    ) -> Result<Self, binder::StatusCode> {
        let is_set: i32 = parcel.read()?; // Same hack because of serialize()
        assert_eq!(is_set, 1);
        SpIBinder::deserialize(parcel).map(|binder| Self {
            binder: <dyn IResultReceiver as FromIBinder>::try_from(binder)
                .expect("Wrong binder type"),
        })
    }
}
