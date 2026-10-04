use engine::winit::dpi::LogicalSize;
use engine::winit::event::{ElementState, MouseButton, WindowEvent};
use engine::winit::platform::windows::WindowAttributesExtWindows;
use engine::winit::window::{Window, WindowAttributes, WindowLevel};
use engine::{Frame, Game, Gpu, wgpu};

pub struct Cat;

impl Game for Cat {
    fn window_attributes() -> WindowAttributes {
        Window::default_attributes()
            .with_title("bongo")
            .with_inner_size(LogicalSize::new(300.0, 200.0))
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(false)
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_skip_taskbar(true)
    }

    fn new(_gpu: &Gpu) -> Self {
        Cat
    }

    fn render(&mut self, _gpu: &Gpu, frame: &mut Frame) {
        let _pass = frame.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Clear Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.2,
                        g: 0.0,
                        b: 0.0,
                        a: 0.5,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }

    fn window_event(&mut self, window: &Window, event: &WindowEvent) {
        if let WindowEvent::MouseInput{
            state: ElementState::Pressed,
            button: MouseButton::Left,
            ..
        } = event
        {
            let _ = window.drag_window();
        }
    }
}
