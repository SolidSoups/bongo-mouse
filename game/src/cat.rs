use engine::winit::dpi::LogicalSize;
use engine::winit::event::{ElementState, MouseButton, WindowEvent};
use engine::winit::platform::windows::WindowAttributesExtWindows;
use engine::winit::window::{Window, WindowAttributes, WindowLevel};
use engine::{Frame, Game, Gpu, Mesh, SpriteVertex, Texture, Vertex, wgpu};

const QUAD_VERTICES: &[SpriteVertex] = &[
    SpriteVertex {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 0.0],
    },
    SpriteVertex {
        position: [-1.0, -1.0],
        tex_coords: [0.0, 1.0],
    },
    SpriteVertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 1.0],
    },
    SpriteVertex {
        position: [1.0, 1.0],
        tex_coords: [1.0, 0.0],
    },
];

const QUAD_INDICES: &[u32] = &[0, 1, 2, 0, 2, 3];

pub struct Cat {
    quad: Mesh,
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
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

        let quad = Mesh::new(&gpu.device, QUAD_VERTICES, QUAD_INDICES, "quad");

        let bind_group_layout =
            gpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("cat_bind_group_layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                    ],
                });

        let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cat_bind_group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
        });

        let pipeline_layout = gpu
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("cat_pipeline_layout"),
                bind_group_layouts: &[Some(&bind_group_layout)],
                immediate_size: 0,
            });
        let shader = gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("sprite.wgsl"));
        let pipeline = gpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("cat_pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[Some(SpriteVertex::desc())],
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: gpu.config.format.add_srgb_suffix(),
                        blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: Default::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            });

        Cat {
            quad,
            pipeline,
            bind_group,
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
                        load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        self.quad.draw(&mut render_pass);
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
