use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};

use vulqueno::prelude::*;

#[test]
fn check_compute_shader() {
    let runtime = VulkanRuntime::new();

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

    let future = execute_compute("shaders/shader.spv", data_buffer.clone(), &runtime);

    future.wait(None).unwrap();

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }

    println!("Success!")
}
