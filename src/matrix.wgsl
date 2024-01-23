
struct Matrix {
    elements: array<f32>,
};

@group(0) @binding(0) var<storage, read> A : Matrix;
@group(0) @binding(1) var<storage, read> B : Matrix;
@group(0) @binding(2) var<storage, read_write> C : Matrix;


@compute @workgroup_size(4)
fn main(@builtin(global_invocation_id) global_id : vec3<u32>) {
    // Simple addition of two matrices:
    C.elements[global_id.x] = A.elements[global_id.x] + B.elements[global_id.x];
}
