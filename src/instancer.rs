// Based on the Vulkano book.
// https://vulkano.rs/guide
use std::sync::Arc;

use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, DeviceCreateInfo, Features, Queue, QueueCreateInfo};
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::sync::{self, GpuFuture};
use vulkano::VulkanLibrary;

pub mod prelude {
    use crate::instancer::VulkanRuntime;
}

#[derive(Clone)]
pub struct VulkanRuntime {
    pub instance: Arc<Instance>,
    pub physical_device: Arc<PhysicalDevice>,
    pub device: Arc<Device>,
    pub queue: Arc<Queue>,
}

impl VulkanRuntime {
    pub fn new() -> Self {
        let (inst, phys) = init_vulkan_device();
        let (q, dev) = get_device_queue(phys.clone());

        Self {
            instance: inst,
            physical_device: phys,
            device: dev,
            queue: q,
        }
    }
}

impl Default for VulkanRuntime {
    fn default() -> Self {
        Self::new()
    }
}

fn init_vulkan_device() -> (Arc<Instance>, Arc<PhysicalDevice>) {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance =
        Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");

    println!(
        "Vulkan Instance created, API Version: {:?}",
        instance.api_version()
    );

    let physical = instance
        .enumerate_physical_devices()
        .expect("failed to enumerate physical devices")
        .next()
        .expect("no device available");

    #[cfg(feature = "verbose_vulkan_creation")]
    println!("Physical device: {:?}", physical.properties().device_name);

    (instance, physical)
}

fn get_device_queue(physical: Arc<PhysicalDevice>) -> (Arc<Queue>, Arc<Device>) {
    for family in physical.queue_family_properties() {
        println!(
            "\tFound a queue family with {:?} queue(s)",
            family.queue_count
        );
    }

    let queue_family_index = physical
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_, q)| q.queue_flags.graphics)
        .expect("couldn't find a graphical queue family") as u32;

    #[cfg(feature = "verbose_vulkan_creation")]
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

    #[cfg(feature = "verbose_vulkan_creation")]
    println!(
        "Device created, enabled features: {:?}",
        device.enabled_features()
    );

    let queue = queues.next().unwrap();

    #[cfg(feature = "verbose_vulkan_creation")]
    println!(
        "Queue created, family: {:?}:{:?}",
        queue.queue_family_index(),
        queue.id_within_family()
    );

    (queue, device)
}
