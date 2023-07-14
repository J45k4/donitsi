use std::collections::HashMap;

use winit::event::ElementState;
use winit::event::Event;
use winit::event::KeyboardInput;
use winit::event::VirtualKeyCode;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::window::WindowId;

use crate::window::Window;


pub struct Donitsi {
    pub instance: wgpu::Instance,
    pub event_loop: EventLoop<()>,
    pub windows: HashMap<WindowId, Window>,
}

impl Donitsi {
    pub fn new() -> Self {
        Self {
            instance: wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                dx12_shader_compiler: Default::default(),
            }),
            event_loop: EventLoop::new(),
            windows: HashMap::new(),
        }
    }

    async fn create_window(&self, title: &str) -> Window {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop).unwrap();

        Window::new(&event_loop, &self.instance).await
    }

    pub fn run(mut self) {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } => {
                    println!("[{:?}] window event: {:?}", window_id, event);

                    

                    match self.windows.get_mut(&window_id) {
                        Some(window) => {
                            match event {
                                WindowEvent::CloseRequested
                                | WindowEvent::KeyboardInput {
                                    input:
                                        KeyboardInput {
                                            state: ElementState::Pressed,
                                            virtual_keycode: Some(VirtualKeyCode::Escape),
                                            ..
                                        },
                                    ..
                                } => *control_flow = ControlFlow::Exit,
                                WindowEvent::Resized(physical_size) => {
                                    window.resize(*physical_size);
                                }
                                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                                    // new_inner_size is &mut so w have to dereference it twice
                                    window.resize(**new_inner_size);
                                }
                                _ => {}
                            }

                            // match window.input(event) {
                            //     Ok(_) => {}
                            //     Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            //         // TODO state.
                            //     }
                            //     Err(e) => {
                            //         eprintln!("Error: {:?}", e);
                            //     }
                            // }
                        }
                        None => {
                            println!("Window not found: {:?}", window_id);
                        }
                    }

                    // if window_id == state.window().id() {
                    //     if !state.input(event) {
                    //         match event {
                    //             WindowEvent::CloseRequested
                    //             | WindowEvent::KeyboardInput {
                    //                 input:
                    //                     KeyboardInput {
                    //                         state: ElementState::Pressed,
                    //                         virtual_keycode: Some(VirtualKeyCode::Escape),
                    //                         ..
                    //                     },
                    //                 ..
                    //             } => *control_flow = ControlFlow::Exit,
                    //             WindowEvent::Resized(physical_size) => {
                    //                 state.resize(*physical_size);
                    //             }
                    //             WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    //                 // new_inner_size is &mut so w have to dereference it twice
                    //                 state.resize(**new_inner_size);
                    //             }
                    //             _ => {}
                    //         }
                    //     }
                    // }
                }
                Event::RedrawRequested(window_id) => {
                    match self.windows.get_mut(&window_id) {
                        Some(window) => {
                            match window.render() {
                                Ok(_) => {}
                                // Reconfigure the surface if it's lost or outdated
                                Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                    // TODO state.resize(state.size)
                                }
                                // The system is out of memory, we should probably quit
                                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                                // We're ignoring timeouts
                                Err(wgpu::SurfaceError::Timeout) => log::warn!("Surface timeout"),
                            }
                        },
                        None => {
                            println!("Window not found: {:?}", window_id);
                        }
                    }
                }
                Event::MainEventsCleared => {
                    for window in self.windows.values() {
                        window.request_redraw();
                    }
                }
                _ => {}
            }
        });
    }
}