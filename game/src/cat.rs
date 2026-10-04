use engine::winit::dpi::LogicalSize;
use engine::winit::event::{ElementState, MouseButton, WindowEvent};
use engine::winit::platform::windows::WindowAttributesExtWindows;
use engine::winit::window::{Window, WindowAttributes, WindowLevel};
use engine::{Frame, Game, Gpu, Rect, Sprite, SpritePass, Texture, wgpu};

pub struct Cat {
    sprites: SpritePass,
    cat: Sprite,
}

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

    fn new(gpu: &Gpu) -> Self {
        let texture = Texture::from_bytes(
            &gpu.device,
            &gpu.queue,
            include_bytes!("../assets/cat.png"),
            "cat",
        )
        .unwrap();

        println!(
            "loaded cat texture: {}x{}",
            texture.size.width, texture.size.height
        );

        let sprites = SpritePass::new(gpu);
        let cat = sprites.create_sprite(
            gpu, 
            &texture,
            Rect {
                x: 0.0,
                y: 0.0,
                width: 1.0,
                height: 1.0,
            },
        );

        Cat {
            sprites,
            cat
        }
    }

    fn render(&mut self, _gpu: &Gpu, frame: &mut Frame) {
        let mut render_pass = frame
            .encoder
            .begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Cat Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.5,
                            g: 0.0,
                            b: 0.0,
                            a: 0.7,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

        self.sprites.draw(&mut render_pass, &self.cat);
    }

    fn window_event(&mut self, window: &Window, event: &WindowEvent) {
        if let WindowEvent::MouseInput {
            state: ElementState::Pressed,
            button: MouseButton::Left,
            ..
        } = event
        {
            let _ = window.drag_window();
        }
    }
}
