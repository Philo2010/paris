use arrayvec::ArrayVec;
// Main file for createing the instance
//TODO: come up with a better error handling system
use ash::{Entry, prelude::VkResult, vk::{self, StructureType}};
use raw_window_handle::RawDisplayHandle;
use std::ffi::{CStr, c_char};

use crate::rawdaug::error::RDError;



//baked in info for the instance infomation
const APP_NAME: &CStr = c"Paris Runtime";
const ENGINE_NAME: &CStr = c"Paris Engine (rawdaug render)";

//the vaildation layers that are going to be used
const DEBUG_PTRS: [*const i8; 1] = [
    c"VK_LAYER_KHRONOS_validation".as_ptr() as *const i8,
];
const DEBUG_EXT: [*const i8; 1] = [
    c"VK_EXT_debug_utils".as_ptr() as *const i8,
];

fn get_needed_ext(display_handle: &RawDisplayHandle) -> VkResult<ArrayVec<*const i8, 64>> {
    let win_needed = ash_window::enumerate_required_extensions(*display_handle)?;
    
    let mut exts = ArrayVec::<*const i8, 64>::new();
    exts.try_extend_from_slice(win_needed).unwrap();
    exts.try_extend_from_slice(&DEBUG_EXT).unwrap();
    
    Ok(exts)
}

pub unsafe fn new(entry: &ash::Entry, display_handle: &RawDisplayHandle) -> Result<ash::Instance, RDError> {
    let app_info: vk::ApplicationInfo = vk::ApplicationInfo 
        {
            s_type: StructureType::APPLICATION_INFO,
            p_next: std::ptr::null(),
            p_application_name: APP_NAME.as_ptr(),
            application_version: vk::make_api_version(0,0, 1 , 0),
            p_engine_name: ENGINE_NAME.as_ptr(),
            engine_version: vk::make_api_version(0,0, 1 , 0),
            api_version: vk::make_api_version(0, 1, 4, 0),
            _marker: std::marker::PhantomData,
        };
    let needed_ext: ArrayVec<*const i8, 64> = get_needed_ext(display_handle)?;
    
    // This a safe unsafe call, as it does not create any objects to handle, since we are linking, this should be fineS
    let haved_ext = unsafe { entry.enumerate_instance_extension_properties(None)? };
    
    //checks if all exstions needed by winit are present
    let all_present = needed_ext.iter().all(|&needed| {
        let needed_prop = unsafe { CStr::from_ptr(needed) };
        haved_ext.iter().any(|haved| {
            haved.extension_name_as_c_str()
                .map(|h| h == needed_prop)
                .unwrap_or(false)
        })
    });

    if !all_present {
        return Err(RDError::VulkanMissedRequrement("Proper Vulkan extensions (your pc is too old!)"));
    }

    let layer_count: u32;
    let layer_names: *const *const i8;

    if cfg!(debug_assertions) {
        layer_count =  DEBUG_PTRS.len() as u32;
        layer_names =  DEBUG_PTRS.as_ptr();

        let haved_layers = unsafe { entry.enumerate_instance_layer_properties()? };
        let all_pres = DEBUG_PTRS.iter().all( |&x| {
            let needed_layer = unsafe { CStr::from_ptr(x) };
            haved_layers.iter().any(|haved| {
                haved.layer_name_as_c_str()
                    .map(|haved_sexy_form| {haved_sexy_form == needed_layer})
                    .unwrap_or(false)
            })
        });

        if !all_pres {
            return Err(RDError::VulkanMissedRequrement("Missing proper layers (GPU is too old or you forgot to install LunerG SDK) Should you have a debug build?"));
        }
        
        
    } else {
        layer_count = 0;
        layer_names = std::ptr::null();
    }

    let create_info: vk::InstanceCreateInfo = vk::InstanceCreateInfo {
        s_type: StructureType::INSTANCE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: vk::InstanceCreateFlags::empty(),
        p_application_info: &app_info,
        enabled_layer_count: layer_count, //TODO: add the valduion layer
        pp_enabled_layer_names: layer_names,  
        // not safe but you can come and kill me over it once winit gets so bloated it needs 4,294,967,295 layer          
        enabled_extension_count: (needed_ext.len() as u32),
        pp_enabled_extension_names: needed_ext.as_ptr(),
        _marker: std::marker::PhantomData,
    };


    //this is a truely unsafe call, we need to make sure that everything is hunky dorry before closing this one out
    let instance = unsafe { entry.create_instance(&create_info, None)? };
    
    Ok(instance)
}


//we dont implment drop, as drop is handled more by the RD object.