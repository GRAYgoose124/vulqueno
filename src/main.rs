

mod instancer;
mod runner;



fn main() {
    let runtime = instancer::VulkanRuntime::new();

    let future = runner::create_shader_future(&runtime);
    future.wait(None).unwrap();


}
