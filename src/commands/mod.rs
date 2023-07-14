mod run;
mod ast;

pub use run::run;
pub use ast::*;
use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyboardInput;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use crate::donitsi::Donitsi;

pub async fn donitsi() {
    Donitsi::new().run();

    // let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
    //     backends: wgpu::Backends::all(),
    //     dx12_shader_compiler: Default::default(),
    // });

    // let event_loop = EventLoop::new();
    // let window = WindowBuilder::new()
    //     .with_title("Donitsi")
    //     .build(&event_loop).unwrap();

    // let size = window.inner_size();

    // let surface = unsafe { instance.create_surface(&window) }.unwrap();

    // let adapter = instance.request_adapter(
    //     &wgpu::RequestAdapterOptions {
    //         power_preference: wgpu::PowerPreference::default(),
    //         compatible_surface: Some(&surface),
    //         force_fallback_adapter: false,
    //     },
    // ).await.unwrap();
    // let surface_caps = surface.get_capabilities(&adapter);

    // let (device, queue) = adapter.request_device(
    //     &wgpu::DeviceDescriptor {
    //         label: None,
    //         features: wgpu::Features::empty(),
    //         limits: wgpu::Limits::default(),
    //     },
    //     None, // Trace path
    // ).await.expect("Unable to create device and queue!");

    // // let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    // //     label: Some("Vertex Shader"),
    // //     source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("../../shaders/vertex_shader.wgsl"))),
    // // });
    
    // // let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
    // //     label: Some("Fragment Shader"),
    // //     source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!("../../shaders/fragment_shader.wgsl"))),
    // // });

    // let shader = device.create_shader_module(wgpu::include_wgsl!("../../shaders/shader.wgsl"));

    // let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
    //     label: Some("Render Pipeline Layout"),
    //     bind_group_layouts: &[],
    //     push_constant_ranges: &[],
    // });

    // let surface_format = surface_caps.formats.iter()
    //     .copied()
    //     .next()            
    //     .unwrap_or(surface_caps.formats[0]);
    // let config = wgpu::SurfaceConfiguration {
    //     usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
    //     format: surface_format,
    //     width: size.width,
    //     height: size.height,
    //     present_mode: surface_caps.present_modes[0],
    //     alpha_mode: surface_caps.alpha_modes[0],
    //     view_formats: vec![],
    // };
    
    // let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
    //     label: Some("Render Pipeline"),
    //     layout: Some(&render_pipeline_layout),
    //     vertex: wgpu::VertexState {
    //         module: &shader,
    //         entry_point: "vs_main",
    //         buffers: &[],
    //     },
    //     fragment: Some(wgpu::FragmentState {
    //         module: &shader,
    //         entry_point: "fs_main",
    //         targets: &[Some(wgpu::ColorTargetState {
    //             format: config.format,
    //             blend: Some(wgpu::BlendState::REPLACE),
    //             write_mask: wgpu::ColorWrites::ALL,
    //         })],
    //     }),
    //     primitive: wgpu::PrimitiveState {
    //         topology: wgpu::PrimitiveTopology::TriangleList,
    //         ..Default::default()
    //     },
    //     depth_stencil: None,
    //     multisample: wgpu::MultisampleState::default(),
    //     multiview: None
    // });
    

    // event_loop.run(move |event, _, control_flow| match event {
    //     Event::WindowEvent {
    //         ref event,
    //         window_id,
    //     } if window_id == window.id() => match event {
    //         WindowEvent::CloseRequested
    //         | WindowEvent::KeyboardInput {
    //             input:
    //                 KeyboardInput {
    //                     state: ElementState::Pressed,
    //                     virtual_keycode: Some(VirtualKeyCode::Escape),
    //                     ..
    //                 },
    //             ..
    //         } => *control_flow = ControlFlow::Exit,
    //         _ => {}
    //     },
    //     _ => {}
    // });
}