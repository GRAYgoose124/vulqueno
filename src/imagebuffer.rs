use std::sync::Arc;


use vulkano::image::{ImageDimensions, StorageImage};
use vulkano::format::{Format};

use crate::prelude::VulkanRuntime;


pub fn create_imagebuffer(runtime: &VulkanRuntime, dims: ImageDimensions) -> Arc<StorageImage> {
    StorageImage::new(
        runtime.device.clone(),
        dims,
        Format::R8G8B8A8_UNORM,
        
        Some(runtime.queue.queue_family_index()),
    )
    .unwrap()
}
