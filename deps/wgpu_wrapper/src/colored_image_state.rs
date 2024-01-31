use anyhow::Result;
use gm::{flat::Rect, volume::UIVertex};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    TextureFormat,
};

use crate::{image::Image, utils::make_pipeline, IMAGE_VERTICES};

#[derive(Debug)]
pub struct ColoredImageState {
    pub render_pipeline: wgpu::RenderPipeline,
    pub vertex_buffer:   wgpu::Buffer,
    pub num_vertices:    u32,
}

impl ColoredImageState {
    pub fn new(device: &wgpu::Device) -> Result<Self> {
        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/ui_image_color.wgsl"));

        let bind_group_layout = Image::bind_group_layout(device);

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:                Some("Colored Image Pipeline Layout"),
            bind_group_layouts:   &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = make_pipeline::<UIVertex>(
            "Colored Image Render Pipeline",
            device,
            &pipeline_layout,
            &shader,
            TextureFormat::Bgra8UnormSrgb,
        );

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label:    "Colored Image Vertex Buffer".into(),
            contents: bytemuck::cast_slice(IMAGE_VERTICES),
            usage:    wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = u32::try_from(IMAGE_VERTICES.len()).unwrap();

        Ok(Self {
            render_pipeline,
            vertex_buffer,
            num_vertices,
        })
    }

    pub fn draw<'a>(&'a self, image: &'static Image, rect: &Rect, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_viewport(rect.x(), rect.y(), rect.width(), rect.height(), 0.0, 1.0);
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &image.bind, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);
    }
}
