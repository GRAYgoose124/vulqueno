use vulkano::buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer};
use vulkano::pipeline::ComputePipeline;

use vulkano::pipeline::Pipeline;
use vulkano::descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet};

use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CommandBufferExecFuture, PrimaryAutoCommandBuffer};
use vulkano::pipeline::PipelineBindPoint;
use vulkano::sync::{self, GpuFuture, NowFuture, FenceSignalFuture};

use crate::instancer::VulkanRuntime;


pub(crate) fn create_shader_future(runtime: &VulkanRuntime) -> FenceSignalFuture<CommandBufferExecFuture<NowFuture, PrimaryAutoCommandBuffer>> {
    let data_iter = 0..65536;
    let data_buffer = CpuAccessibleBuffer::from_iter(
        runtime.device.clone(),
        BufferUsage {
            storage_buffer: true,
            ..Default::default()
        },
        false,
        data_iter,
    )
    .expect("failed to create buffer");

    // Loading shader source from macro.
    let shader = cs::load(runtime.device.clone())
    .expect("failed to create shader module");

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

    let command_buffer = builder.build().unwrap();

    sync::now(runtime.device.clone())
    .then_execute(runtime.queue.clone(), command_buffer)
    .unwrap()
    .then_signal_fence_and_flush()
    .unwrap()
}

mod cs {
    vulkano_shaders::shader!{
        ty: "compute",
        src: 
"#version 450

layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {
    uint data[];
} buf;

void main() {
    uint idx = gl_GlobalInvocationID.x;
    buf.data[idx] *= 12;
}"  }
}
