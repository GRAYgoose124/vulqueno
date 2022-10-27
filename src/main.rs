use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};

mod compute_runner;
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

    let future =
        compute_runner::run_compute_shader("shaders/shader.spv", data_buffer.clone(), &runtime);

    future.wait(None).unwrap();

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }

    println!("Everything succeeded!");
}
