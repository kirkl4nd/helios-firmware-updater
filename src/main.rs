mod device;

use rusb::{Context, Device, DeviceDescriptor};
use std::error::Error;

use crate::device::detect_helios_dacs;

fn main() -> Result<(), Box<dyn Error>> {
    let helios_dacs = detect_helios_dacs()?;

    println!("Detected {} Helios DAC(s).", helios_dacs.len());
    for (index, device) in helios_dacs.iter().enumerate() {
        let device_desc = device.device_descriptor()?;
        println!("Device {}: Vendor ID = {:04x}, Product ID = {:04x}", index + 1, device_desc.vendor_id(), device_desc.product_id());
    }

    Ok(())
}