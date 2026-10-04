use wgpu::util::DeviceExt;

use crate::gpu::Gpu;
use crate::mesh::{Mesh, SpriteVertex, Vertex};
use crate::texture::Texture;

const QUAD_VERTICES: &[SpriteVertex] = &[
    SpriteVertex { position: [0.0, 0.0], tex_coords: [0.0, 0.0] },
    SpriteVertex { position: [0.0, 1.0], tex_coords: [0.0, 1.0] },
    SpriteVertex { position: [1.0, 1.0], tex_coords: [1.0, 1.0] },
    SpriteVertex { position: [1.0, 0.0], tex_coords: [1.0, 0.0] },
];

const QUAD_INDICES: &[u32] = &[0, 1, 2, 0, 2, 3];

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct Sprite {
    bind_group: wgpu::BindGroup,
    rect_buffer: wgpu::Buffer,
}

impl Sprite {
    pub fn set_rect(&self, gpu: &Gpu, rect: Rect) {
        gpu.queue
            .write_buffer(&self.rect_buffer, 0, bytemuck::cast_slice(&[rect]));
    }
}

pub struct SpritePass {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    quad: Mesh,
}

impl SpritePass {
    pub fn new(gpu: &Gpu) -> Self {
        let bind_group_layout = gpu.device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("sprite_bind_group_layout"),
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
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
            });

        let pipeline_layout = gpu.device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("sprite_pipeline_layout"),
                bind_group_layouts: &[Some(&bind_group_layout)],
                immediate_size: 0,
            });

        let shader = gpu.device
            .create_shader_module(wgpu::include_wgsl!("sprite.wgsl"));

        let pipeline = gpu.device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("sprite_pipeline"),
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

        let quad = Mesh::new(&gpu.device, QUAD_VERTICES, QUAD_INDICES, "sprite_quad");

        Self {
            pipeline,
            bind_group_layout,
            quad,
        }
    }

    pub fn create_sprite(&self, gpu: &Gpu, texture: &Texture, rect: Rect) -> Sprite {
        let rect_buffer = gpu.device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("sprite_rect"),
                contents: bytemuck::cast_slice(&[rect]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        
        let bind_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("sprite_bind_group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry{
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: rect_buffer.as_entire_binding(),
                },
            ],
        });

        Sprite {
            bind_group,
            rect_buffer,
        }
    }

    pub fn draw(&self, render_pass: &mut wgpu::RenderPass<'_>, sprite: &Sprite) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &sprite.bind_group, &[]);
        self.quad.draw(render_pass);
    }
}

