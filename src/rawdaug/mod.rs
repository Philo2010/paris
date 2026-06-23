// The main moduale for the vulkan render for now...
// To denote a vulkan object and not a gentric paris object,
// we shall use the RD prefix
// This object in the future will be called: RDobject as the main grapics 

use ash::{Entry, vk};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::rawdaug::{error::RDError, logical_device::create_logical_device, physical_device::pick_device};

pub mod instance;
pub mod error;
pub mod vulkan_callback;
pub mod physical_device;
pub mod logical_device;
pub mod swapchain;


pub struct RDObject {
    physical_device: vk::PhysicalDevice,
    graphics_index: u32,
    logical_device: ash::Device,
    surface: vk::SurfaceKHR,
    surface_loader: ash::khr::surface::Instance,
    instance: ash::Instance,
    entry: ash::Entry,
    graphics_queue: vk::Queue,
}

impl RDObject {
    pub fn new(display_handle: &RawDisplayHandle, window_handle: &RawWindowHandle) -> Result<RDObject, RDError>  {
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
        //finally we grab the queue (just grabing a ref and auto deletes so its ok)
        //Its always going to be zero for now because RD is a single queue system
        let graphics_queue = unsafe { logical_device.get_device_queue(graphics_index, 0) }; 
        
        Ok(RDObject { entry, instance, surface_loader, surface, physical_device, graphics_index, logical_device, graphics_queue })
    }
}


impl Drop for RDObject {
    fn drop(&mut self) {
        //thanks to rust's type system, we know its has lived untill this point
        log::info!("destroying rawdaug object...");
        unsafe { self.logical_device.destroy_device(None);}
        unsafe { self.surface_loader.destroy_surface(self.surface, None);}
        unsafe { self.instance.destroy_instance(None)};
    }
}