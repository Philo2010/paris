use ash::vk::{self, DeviceCreateFlags, DeviceCreateInfo, DeviceQueueCreateFlags, DeviceQueueCreateInfo, PhysicalDeviceFeatures, QueueFlags, StructureType};

use crate::rawdaug::error::RDError;


pub unsafe fn create_logical_device(instance: &ash::Instance, p_device: &vk::PhysicalDevice) -> Result<ash::Device, RDError> {
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

    let create_info: DeviceCreateInfo = DeviceCreateInfo { 
        s_type: StructureType::DEVICE_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: DeviceCreateFlags::empty(),
        queue_create_info_count: todo!(),
        p_queue_create_infos: todo!(),
        enabled_extension_count: todo!(),
        pp_enabled_extension_names: todo!(),
        p_enabled_features: todo!(),
        _marker: std::marker::PhantomData,
        #[allow(deprecated)] //This is no longer used by vulkan, so i just set it to its default values
        enabled_layer_count: 0,
        #[allow(deprecated)]
        pp_enabled_layer_names: core::ptr::null(),
    };


    todo!()
}