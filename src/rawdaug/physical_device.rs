use std::{collections::BTreeMap, ffi::CStr};

use ash::{prelude::VkResult, vk::{self, PhysicalDeviceType}};

use crate::rawdaug::{error::{self, RDError}};

// all of the code to pick and load a physical device
pub const neededD_EXT: [&CStr; 1] = [
    c"VK_KHR_swapchain"
];

pub fn pick_from_list(list: Vec<vk::PhysicalDevice>, instance: &ash::Instance, surface_loader: &ash::khr::surface::Instance, surface: vk::SurfaceKHR) -> Result<vk::PhysicalDevice, error::RDError> {   
    //BTreeMap colletions are never an issue because my playerbase it too broke to have two good quatly gpus
    let mut map: BTreeMap<u32, vk::PhysicalDevice> = BTreeMap::new();


    for gpu in list {
        // These unsafe calls are safe as we only pass in vaild PhysicalDevices and we have a valid instance thanks to RDobject
        let d_prop = unsafe { instance.get_physical_device_properties(gpu) };
        // not needed utill the render has more demands
        let d_ext = unsafe {instance.enumerate_device_extension_properties(gpu)?};
        let mut vk11_features = vk::PhysicalDeviceVulkan11Features::default();
        let mut vk13_features = vk::PhysicalDeviceVulkan13Features::default();
        let mut ext_features = vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT::default();

        let mut features2 = vk::PhysicalDeviceFeatures2::default()
            .push_next(&mut vk11_features)
            .push_next(&mut vk13_features)
            .push_next(&mut ext_features);

        let d_feat = unsafe { instance.get_physical_device_features2(gpu, &mut features2) };
        let mut score: u32 = 0;
        if !(d_prop.api_version >= vk::make_api_version(0, 1, 4, 0)) {
            continue;
        }
        //safe as we know insatance and gpu are vaild (unless the gpu driver is broken in that cause we have bigger problems)
        let haved_queued_famlies = unsafe { instance.get_physical_device_queue_family_properties(gpu) };
        

        let has_graphics = haved_queued_famlies.iter().enumerate().any(|(index, x)| {
            let graphics = x.queue_flags.contains(vk::QueueFlags::GRAPHICS);
            //safe as no acloc
            let surface_support = unsafe { surface_loader.get_physical_device_surface_support(gpu, index as u32, surface).unwrap_or(false) };
            graphics && surface_support
        });
        let supports_required_features =
            vk11_features.shader_draw_parameters == vk::TRUE &&
            vk13_features.dynamic_rendering == vk::TRUE &&
            ext_features.extended_dynamic_state == vk::TRUE;

        let has_needed_ext = neededD_EXT.iter().all(|needed| {
            d_ext.iter().any(|ext| {
                ext.extension_name_as_c_str().map_or(false, |name| *needed == name) //dont like how it fails silently but eh
            })
        });
        

        if !(has_graphics && supports_required_features && has_needed_ext) {
            continue;
        }

        // as a game engine, we massively want to use a discetic gpu as they are genrally the most preformate
        if d_prop.device_type == PhysicalDeviceType::DISCRETE_GPU {
            score += 1000;
        }

        score += d_prop.limits.max_image_dimension2_d;
        //right now, rawdaug does not need geometry shaders
        map.insert(score, gpu);
    }

    match map.into_iter().next_back() {
        Some(a) => {
            Ok(a.1)
        },
        None => {
            Err(RDError::NoVaildDeviceFound)
        },
    }
}


pub fn pick_device(instance: &ash::Instance,  surface_loader: &ash::khr::surface::Instance, surface: vk::SurfaceKHR) -> Result<vk::PhysicalDevice, RDError> {
    //safe as it only enumatres, we dont own shit   
    let devices = unsafe { instance.enumerate_physical_devices()? };
    pick_from_list(devices, instance, surface_loader, surface)
}