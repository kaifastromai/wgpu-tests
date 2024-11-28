use std::path::Path;

use wgpu::{Extent3d, PipelineCompilationOptions};
use winit::window::Window;

pub struct ComputeState {
    device: wgpu::Device,
    queue: wgpu::Queue,
    compute_pipeline: wgpu::ComputePipeline,
    // vertex_buffer: wgpu::Buffer,
}

impl ComputeState {
    // Creating some of the wgpu types requires async code
    pub async fn new() -> ComputeState {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: None,
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .unwrap();
        let limits = device.limits();
        tracing::info!("HELLO???!");
        tracing::info!(?limits, "Loaded device");

        let compute_shader =
            device.create_shader_module(wgpu::include_wgsl!("../shaders/test_compute.wgsl"));
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            module: &compute_shader,
            entry_point: "compute_diff",
            compilation_options: PipelineCompilationOptions::default(),
            cache: None,
        });

        Self {
            device,
            queue,
            compute_pipeline,
            // vertex_buffer: buffer,
        }
    }
    pub fn compute(path1: &Path, path2: &Path) {
        let (image1, dim1) = load_tiff_image(path1).unwrap();
        let (image2, dim2) = load_tiff_image(path2).unwrap();
        let texture1_size = Extent3d {
            width: dim1.0,
            height: dim1.1,
            depth_or_array_layers: 1,
        };
        let texture2_size = Extent3d {
            width: dim2.0,
            height: dim2.1,
            depth_or_array_layers: 1,
        };
    }
}

pub fn load_tiff_image(path: &std::path::Path) -> std::io::Result<(Vec<u8>, (u32, u32))> {
    let file = std::fs::File::open(path)?;

    let mut image = tiff::decoder::Decoder::new(file).unwrap();
    let dim = image.dimensions().unwrap();

    let mut image = image.read_image().unwrap();

    let tiff::decoder::DecodingBuffer::F32(buf) = image.as_buffer(0) else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Could not decode as f32 image",
        ));
    };
    Ok((bytemuck::cast_slice(buf).to_owned(), dim))
}
#[test]
pub fn test_image_load() -> std::io::Result<()> {
    load_tiff_image(&Path::new("test_image1.tif"))?;
    Ok(())
}
