#![warn(unused_qualifications)]

use std::{fmt, str::FromStr, time::Duration};

use android_hardware_power_stats::{
    BpPowerStats, Channel, EnergyConsumerResult, EnergyMeasurement, IPowerStats,
};
use android_os_powerstatsservice::{IPowerStatsService, PowerMonitorType};
use anyhow::Result;
use binder::Strong;
use log::warn;

mod android_hardware_power_stats;
mod android_os_powerstatsservice;
mod bundle;
mod result_receiver;

pub(crate) mod mangled {
    pub(crate) use super::android_hardware_power_stats::mangled::*;
    pub(crate) use super::bundle::mangled::*;
    pub(crate) use super::result_receiver::mangled::*;
}

enum Backend {
    VendorHardwareService(Strong<dyn IPowerStats>),
    SystemJavaService(Strong<dyn IPowerStatsService>),
}

impl fmt::Debug for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut b = match self {
            Backend::VendorHardwareService(s) => s.as_binder(),
            Backend::SystemJavaService(s) => s.as_binder(),
        };
        f.debug_struct("Backend")
            .field("descriptor", &b.get_class().unwrap().get_descriptor())
            .finish_non_exhaustive()
        // f.write_str(match self {
        //     Backend::VendorHardwareService(_) => "android.hardware.power.stats.IPowerStats/default",
        //     Backend::SystemJavaService(_) => "powerstats",
        // })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BackendSelection {
    VendorHardwareService,
    SystemJavaService,
}

#[derive(Debug)]
pub struct PowerStats {
    backend: Backend,
}

impl PowerStats {
    /// Tries to talk to `android.hardware.power.stats.IPowerStats/default`, otherwise falls back to `powerstats`
    pub fn new() -> Result<Self> {
        match Self::new_with_backend(BackendSelection::SystemJavaService) {
            Ok(s) => Ok(s),
            Err(e) => {
                warn!("Failed to get `powerstats` service because of `{e:?}`. Falling back to vendor HAL");
                Self::new_with_backend(BackendSelection::VendorHardwareService).inspect_err(|e| {
                    warn!("Failed to get `android.hardware.power.stats.IPowerStats/default` because of `{e:?}`");
                    // type=1400 audit(0.0:419): avc:  denied  { call } for  scontext=u:r:untrusted_app_32:s0:c13,c257,c512,c768 tcontext=u:r:hal_power_stats_default:s0 tclass=binder permissive=1 app=...
                    warn!("If you see `denied {{ call }} for scontext=..untrusted_app.. tcontext=..hal_power_stats_default..` in `logcat`, issue `setenforce 0` from a root shell to allow access");
                })
            }
        }
    }

    pub fn new_with_backend(selection: BackendSelection) -> Result<Self> {
        match selection {
            BackendSelection::VendorHardwareService => {
                let descriptor = <BpPowerStats as IPowerStats>::get_descriptor();
                let i =
                    binder::get_interface::<dyn IPowerStats>(&format!("{}/default", descriptor))?;
                Ok(Self {
                    backend: Backend::VendorHardwareService(i),
                })
            }
            BackendSelection::SystemJavaService => {
                let i = binder::get_interface::<dyn IPowerStatsService>("powerstats")?;
                Ok(Self {
                    backend: Backend::SystemJavaService(i),
                })
            }
        }
    }

