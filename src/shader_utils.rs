use std::{sync::Arc, io::Read};

use vulkano::shader::ShaderModule;

use crate::prelude::VulkanRuntime;

pub fn load_shader_module(runtime: &VulkanRuntime, shader_path: &str) -> Arc<ShaderModule> {
    let path = std::path::Path::new(shader_path);
    let mut file = std::fs::File::open(path).expect("failed to open shader file");

    let mut code = Vec::new();
    file.read_to_end(&mut code).expect("failed to read shader file");

    let shader = unsafe { ShaderModule::from_bytes(runtime.device.clone(), &code)
                                            .expect("failed to create shader module") };
    shader
}
