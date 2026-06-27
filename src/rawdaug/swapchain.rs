use ash::{prelude::VkResult, vk::{self, PresentModeKHR}};

use crate::rawdaug::error::RDError;




pub fn new(physical_device: vk::PhysicalDevice, surface_loader: ash::khr::surface::Instance, surface: vk::SurfaceKHR) -> Result<(), RDError> {
    //get info to find create swapchain (safe as no alloc)
    let surface_cap = unsafe { surface_loader.get_physical_device_surface_capabilities(physical_device, surface)? };
    let surface_formats = unsafe { surface_loader.get_physical_device_surface_formats(physical_device, surface)? };
    let surface_modes = unsafe { surface_loader.get_physical_device_surface_present_modes(physical_device, surface)?};
    
    //first, lets lick the right format
    let picked_format = match surface_formats.iter().find(|x| {
        x.format == vk::Format::R8G8B8A8_SRGB && x.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
    }) {
        Some(a) => a,
        None => {
            match surface_formats.first() {
                Some(a) => a,
                None => {return Err(RDError::DeviceHadNoFormats)}, 
            }
        }, 
    };

    let picked_mode: &PresentModeKHR = surface_modes.iter().find(|x| {
        **x == PresentModeKHR::MAILBOX
    }).unwrap_or(&PresentModeKHR::FIFO);

    

    Ok(())
}