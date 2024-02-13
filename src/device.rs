use rusb::{Device, DeviceHandle, Context, UsbContext};
use std::error::Error;

const VENDOR_ID: u16 = 0x1209;
const PRODUCT_ID: u16 = 0xE500;

pub fn detect_helios_dacs() -> Result<Vec<Device<Context>>, Box<dyn Error>> {
    let mut helios_dacs = Vec::new();
    let context = Context::new()?;

    for device in context.devices()?.iter() {
        let device_description = device.device_descriptor()?;

        if device_description.vendor_id() == VENDOR_ID && device_description.product_id() == PRODUCT_ID {
            helios_dacs.push(device);
        }
    }

    Ok(helios_dacs)
}