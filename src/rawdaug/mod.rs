// The main moduale for the vulkan render for now...
// To denote a vulkan object and not a gentric paris object,
// we shall use the RD prefix
// This object in the future will be called: RDobject as the main grapics 

use ash::{Entry, vk::{self, Extent2D, SurfaceFormatKHR}};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::rawdaug::{error::RDError, logical_device::create_logical_device, physical_device::pick_device};

pub mod instance;
pub mod error;
pub mod vulkan_callback;
pub mod physical_device;
pub mod logical_device;
pub mod swapchain;
pub mod image_view;
pub mod pipeline;


pub struct RDObject {
    physical_device: vk::PhysicalDevice,
    graphics_index: u32,
    swapchain_loader: ash::khr::swapchain::Device,
    image_views: Vec<vk::ImageView>,
    images: Vec<vk::Image>,
    swapchain: vk::SwapchainKHR,
    swapchain_format: SurfaceFormatKHR,
    swapchain_extent: Extent2D,
    logical_device: ash::Device,
    surface: vk::SurfaceKHR,
    surface_loader: ash::khr::surface::Instance,
    instance: ash::Instance,
    entry: ash::Entry,
    graphics_queue: vk::Queue,
}

impl RDObject {
    pub fn new(display_handle: &RawDisplayHandle, window_handle: &RawWindowHandle, window: &winit::window::Window) -> Result<RDObject, RDError>  {
        log::info!("Creating rawdaug object!");
        let entry = Entry::linked();

        // this unsafe is fine as we will destory the object properely at the end with drop
        let instance = unsafe { instance::new(&entry, display_handle)? };
        //create the surface with ash_window (unsafe ok as we destroy)
        let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);
        let surface = unsafe { ash_window::create_surface(&entry, &instance, *display_handle, *window_handle, None)? };
        //now, we find a good physical device
        let physical_device = pick_device(&instance, &surface_loader, surface)?;
        //We will delete this logical device, so this becomes safe
        let (graphics_index, logical_device) = unsafe { create_logical_device(&instance, &physical_device, &surface, &surface_loader)? };
        //we know that the current logical device does indeed suport swapchains, so its safe to load
        let swapchain_loader = ash::khr::swapchain::Device::new(&instance, &logical_device);
        //finally we grab the queue (just grabing a ref and auto deletes so its ok)
        //Its always going to be zero for now because RD is a single queue system
        let graphics_queue = unsafe { logical_device.get_device_queue(graphics_index, 0) }; 
        //create the swapchain
        let (swapchain, swapchain_format, swapchain_extent) = unsafe { swapchain::new(physical_device, &surface_loader, surface, &swapchain_loader, &window)? };
        //these are created by the swapchain and as such will be destored naturaly
        let images = unsafe { swapchain_loader.get_swapchain_images(swapchain) }?;
        //image views (need to be cleaned up)
        let image_views = unsafe { image_view::new(&images, swapchain_format.format, &logical_device)? };
        
        Ok(RDObject { entry, images, image_views, swapchain, swapchain_format, swapchain_extent, instance, surface_loader, swapchain_loader, surface, physical_device, graphics_index, logical_device, graphics_queue })
    }
}


impl Drop for RDObject {
    fn drop(&mut self) {
        //thanks to rust's type system, we know its has lived untill this point
        log::info!("destroying rawdaug object...");
        unsafe { image_view::clean(&self.image_views, &self.logical_device);}
        unsafe { self.swapchain_loader.destroy_swapchain(self.swapchain, None);}
        unsafe { self.logical_device.destroy_device(None);}
        unsafe { self.surface_loader.destroy_surface(self.surface, None);}
        unsafe { self.instance.destroy_instance(None)};
    }
}