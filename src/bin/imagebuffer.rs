use vulkano::buffer::{CpuAccessibleBuffer, BufferUsage};
use vulkano::command_buffer::{AutoCommandBufferBuilder, ClearColorImageInfo, CommandBufferUsage, CopyImageToBufferInfo};
use vulkano::image::{ImageDimensions, StorageImage, ImageAccess};
use vulkano::format::{Format, ClearColorValue};
use vulkano::sync::{self, GpuFuture};

use image::{ImageBuffer, Rgba};

use vulqueno::prelude::VulkanRuntime;

// TODO: Refactor
fn main() { 
    let runtime = VulkanRuntime::new();
    
    let dims = ImageDimensions::Dim2d { width: 2560, height: 1440, array_layers: 1 };

    // Create image surface.
    let image = StorageImage::new(
        runtime.device.clone(),
        dims,
        Format::R8G8B8A8_UNORM,
        
        Some(runtime.queue.queue_family_index()),
    )
    .unwrap(); 

    let mut builder = AutoCommandBufferBuilder::primary(
        runtime.device.clone(),
        runtime.queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();
    
    builder
        .clear_color_image(ClearColorImageInfo {
            clear_value: ClearColorValue::Float([0.0, 0.0, 1.0, 1.0]),
            ..ClearColorImageInfo::image(image.clone())
        })
        .unwrap();

    // Image Buffer
    let buf = CpuAccessibleBuffer::from_iter(
        runtime.device.clone(),
        BufferUsage {
            transfer_dst: true,
            ..Default::default()
        },
        false,
        (0..dims.width() * dims.height() * 4).map(|_| 0u8),
    )
    .expect("failed to create buffer");

    // Copy image to buffer with the command buffer
    builder
    .clear_color_image(ClearColorImageInfo {
        clear_value: ClearColorValue::Float([0.0, 0.0, 1.0, 1.0]),
        ..ClearColorImageInfo::image(image.clone())
    })
    .unwrap()
    .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
        image.clone(),
        buf.clone(),
    ))
    .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(runtime.device.clone())
    .then_execute(runtime.queue.clone(), command_buffer)
    .unwrap()
    .then_signal_fence_and_flush()
    .unwrap();

    future.wait(None).unwrap();

    println!("Image: {:?}", image.dimensions());
}
