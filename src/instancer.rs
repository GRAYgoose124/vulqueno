use std::sync::Arc;

use vulkano::VulkanLibrary;
use vulkano::device::physical::PhysicalDevice;
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::device::{Device, DeviceCreateInfo, Features, QueueCreateInfo, Queue};
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, BufferAccess};
use vulkano::sync:: {self, GpuFuture};





fn init_vulkan_device() -> Arc<PhysicalDevice> {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");

    println!("Vulkan Instance created, API Version: {:?}", instance.api_version());

    let physical = instance
        .enumerate_physical_devices()
        .expect("failed to enumerate physical devices")
        .next()
        .expect("no device available");

    #[cfg(feature="verbose_vulkan_creation")]
    println!("Physical device: {:?}", physical.properties().device_name);

    physical
}

pub(crate) fn get_device_queue() -> Arc<Queue> {
    let physical = init_vulkan_device();

    for family in physical.queue_family_properties() {
        println!("\tFound a queue family with {:?} queue(s)", family.queue_count);
    }

    let queue_family_index = physical
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_, q)| q.queue_flags.graphics)
        .expect("couldn't find a graphical queue family") as u32;
    
    #[cfg(feature="verbose_vulkan_creation")]
    println!("\t\tQueue family index selected: {:?}", queue_family_index);

    let (device, mut queues) = Device::new(
        physical,
        DeviceCreateInfo {
            // here we pass the desired queue family to use by index
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    )
    .expect("failed to create device");

    #[cfg(feature="verbose_vulkan_creation")]
    println!("Device created, enabled features: {:?}", device.enabled_features());

    let queue = queues.next().unwrap();

    queue
}