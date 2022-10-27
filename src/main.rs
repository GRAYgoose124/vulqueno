use vulkano::buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer};

mod instancer;

fn main() {
    let runtime = instancer::VulkanRuntime::new();

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
}
