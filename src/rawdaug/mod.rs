// The main moduale for the vulkan render for now...
// To denote a vulkan object and not a gentric paris object,
// we shall use the RD prefix
// This object in the future will be called: RDobject as the main grapics 

use ash::{Entry, vk};
use raw_window_handle::RawDisplayHandle;

use crate::rawdaug::error::RDError;

pub mod instance;
pub mod error;


macro_rules! rd_error {
    ($($arg:tt)*) => { log::error!(target: "RD", $($arg)*) }
}
macro_rules! rd_info {
    ($($arg:tt)*) => { log::info!(target: "RD", $($arg)*) }
}
macro_rules! rd_debug {
    ($($arg:tt)*) => { log::debug!(target: "RD", $($arg)*) }
}


pub struct RDObject {
    entry: ash::Entry,
    instance: ash::Instance,
}

impl RDObject {
    pub fn new(display_handle: &RawDisplayHandle) -> Result<RDObject, RDError>  {
        rd_info!("Creating rawdaug object!");
        let entry = Entry::linked();

        // this unsafe is fine as we will destory the object properely at the end with drop
        let instance = unsafe { instance::new(&entry, display_handle)? };
        
        Ok(RDObject { entry, instance })
    }
}


impl Drop for RDObject {
    fn drop(&mut self) {
        //thanks to rust's type system, we know its has lived untill this point
        rd_info!("destroying rawdaug object...");
        unsafe { self.instance.destroy_instance(None) };
    }
}