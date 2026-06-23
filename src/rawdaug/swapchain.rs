use ash::{prelude::VkResult, vk};




pub fn new(physical_device: vk::PhysicalDevice, surface_loader: ash::khr::surface::Instance, surface: vk::SurfaceKHR) -> VkResult<()> {
    //get info to find create swapchain (safe as no alloc)
    let surface_cap = unsafe { surface_loader.get_physical_device_surface_capabilities(physical_device, surface)? };
    let surface_formats = unsafe { surface_loader.get_physical_device_surface_formats(physical_device, surface)? };
    let surface_modes = unsafe { surface_loader.get_physical_device_surface_present_modes(physical_device, surface)?};
    
    Ok(())
}