    /// Only returns meters, i.e. individual regulators.  To be used with [`Self::read_energy_meters()`].
    pub fn energy_meters(&self) -> Result<Vec<EnergyMeter>> {
        match &self.backend {
            Backend::VendorHardwareService(s) => {
                // let meters = s.getEnergyConsumerInfo()?;
                let meters = s.getEnergyMeterInfo()?;
                Ok(meters
                    .into_iter()
                    .map(
                        |Channel {
                             id,
                             name,
                             subsystem,
                         }| EnergyMeter {
                            id,
                            name,
                            subsystem,
                        },
                    )
                    .collect())
            }
            Backend::SystemJavaService(s) => {
                let monitors = s.receive_supported_power_monitors()?;

                Ok(monitors
                    .into_iter()
                    // Only return measurements directly from power rails, corresponding to the HAL's "energy meter" concept
                    .filter(|pm| pm.r#type == PowerMonitorType::Measurement)
                    .map(|pm| {
                        let (name, subsystem) = pm.name.split_once(':').unwrap();
                        let name = name.strip_prefix('[').unwrap();
                        let name = name.strip_suffix(']').unwrap();
                        EnergyMeter {
                            id: pm.index,
                            name: name.to_string(),
                            subsystem: subsystem.to_string(),
                        }
                    })
                    .collect())
            }
        }
    }

    // /// Only returns consumers (i.e. aggregated data from individual meters/rails/regulators for
    // /// consumers like CPU clusters, GPU, etc), not individual meters on regulators.
    pub fn energy_consumers(&self) -> Result<Vec<EnergyConsumer>> {
        match &self.backend {
            Backend::VendorHardwareService(s) => {
                // let meters = s.getEnergyConsumerInfo()?;
                let meters = s.getEnergyConsumerInfo()?;
                Ok(meters
                    .into_iter()
                    .map(
                        |android_hardware_power_stats::EnergyConsumer {
                             id,
                             ordinal,
                             r#type,
                             name,
                         }| EnergyConsumer {
                            id,
                            name,
                            ordinal,
                            r#type: r#type.into(),
                        },
                    )
                    .collect())
            }
            Backend::SystemJavaService(s) => {
                let monitors = s.receive_supported_power_monitors()?;

                Ok(monitors
                    .into_iter()
                    // Only return consumers
                    .filter(|pm| pm.r#type == PowerMonitorType::Consumer)
                    .map(
                        |android_os_powerstatsservice::PowerMonitor {
                             index,
                             r#type: _,
                             name,
                         }| {
                            let (type_name, ordinal) = match name.split_once('/') {
                                Some((type_name, ordinal)) => {
                                    (type_name.to_string(), ordinal.parse().unwrap())
                                }
                                None => (name, 0),
                            };
                            // i.e. GPU is Other
                            let r#type = type_name.parse().unwrap_or(EnergyConsumerType::Other);

                            EnergyConsumer {
                                id: index,
                                name: type_name,
                                ordinal,
                                r#type,
                            }
                        },
                    )
                    .collect())
            }
        }
    }

    // Same code
    fn read_power_monitors(
        s: &Strong<dyn IPowerStatsService>,
        ids: &[i32],
    ) -> Result<Vec<EnergyMeterReading>> {
        let (receiver, chan) = android_os_powerstatsservice::ReceivePowerMonitorReadings::new();
        let receiver = result_receiver::ResultReceiver::new(receiver);
        // TODO: The caller might wish to reuse the receiver?

        s.getPowerMonitorReadings(ids, &receiver)?;
        let readings = chan.recv().unwrap();

        let result = readings
            .timestamps_ms
            .into_iter()
            .zip(readings.energy_uws)
            .map(|(t, e)| EnergyMeterReading {
                timestamp: Duration::from_millis(t.try_into().unwrap()),
                // TODO: Help, for meters the system service "conveniently" ignores the durationMs field?
                // Makes it so that we can not even calculate proper deltas since the last call, since noise
                // will be inserted based on the previous and current value.
                // https://cs.android.com/android/platform/superproject/main/+/main:frameworks/base/services/core/java/com/android/server/powerstats/PowerStatsService.java;l=767-779;drc=515faa7cf81b33607b7926600265be5c58ef300d
                duration: None,
                energy_uws: e,
            })
            .collect();
        Ok(result)
    }

    /// Returns a list of meter readings in the same order as the ids specified in `meter_ids`
    pub fn read_energy_meters(&self, meter_ids: &[i32]) -> Result<Vec<EnergyMeterReading>> {
        match &self.backend {
            Backend::VendorHardwareService(s) => {
                let readings = s.readEnergyMeter(meter_ids)?;
                let result = readings.into_iter().map(|m| m.into()).collect();
                Ok(result)
            }
            Backend::SystemJavaService(s) => Self::read_power_monitors(s, meter_ids),
        }
    }

    /// Returns a list of consumer readings in the same order as the ids specified in `consumer_ids`
    pub fn read_energy_consumers(
        &self,
        consumer_ids: &[i32],
    ) -> Result<Vec<EnergyConsumerReading>> {
        match &self.backend {
            Backend::VendorHardwareService(s) => {
                let readings = s.getEnergyConsumed(consumer_ids)?;
                let result = readings.into_iter().map(|e| e.into()).collect();
                Ok(result)
            }
            Backend::SystemJavaService(s) => {
                let monitors = Self::read_power_monitors(s, consumer_ids)?;
                // As soon as the code was generalized, need arised for a separate type. Since the
                // Java service doesn't provide most of the info anyway, just drop it
                Ok(monitors.into_iter().map(|m| m.into()).collect())
            }
        }
    }
}

#[doc(alias = "android.os.PowerMonitor")]
#[doc(alias = "android.hardware.power.stats.Channel")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnergyMeter {
    pub id: i32,
    pub name: String,
    /// Extracted from the name on [`Backend::SystemJavaService`], where is typically appended with a colon (`:`).
    pub subsystem: String,
}

/// <https://cs.android.com/android/platform/superproject/main/+/main:hardware/interfaces/power/stats/aidl/android/hardware/power/stats/EnergyConsumerType.aidl>
#[doc(alias = "android.hardware.power.stats.EnergyConsumerType")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnergyConsumerType {
    Other,
    Bluetooth,
    CpuCluster,
    Display,
    Gnss,
    MobileRadio,
    Wifi,
    Camera,
}

