
struct Matrix {
    elements: array<f32>,
};

@group(0) @binding(0) var<storage, read> A : Matrix;
@group(0) @binding(1) var<storage, read> B : Matrix;
@group(0) @binding(2) var<storage, read_write> C : Matrix;

@compute @workgroup_size(2, 2)
fn main(@builtin(global_invocation_id) global_id : vec3<u32>) {
    let M: u32 = 2u; // Number of rows in matrix A and C
    let K: u32 = 3u; // Number of columns in matrix A and rows in matrix B
    let N: u32 = 2u; // Number of columns in matrix B and C

    let row: u32 = global_id.x;
    let col: u32 = global_id.y;

    if (row < M && col < N) {
        var sum: f32 = 0.0;
        for (var k: u32 = 0u; k < K; k = k + 1u) {
            sum = sum + A.elements[row * K + k] * B.elements[k * N + col];
        }
        C.elements[row * N + col] = sum;
    }
}
