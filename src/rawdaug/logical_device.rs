use std::ffi::c_void;

use ash::vk::{self, DeviceCreateFlags, DeviceCreateInfo, DeviceQueueCreateFlags, DeviceQueueCreateInfo, PhysicalDeviceFeatures, PhysicalDeviceFeatures2, QueueFlags, StructureType};

use crate::rawdaug::{error::RDError, physical_device};


macro_rules! cstr_ptrs {
    ($slice:expr) => {{
        const N: usize = $slice.len();
        let mut arr = [std::ptr::null::<i8>(); N];
        let mut i = 0;
        while i < N {
            arr[i] = $slice[i].as_ptr();
            i += 1;
        }
        arr
    }};
}

pub unsafe fn create_logical_device(instance: &ash::Instance, p_device: &vk::PhysicalDevice) -> Result<(u32, ash::Device), RDError> {
    let graphics_queue_index = instance
        .get_physical_device_queue_family_properties(*p_device) //UNSAFE CALL IS FINE BECAUSE NO MEMEORY LEAKS!
        .iter()
        .position(|q| q.queue_flags.contains(QueueFlags::GRAPHICS))
        .ok_or(RDError::DeviceHadNoGraphicsQueues)? as u32;

    let queue_priority = 1.0f32;
    let create_info_queue: DeviceQueueCreateInfo = DeviceQueueCreateInfo {
        s_type: StructureType::DEVICE_QUEUE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: DeviceQueueCreateFlags::empty(),
        queue_family_index: graphics_queue_index,
        queue_count: 1,
        p_queue_priorities: &queue_priority,
        _marker: std::marker::PhantomData,
    };

    //WE will be using more features later, however, for NOW, na
    let device_features = PhysicalDeviceFeatures::default(); // all false
    let mut extended_dynamic_state = vk::PhysicalDeviceExtendedDynamicStateFeaturesEXT::default()
        .extended_dynamic_state(true);

    let mut vulkan13_features = vk::PhysicalDeviceVulkan13Features::default()
        .dynamic_rendering(true);

    let mut vulkan11_features = vk::PhysicalDeviceVulkan11Features::default()
        .shader_draw_parameters(true);

    let mut features2 = vk::PhysicalDeviceFeatures2::default();

    // push_next prepends each struct into the chain
    features2 = features2
        .features(device_features)
        .push_next(&mut vulkan11_features)
        .push_next(&mut vulkan13_features)
        .push_next(&mut extended_dynamic_state);

    let extension_ptrs: [*const i8; 1] = cstr_ptrs!(physical_device::neededD_EXT);

    let create_info: DeviceCreateInfo = DeviceCreateInfo { 
        s_type: StructureType::DEVICE_CREATE_INFO,
        p_next: &mut features2 as *mut vk::PhysicalDeviceFeatures2 as *mut c_void,
        flags: DeviceCreateFlags::empty(),
        queue_create_info_count: 1,
        p_queue_create_infos: &create_info_queue,
        enabled_extension_count: physical_device::neededD_EXT.len() as u32, 
        pp_enabled_extension_names: extension_ptrs.as_ptr(),
        p_enabled_features: std::ptr::null(),
        _marker: std::marker::PhantomData,
        #[allow(deprecated)] //This is no longer used by vulkan, so i just set it to its default values
        enabled_layer_count: 0,
        #[allow(deprecated)]
        pp_enabled_layer_names: core::ptr::null(),
    };


    //THIS IS TRUELY UNSAFE, it creates an object that HAS to be cleared by RDobject.
    let device = instance.create_device(*p_device, &create_info, None);


    match device {
        Ok(a) => {
            Ok((graphics_queue_index, a))
        },
        Err(e) => {
            Err(RDError::VulkanError(e))
        },
    }
}


//this *should* be in a another file, but its so freaking small i dont care to make another file this
