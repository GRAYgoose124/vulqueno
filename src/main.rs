
mod instancer;

fn main() {
    let queue = instancer::get_device_queue();

    println!("Queue created, family index: {:?}",  queue.queue_family_index());
}
