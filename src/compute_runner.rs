use std::fs::File;
use std::io::Read;
use std::sync::Arc;

use vulkano::buffer::BufferAccess;
use vulkano::pipeline::ComputePipeline;

use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};
use vulkano::pipeline::Pipeline;

use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferExecFuture, CommandBufferUsage, PrimaryAutoCommandBuffer,
};
use vulkano::pipeline::PipelineBindPoint;
use vulkano::shader::ShaderModule;
use vulkano::sync::{self, FenceSignalFuture, GpuFuture, NowFuture};

use crate::instancer::VulkanRuntime;

pub fn execute_compute(
    shader_path: &str,
    // Todo: buffers
    data_buffer: Arc<dyn BufferAccess>,
    runtime: &VulkanRuntime,
) -> FenceSignalFuture<CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer>> {
    // Loading shader source from macro.

    let shader = {
        let path = std::path::Path::new(shader_path);
        let mut file = File::open(path).expect("Failed to open shader file");
        let mut v = vec![];
        file.read_to_end(&mut v).unwrap();
        unsafe { ShaderModule::from_bytes(runtime.device.clone(), &v) }.unwrap()
    };

    // Create compute pipeline from shader.
    let compute_pipeline = ComputePipeline::new(
        runtime.device.clone(),
        shader.entry_point("main").unwrap(),
        &(),
        None,
        |_| {},
    )
    .expect("failed to create compute pipeline");

    // Descriptor set.
    let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
    let set = PersistentDescriptorSet::new(
        layout.clone(),
        [WriteDescriptorSet::buffer(0, data_buffer.clone())], // 0 is the binding
    )
    .unwrap();

    let mut builder = AutoCommandBufferBuilder::primary(
        runtime.device.clone(),
        runtime.queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .bind_pipeline_compute(compute_pipeline.clone())
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            compute_pipeline.layout().clone(),
            0, // 0 is the index of our set
            set,
        )
        .dispatch([1024, 1, 1])
        .unwrap();

    // Command buffer and execute it, returning a future.
    let command_buffer = builder.build().unwrap();

    sync::now(runtime.device.clone())
        .then_execute(runtime.queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap()
}
