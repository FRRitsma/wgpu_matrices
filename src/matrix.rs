// Allow unused functions:
#![allow(dead_code)]

use wgpu::util::DeviceExt;
use wgpu::{Buffer, Device};

pub struct Matrix<'a> {
    // Entries are stored in row order
    rows: usize,
    columns: usize,
    entries: &'a [f32],
}

impl<'a> Matrix<'a> {
    pub fn new(rows: usize, columns: usize, entries: &'a [f32]) -> Self {
        // Panic if rows* columns != entries.len():
        assert_eq!(rows * columns, entries.len());
        Matrix {
            rows,
            columns,
            entries,
        }
    }

    fn buffer_length(&self) -> u64 {
        self.entries.len() as u64 * 4
    }

    pub fn as_buffer(&self, device: &Device) -> Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(self.entries),
            usage: wgpu::BufferUsages::STORAGE,
        })
    }

    pub fn result_buffer_addition(&self, device: &Device, other: Matrix) -> Buffer {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.columns, other.columns);
        crate::result_buffer(device, self.buffer_length())
    }

    pub fn result_buffer_multiplication(&self, device: &Device, other: Matrix) -> Buffer {
        assert_eq!(self.columns, other.rows);
        let buffer_length = (self.rows * other.columns * 4) as u64;
        crate::result_buffer(device, buffer_length)
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        // Panic if dimensions don't match:
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.columns, other.columns);
        todo!(
            "Implement addition of matrices with dimensions {}x{} and {}x{}",
            self.rows,
            self.columns,
            other.rows,
            other.columns
        )
    }
}

#[test]
pub fn instantiate_matrix() {
    let entries: [f32; 2] = [1.0, 2.0];
    let rows: usize = 1;
    let columns: usize = 2;
    let matrix = Matrix::new(rows, columns, &entries);
    assert_eq!(matrix.buffer_length(), 8u64);
}
