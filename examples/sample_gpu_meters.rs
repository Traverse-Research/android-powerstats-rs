use android_powerstats::*;
use anyhow::Result;

pub fn main() -> Result<()> {
    // Ensure that we can handle all transactions on the main thread.
    binder::ProcessState::set_thread_pool_max_thread_count(0);
    binder::ProcessState::start_thread_pool();

    for s in [
        BackendSelection::VendorHardwareService,
        BackendSelection::SystemJavaService,
    ] {
        eprintln!("Trying {s:?}…");
        let stats = match PowerStats::new_with_backend(s) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("{s:?} not supported: {e:?}");
                continue;
            }
        };

        let gpu_meters = stats.energy_meters()?;
        // .into_iter()
        // .filter(|m| m.subsystem == "GPU")
        // .collect::<Vec<_>>();
        let gpu_consumers = stats.energy_consumers()?;
        // .into_iter()
        // .filter(|c| c.r#type == EnergyConsumerType::Other && c.name == "GPU")
        // .collect::<Vec<_>>();
        println!("{s:?} meter(s): {:?}", gpu_meters);
        println!("{s:?} consumer(s): {:?}", gpu_consumers);

        let meter_ids = gpu_meters.iter().map(|m| m.id).collect::<Vec<_>>();
        let meter_readings = stats.read_energy_meters(&meter_ids)?;
        println!("{s:?} meter reading(s): {:?}", meter_readings);

        let consumer_ids = gpu_consumers.iter().map(|c| c.id).collect::<Vec<_>>();
        let consumer_readings = stats.read_energy_consumers(&consumer_ids)?;
        println!("{s:?} consumer reading(s): {:?}", consumer_readings);
    }

    Ok(())
}
