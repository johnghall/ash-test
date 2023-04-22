#![allow(unused)]
use anyhow::{Context, Result};
use ash::{self, vk};

fn main() -> Result<()> {
    let entry = unsafe { ash::Entry::load() }?;
    let instance = {
        let application_info = vk::ApplicationInfo::builder().api_version(vk::API_VERSION_1_3);
        let create_info = vk::InstanceCreateInfo::builder().application_info(&application_info);
        unsafe { entry.create_instance(&create_info, None) }?
    };

    let physical_device = unsafe { instance.enumerate_physical_devices() }?
        .into_iter()
        .next()
        .context("No physical device found")?;

    let device = {
        let queue_priorities = [1.0];
        let queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(0)
            .queue_priorities(&queue_priorities);

        let create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(std::slice::from_ref(&queue_create_info));

        unsafe { instance.create_device(physical_device, &create_info, None) }?
    };

    let queue = unsafe { device.get_device_queue(0, 0) };

    unsafe { device.destroy_device(None) }
    unsafe { instance.destroy_instance(None) }
    Ok(())
}
