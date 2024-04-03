use std::sync::{
    mpsc::{channel, Receiver, Sender},
    OnceLock,
};

use binder::{
    binder_impl::{BorrowedParcel, Deserialize},
    Parcelable, StatusCode,
};

use crate::{
    bundle::{
        parcel_read_string8, register_creator, Bundle, Object, ParcelableCreator,
        ParcelableInstance,
    },
    result_receiver::{IResultReceiver, ResultReceiver},
};

#[path = "android/os/IPowerStatsService.rs"]
#[allow(dead_code, clippy::identity_op, unused_imports, unused_qualifications)]
pub mod powerstatsservice;

// pub(crate) mod mangled {
//     pub(crate) use super::powerstatsservice::mangled::*;
// }

pub use powerstatsservice::IPowerStatsService;

/// Java-only parcelable
/// <https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/core/java/android/os/PowerMonitor.java;l=40;drc=82bdcd7ff7ba4962274f1d88caac0594ae964bef>
#[derive(Clone, Debug, Default)]
pub(crate) struct PowerMonitor {
    pub(crate) index: i32,
    pub(crate) r#type: PowerMonitorType,
    pub(crate) name: String,
}

impl Parcelable for PowerMonitor {
    fn write_to_parcel(&self, _parcel: &mut BorrowedParcel<'_>) -> Result<(), StatusCode> {
        todo!()
    }

    fn read_from_parcel(&mut self, _parcel: &BorrowedParcel<'_>) -> Result<(), StatusCode> {
        todo!()
    }
}

impl Deserialize for PowerMonitor {
    fn deserialize(parcel: &BorrowedParcel<'_>) -> Result<Self, StatusCode> {
        Ok(Self {
            index: parcel.read()?,
            r#type: match parcel.read::<i32>()? {
                x if x == PowerMonitorType::Consumer as i32 => PowerMonitorType::Consumer,
                x if x == PowerMonitorType::Measurement as i32 => PowerMonitorType::Measurement,
                x => todo!("Unknown PowerMonitorType {x:?}"),
            },
            name: parcel_read_string8(parcel)?,
        })
    }
}
// impl_deserialize_for_parcelable!(PowerMonitor);

/// <https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/core/java/android/os/PowerMonitor.java;l=42-67;drc=d68742df4e3c723ea5296c743606362cd04180bb>
#[repr(i32)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub(crate) enum PowerMonitorType {
    /**
     * Power monitor corresponding to a subsystem. The energy value may be a direct pass-through
     * power rail measurement, or modeled in some fashion.  For example, an energy consumer may
     * represent a combination of multiple rails or a portion of a rail shared between subsystems,
     * e.g. WiFi and Bluetooth are often handled by the same chip, powered by a shared rail.
     * Some consumer names are standardized, others are not.
     */
    #[default]
    Consumer = 0,

    /**
     * Power monitor corresponding to a directly measured power rail. Rails are device-specific:
     * no assumptions can be made about the source of those measurements across different devices,
     * even if they have the same name.
     */
    Measurement = 1,
}

struct PowerMonitorCreator;
impl ParcelableCreator for PowerMonitorCreator {
    fn create_from_parcel(
        &self,
        parcel: &BorrowedParcel<'_>,
    ) -> Result<Box<dyn ParcelableInstance>, StatusCode> {
        parcel.read::<PowerMonitor>().map(|pm| Box::new(pm) as _)
    }
}

pub(crate) struct ReceiveSupportedPowerMonitors(Sender<Vec<PowerMonitor>>);
impl ReceiveSupportedPowerMonitors {
    pub(crate) fn new() -> (Self, Receiver<Vec<PowerMonitor>>) {
        static CREATOR: OnceLock<()> = OnceLock::new();
        CREATOR.get_or_init(|| register_creator("android.os.PowerMonitor", &PowerMonitorCreator));
        let (s, r) = channel();
        (Self(s), r)
    }
}
impl binder::Interface for ReceiveSupportedPowerMonitors {}
impl IResultReceiver for ReceiveSupportedPowerMonitors {
    fn r#send(&self, code: i32, data: &Bundle) -> binder::Result<()> {
        assert_eq!(code, 0);
        let Object::ParcelableArray(monitors) = &data.0[powerstatsservice::KEY_MONITORS] else {
            panic!("Must have ParcelableArray")
        };

        let result = monitors
            .iter()
            .map(|monitor| {
                let monitor: &PowerMonitor = monitor.as_any().downcast_ref().unwrap();
                monitor.clone()
            })
            .collect::<Vec<_>>();

        self.0.send(result).unwrap();

        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct PowerMonitorReadings {
    // pub(crate) timestamp: Duration,
    // pub(crate) energy_uws: i64,
    pub(crate) timestamps_ms: Vec<i64>,
    pub(crate) energy_uws: Vec<i64>,
}

pub(crate) struct ReceivePowerMonitorReadings(Sender<PowerMonitorReadings>);
impl ReceivePowerMonitorReadings {
    pub(crate) fn new() -> (Self, Receiver<PowerMonitorReadings>) {
        let (s, r) = channel();
        (Self(s), r)
    }
}
impl binder::Interface for ReceivePowerMonitorReadings {}
impl IResultReceiver for ReceivePowerMonitorReadings {
    fn r#send(&self, code: i32, data: &Bundle) -> binder::Result<()> {
        assert_eq!(code, 0);
        let Object::LongArray(timestamps) = &data.0[powerstatsservice::KEY_TIMESTAMPS] else {
            panic!("Must have LongArray")
        };
        let Object::LongArray(energy) = &data.0[powerstatsservice::KEY_ENERGY] else {
            panic!("Must have LongArray")
        };

        self.0
            .send(PowerMonitorReadings {
                timestamps_ms: timestamps.clone(),
                energy_uws: energy.clone(),
            })
            .unwrap();

        Ok(())
    }
}

impl dyn IPowerStatsService {
    // Only allowed when having a trait object
    pub fn receive_supported_power_monitors(&self) -> binder::Result<Vec<PowerMonitor>> {
        let (receiver, chan) = ReceiveSupportedPowerMonitors::new();
        let receiver = ResultReceiver::new(receiver);
        // TODO: Since we pass a borrow, can we get access to the contents again?
        self.getSupportedPowerMonitors(&receiver)?;
        let monitors = chan.recv().unwrap();
        Ok(monitors)
    }
}
