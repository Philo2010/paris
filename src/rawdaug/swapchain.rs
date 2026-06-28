use ash::{prelude::VkResult, vk::{self, CompositeAlphaFlagsKHR, Extent2D, ImageUsageFlags, PresentModeKHR, SharingMode, StructureType, SwapchainCreateFlagsKHR}};

use crate::rawdaug::error::RDError;




pub unsafe fn new(physical_device: vk::PhysicalDevice, surface_loader: &ash::khr::surface::Instance, surface: vk::SurfaceKHR, swapchain_loader: &ash::khr::swapchain::Device, window: &winit::window::Window) -> Result<(vk::SwapchainKHR, vk::SurfaceFormatKHR, Extent2D), RDError> {
    //get info to find create swapchain (safe as no alloc)
    let surface_cap = unsafe { surface_loader.get_physical_device_surface_capabilities(physical_device, surface)? };
    let surface_formats = unsafe { surface_loader.get_physical_device_surface_formats(physical_device, surface)? };
    let surface_modes = unsafe { surface_loader.get_physical_device_surface_present_modes(physical_device, surface)?};
    
    //first, lets lick the right format
    let picked_format = match surface_formats.iter().find(|x| {
        x.format == vk::Format::R8G8B8A8_SRGB && x.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
    }) {
        Some(a) => *a,
        None => {
            match surface_formats.first() {
                Some(a) => *a,
                None => {return Err(RDError::DeviceHadNoFormats)}, 
            }
        }, 
    };

    let picked_mode: PresentModeKHR = surface_modes.iter().copied().find(|x| {
        *x == PresentModeKHR::MAILBOX
    }).unwrap_or(PresentModeKHR::FIFO);

    let picked_size: Extent2D;

    //check if we can pick the window size
    if !(surface_cap.current_extent.height == u32::MAX) {
        picked_size = surface_cap.current_extent;
    } else {
        picked_size = Extent2D { 
            width: window.inner_size().width.clamp(surface_cap.min_image_extent.width, surface_cap.max_image_extent.width),
            height: window.inner_size().height.clamp(surface_cap.min_image_extent.height, surface_cap.max_image_extent.height)
        };
    }

    //now we must pick the amount of screens we want
    let mut image_count: u32 = std::cmp::max(3, surface_cap.min_image_count);
    if (surface_cap.max_image_count > 0) && (image_count > surface_cap.max_image_count) {
        image_count = surface_cap.max_image_count;
    }

    let create_info: vk::SwapchainCreateInfoKHR = vk::SwapchainCreateInfoKHR {
        s_type: StructureType::SWAPCHAIN_CREATE_INFO_KHR,
        p_next: std::ptr::null(),
        flags: SwapchainCreateFlagsKHR::empty(),
        surface,
        min_image_count: image_count,
        image_format: picked_format.format,
        image_color_space: picked_format.color_space,
        image_extent: picked_size,
        image_array_layers: 1,//we are not a VR app
        image_usage: ImageUsageFlags::COLOR_ATTACHMENT,
        image_sharing_mode: SharingMode::EXCLUSIVE, //we only are using one queue
        queue_family_index_count: 0, //we are in exlcuslive so these done really matter
        p_queue_family_indices: std::ptr::null(),
        pre_transform: surface_cap.current_transform,
        composite_alpha: CompositeAlphaFlagsKHR::OPAQUE, //a game should not be see threw
        present_mode: picked_mode,
        clipped: vk::TRUE,
        old_swapchain: vk::SwapchainKHR::null(),
        _marker: std::marker::PhantomData 
    };
    
    //RDObject should clean this one up
    let swapchain = unsafe { swapchain_loader.create_swapchain(&create_info, None)? };

    Ok((swapchain, picked_format, picked_size))
}