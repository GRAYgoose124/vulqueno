use image::{ImageBuffer, Rgba};
use vulkano::{pipeline::{graphics::{viewport::{Viewport, ViewportState}, vertex_input::{BuffersDefinition}, input_assembly::InputAssemblyState}, GraphicsPipeline}, render_pass::Subpass, format::Format, buffer::{CpuAccessibleBuffer, BufferUsage}, command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyImageToBufferInfo}, image::{view::ImageView, ImageDimensions}, sync::{self, GpuFuture}};
use vulqueno::{shader_utils::load_shader_module, prelude::VulkanRuntime, imagebuffer::create_imagebuffer};
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo};
use vulkano::command_buffer::{RenderPassBeginInfo, SubpassContents};


use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
struct Vertex {
    position: [f32; 2],
}

vulkano::impl_vertex!(Vertex, position);

const TRIANGLE: [Vertex; 3] = [
    Vertex { position: [-0.5, -0.25] },
    Vertex { position: [0.0, 0.5] },
    Vertex { position: [0.25, -0.1] },
];

fn main() {
    let runtime = VulkanRuntime::new();

    // TODO: Screenbuffer/dims to SimpleShaderRuntime
    let dims = ImageDimensions::Dim2d { width: 2560, height: 1440, array_layers: 1 };
    let image = create_imagebuffer(&runtime, dims);

    let vertex_buffer = CpuAccessibleBuffer::from_iter(
        runtime.device.clone(),
        BufferUsage {
            vertex_buffer: true,
            ..Default::default()
        },
        false,
        vec![TRIANGLE[0], TRIANGLE[1], TRIANGLE[2]].into_iter(),
    )
    .unwrap();

    let render_pass = vulkano::single_pass_renderpass!(runtime.device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: Format::R8G8B8A8_UNORM,
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    )
    .unwrap();

    let view = ImageView::new_default(image.clone()).unwrap();
    let framebuffer = Framebuffer::new(
        render_pass.clone(),
        FramebufferCreateInfo {
            attachments: vec![view],
            ..Default::default()
        },
    )
    .unwrap();

    let fs = load_shader_module(&runtime, "shaders/basic_frag.spv");
    let vs = load_shader_module(&runtime, "shaders/basic_vert.spv");

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dims.width() as f32, dims.height() as f32],
        depth_range: 0.0..1.0,
    };

    let pipeline = GraphicsPipeline::start()
    // Describes the layout of the vertex input and how should it behave
    .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
    // A Vulkan shader can in theory contain multiple entry points, so we have to specify
    // which one.
    .vertex_shader(vs.entry_point("main").unwrap(), ())
    // Indicate the type of the primitives (the default is a list of triangles)
    .input_assembly_state(InputAssemblyState::new())
    // Set the fixed viewport
    .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport]))
    // Same as the vertex input, but this for the fragment input
    .fragment_shader(fs.entry_point("main").unwrap(), ())
    // This graphics pipeline object concerns the first pass of the render pass.
    .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
    // Now that everything is specified, we call `build`.
    .build(runtime.device.clone())
    .unwrap();
    
    let mut builder = AutoCommandBufferBuilder::primary(
        runtime.device.clone(),
        runtime.queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    let buf = CpuAccessibleBuffer::from_iter(
        runtime.device.clone(),
        BufferUsage {
            transfer_dst: true,
            ..Default::default()
        },
        false,
        (0..dims.width() * dims.height() * 4).map(|_| 0u8),
    )
    .expect("failed to create buffer");

    builder
        .begin_render_pass(
            RenderPassBeginInfo {
                clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
            },
            SubpassContents::Inline,
        ).unwrap()
        .bind_pipeline_graphics(pipeline.clone())
        .bind_vertex_buffers(0, vertex_buffer.clone())
        .draw(
            3, 1, 0, 0, 
        )
        .unwrap()
        .end_render_pass()
        .unwrap()
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(image, buf.clone()))
        .unwrap();
    
    let command_buffer = builder.build().unwrap();
    
    let future = sync::now(runtime.device.clone())
        .then_execute(runtime.queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();
    future.wait(None).unwrap();
    
    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(dims.width(), dims.height(), &buffer_content[..]).unwrap();
    image.save("image.png").unwrap();
    
    println!("Everything succeeded!");
   
}