impl From<android_hardware_power_stats::EnergyConsumerType> for EnergyConsumerType {
    fn from(value: android_hardware_power_stats::EnergyConsumerType) -> Self {
        use android_hardware_power_stats::EnergyConsumerType as O;
        match value {
            O::OTHER => Self::Other,
            O::BLUETOOTH => Self::Bluetooth,
            O::CPU_CLUSTER => Self::CpuCluster,
            O::DISPLAY => Self::Display,
            O::GNSS => Self::Gnss,
            O::MOBILE_RADIO => Self::MobileRadio,
            O::WIFI => Self::Wifi,
            O::CAMERA => Self::Camera,
            x => todo!("EnergyConsumerType {x:#?}"),
        }
    }
}

impl FromStr for EnergyConsumerType {
    type Err = ();

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        Ok(match s {
            "OTHER" => Self::Other,
            "BLUETOOTH" => Self::Bluetooth,
            // "CPU_CLUSTER" => Self::CpuCluster,
            "CPU" => Self::CpuCluster,
            "DISPLAY" => Self::Display,
            "GNSS" => Self::Gnss,
            "MOBILE_RADIO" => Self::MobileRadio,
            "WIFI" => Self::Wifi,
            "CAMERA" => Self::Camera,
            _ => return Err(()),
        })
    }
}

/// <https://cs.android.com/android/platform/superproject/main/+/main:hardware/interfaces/power/stats/aidl/android/hardware/power/stats/EnergyConsumer.aidl>
#[doc(alias = "android.os.PowerMonitor")]
#[doc(alias = "android.hardware.power.stats.EnergyConsumer")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnergyConsumer {
    pub id: i32,
    pub name: String,
    pub ordinal: i32,
    pub r#type: EnergyConsumerType,
}

#[doc(alias = "android.os.PowerMonitorReadings")]
#[doc(alias = "android.hardware.power.stats.EnergyMeasurement")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EnergyMeterReading {
    /// Monotonic timestamp since boot
    pub timestamp: Duration,
    /// Period of time over which [`Self::energy_uws`] has accumulated. Not provided on [`Backend::SystemJavaService`], nor for energy consumers
    pub duration: Option<Duration>,
    /// Accumulated energy in `uWs` (uJ) during [`Self::duration`]
    pub energy_uws: i64,
}

