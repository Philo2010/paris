use ash::vk::{ComponentMapping, ComponentSwizzle, Format, Image, ImageAspectFlags, ImageSubresourceRange, ImageView, ImageViewCreateFlags, ImageViewCreateInfo, ImageViewType, StructureType};

use crate::rawdaug::error::RDError;


pub unsafe fn new(images: &Vec<Image>, format: Format, device: &ash::Device) -> Result<Vec<ImageView>, RDError> {
    let mut views: Vec<ImageView> = Vec::with_capacity(images.len());

    for image in images {
        let create_info = ImageViewCreateInfo {
            s_type: StructureType::IMAGE_VIEW_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: ImageViewCreateFlags::empty(),
            image: *image,
            view_type: ImageViewType::TYPE_2D,
            format: format,
            components: ComponentMapping {
                r: ComponentSwizzle::IDENTITY,
                g: ComponentSwizzle::IDENTITY,
                b: ComponentSwizzle::IDENTITY,
                a: ComponentSwizzle::IDENTITY,
            },
            subresource_range: ImageSubresourceRange {
                aspect_mask: ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            _marker: std::marker::PhantomData,
        };

        //needs to be cleaned up in RDObject to be safe
        let view = unsafe { device.create_image_view(&create_info, None)? };
        views.push(view);
    }

    Ok(views)
}

pub unsafe fn clean(views: &Vec<ImageView>, device: &ash::Device) {
    for view in views {
        unsafe { device.destroy_image_view(*view, None) };
    }
}