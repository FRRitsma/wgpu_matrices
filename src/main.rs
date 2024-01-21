use wgpu::util::DeviceExt;
use wgpu::{BindGroupLayoutEntry, Device, Queue};
const MATRIX_A: [f32; 4] = [1.0, 2.0, 3.0, 4.0];
const MATRIX_B: [f32; 4] = [7.0, 8.0, 9.0, 10.0];

async fn initialize_components() -> (Device, Queue) {
    let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
    let adapter = instance.request_adapter(&Default::default()).await.unwrap();
    let features = adapter.features();
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: features & wgpu::Features::TIMESTAMP_QUERY,
                limits: Default::default(),
            },
            None,
        )
        .await
        .unwrap();
    (device, queue)
}

fn bind_group_layout_entry(binding: u32) -> BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

#[tokio::main]
async fn main() {
    // let cpu_buffer_a: &[u8] = bytemuck::cast_slice(&MATRIX_A);
    // let cpu_buffer_b: &[u8] = bytemuck::cast_slice(&MATRIX_B);

    let (device, _queue) = initialize_components().await;

    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("matrix.wgsl").into()),
    });

    // 1. Create Buffer Objects
    let _buffer_a = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Matrix A Buffer"),
        contents: bytemuck::cast_slice(&MATRIX_A),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let _buffer_b = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Matrix B Buffer"),
        contents: bytemuck::cast_slice(&MATRIX_B),
        usage: wgpu::BufferUsages::STORAGE,
    });

    let _buffer_c = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Matrix C Buffer"),
        size: std::mem::size_of_val(&MATRIX_A) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Compute Bind Group Layout"),
        entries: &[
            bind_group_layout_entry(0),
            bind_group_layout_entry(1),
            bind_group_layout_entry(2),
        ],
    });

    // 2. Create Compute Pipeline
    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout], // Use the defined bind group layout here
        push_constant_ranges: &[],
    });

    let _compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&compute_pipeline_layout),
        module: &cs_module,
        entry_point: "main", // Name of the entry point in your WGSL shader
    });
}