impl From<EnergyMeasurement> for EnergyMeterReading {
    fn from(value: EnergyMeasurement) -> Self {
        let EnergyMeasurement {
            id: _,
            timestampMs,
            durationMs,
            energyUWs,
        } = value;
        EnergyMeterReading {
            timestamp: Duration::from_millis(timestampMs.try_into().unwrap()),
            duration: Some(Duration::from_millis(durationMs.try_into().unwrap())),
            energy_uws: energyUWs,
        }
    }
}

#[doc(alias = "android.os.PowerMonitorReadings")]
#[doc(alias = "android.hardware.power.stats.EnergyConsumerResult")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnergyConsumerReading {
    /// Monotonic timestamp since boot
    pub timestamp: Duration,
    /// Accumulated energy in `uWs` (uJ)
    pub energy_uws: i64,

    pub attribution: Vec<EnergyConsumerAttribution>,
}

impl From<EnergyConsumerResult> for EnergyConsumerReading {
    fn from(value: EnergyConsumerResult) -> Self {
        let EnergyConsumerResult {
            id: _,
            timestampMs,
            energyUWs,
            attribution,
        } = value;
        Self {
            timestamp: Duration::from_millis(timestampMs.try_into().unwrap()),
            energy_uws: energyUWs,
            attribution: attribution.into_iter().map(|e| e.into()).collect(),
        }
    }
}

impl From<EnergyMeterReading> for EnergyConsumerReading {
    fn from(value: EnergyMeterReading) -> Self {
        {
            let EnergyMeterReading {
                timestamp,
                duration: None,
                energy_uws,
            } = value
            else {
                unreachable!()
            };

            EnergyConsumerReading {
                timestamp,
                energy_uws,
                // Unavailable
                attribution: vec![],
            }
        }
    }
}

/// How much power a certain UID (app) consumed
#[doc(alias = "android.hardware.power.stats.EnergyConsumerAttribution")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EnergyConsumerAttribution {
    pub uid: i32,
    /// Accumulated energy in `uWs` (uJ)
    pub energy_uws: i64,
}

impl From<android_hardware_power_stats::EnergyConsumerAttribution> for EnergyConsumerAttribution {
    fn from(value: android_hardware_power_stats::EnergyConsumerAttribution) -> Self {
        let android_hardware_power_stats::EnergyConsumerAttribution { uid, energyUWs } = value;
        Self {
            uid,
            energy_uws: energyUWs,
        }
    }
}

#[test]
pub fn sample_gpu_meters() {
    pub fn sample_gpu_meters() -> Result<()> {
        for s in [
            BackendSelection::VendorHardwareService,
            BackendSelection::SystemJavaService,
        ] {
            let stats = match PowerStats::new_with_backend(s) {
                Ok(b) => b,
                Err(_) => todo!(),
            };

            let gpu_meters = stats
                .energy_meters()?
                .into_iter()
                .filter(|m| m.subsystem == "GPU")
                .collect::<Vec<_>>();
            let gpu_consumers = stats
                .energy_consumers()?
                .into_iter()
                .filter(|c| c.r#type == EnergyConsumerType::Other && c.name == "GPU")
                .collect::<Vec<_>>();
            println!("{s:?} GPU meter(s): {:?}", gpu_meters);
            println!("{s:?} GPU consumer(s): {:?}", gpu_consumers);

            let meter_ids = gpu_meters.iter().map(|m| m.id).collect::<Vec<_>>();
            let meter_readings = stats.read_energy_meters(&meter_ids)?;
            println!("{s:?} GPU meter reading(s): {:?}", meter_readings);

            let consumer_ids = gpu_consumers.iter().map(|c| c.id).collect::<Vec<_>>();
            let consumer_readings = stats.read_energy_consumers(&consumer_ids)?;
            println!("{s:?} GPU consumer reading(s): {:?}", consumer_readings);
            if let Some(gpu0) = consumer_readings.first() {
                if !gpu0.attribution.is_empty() {
                    println!("TODO: Have attribution info, read UID for current process!")
                }
            }
        }

        Ok(())
    }

    sample_gpu_meters().unwrap();
}
