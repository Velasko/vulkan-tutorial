#![allow(unused)]
#![allow(non_snake_case)]

use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

use vulkano::{
    Version, VulkanLibrary,
    instance::{Instance, InstanceCreateInfo, InstanceExtensions},
    swapchain::Surface,
};

struct HelloTriangleApplication {
    window: Option<Window>,
    instance: Arc<Instance>,
}

impl HelloTriangleApplication {
    fn new(event_loop: &EventLoop<()>) -> Self {
        let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
        let required_extensions = Surface::required_extensions(&event_loop).unwrap();
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                // flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )
        .expect("failed to create instance");
        Self {
            window: None,
            instance,
        }
    }
}

// lunarg's cleanup()
// impl Drop for HelloTriangleApplication {
//     fn drop(&mut self) {}
// }

impl ApplicationHandler for HelloTriangleApplication {
    // lunarg's initWindow()
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();
        // window.set_visible(false);
        window.set_title("Lunarg Vulkan");
        window.set_resizable(false);
        window.set_min_inner_size(Some(LogicalSize::new(800, 600)));
        self.window = Some(window);
    }

    // lunarg's mainLoop()
    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("Close button pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    if let Ok(event_loop) = EventLoop::new() {
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut app = HelloTriangleApplication::new(&event_loop);

        // lunarg's app.run()
        event_loop.run_app(&mut app);
    };
}
