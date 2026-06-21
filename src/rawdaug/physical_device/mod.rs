use ash::{prelude::VkResult, vk};

use crate::rawdaug::{error::RDError, physical_device::pick::pick_from_list};

// all of the code to pick and load a physical device
mod pick;


pub fn pick_device(instance: ash::Instance) -> Result<vk::PhysicalDevice, RDError> {
    //safe as it only enumatres, we dont own shit   
    let devices = unsafe { instance.enumerate_physical_devices()? };
    pick_from_list(devices, &instance)
}