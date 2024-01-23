use matrix::Matrix;
use wgpu::{BindGroupLayoutEntry, Buffer, Device, Queue};

mod matrix;

const K: usize = 100;
const M: usize = 1000;
const N: usize = 10000;
const MATRIX_A: [f32; K * M] = [1.0; K * M];
const MATRIX_B: [f32; K * N] = [1.0; K * N];

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

fn bind_group_layout_entry(binding: u32, read_only: bool) -> BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn result_buffer(device: &Device, buffer_length: u64) -> Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: buffer_length,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    })
}

#[tokio::main]
async fn main() {
    let (device, queue) = initialize_components().await;
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("matrix_backup.wgsl").into()),
    });

    let matrix_a = Matrix::new(M, K, &MATRIX_A);
    let matrix_b = Matrix::new(K, N, &MATRIX_B);

    // 1. Create Buffer Objects
    let buffer_a = matrix_a.as_buffer(&device);
    let buffer_b = matrix_b.as_buffer(&device);
    let buffer_c = matrix_a.result_buffer_multiplication(&device, matrix_b);

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("Compute Bind Group Layout"),
        entries: &[
            bind_group_layout_entry(0, true),
            bind_group_layout_entry(1, true),
            bind_group_layout_entry(2, false),
        ],
    });

    // 2. Create Compute Pipeline
    let compute_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Compute Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout], // Use the defined bind group layout here
        push_constant_ranges: &[],
    });

    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("Compute Pipeline"),
        layout: Some(&compute_pipeline_layout),
        module: &cs_module,
        entry_point: "main", // Name of the entry point in your WGSL shader
    });

    // 3. Create Bind Groups
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0, // Corresponds to buffer A
                resource: buffer_a.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1, // Corresponds to buffer B
                resource: buffer_b.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2, // Corresponds to buffer C
                resource: buffer_c.as_entire_binding(),
            },
        ],
        label: Some("Bind Group"),
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Command Encoder"),
    });

    {
        let n = 16;
        let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("Compute Pass"),
        });
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(n, n, 1); // Adjust N according to your workgroup size and number of elements
    }

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Matrix C CPU Buffer"),
        size: buffer_c.size(),
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    encoder.copy_buffer_to_buffer(&buffer_c, 0, &staging_buffer, 0, buffer_c.size());
    queue.submit(Some(encoder.finish()));

    // Step 4: Map the staging buffer and read data
    let output_slice = staging_buffer.slice(..);
    let (sender, receiver) = futures_intrusive::channel::shared::oneshot_channel();
    output_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    let pre_poll = std::time::Instant::now();
    device.poll(wgpu::Maintain::Wait);
    let post_poll = std::time::Instant::now();
    println!("poll time: {:?}", post_poll - pre_poll);
    if let Some(Ok(())) = receiver.receive().await {
        let data_raw = &*output_slice.get_mapped_range();
        let _data: &[f32] = bytemuck::cast_slice(data_raw);
    }
}
