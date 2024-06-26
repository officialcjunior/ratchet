@group(0) @binding(0)
var<storage, read_write> C: array<f32>;

@group(0) @binding(1)
var<storage, read> S: array<f32>;

@group(0) @binding(2)
var<storage, read_write> D: array<f32>;

struct Meta {
    cache_stride: vec4<u32>,
    src_stride: vec4<u32>,
    dst_stride: vec4<u32>,
    dst_numel: u32,
    cum0: u32, //cache_numel
    cum1: u32, //cache_numel + src_numel
    dim: u32,
}

@group(1) @binding(0)
var<uniform> metadata: Meta;

//Converts 1D offset into 4D index
fn offsetToNdIndex(offset: u32, stride: vec4<u32>) -> vec4<u32> {
    var index: vec4<u32> = vec4<u32>(0u, 0u, 0u, 0u);
    var remaining = offset;

    for (var i: i32 = 0; i < 3; i++) {
        let idx = remaining / stride[i];
        index[i] = idx;
        remaining -= idx * stride[i];
    }
    index.w = remaining;
    return index;
}

//Converts 4D index into 1D offset
fn ndIndexToOffset(index: vec4<u32>, stride: vec4<u32>) -> u32 {
    var offset: u32 = 0u;
    for (var i: i32 = 0; i < 4; i++) {
        offset += index[i] * stride[i];
    }
    return offset;
}

@compute @workgroup_size(8,8,1)
fn main( 
        @builtin(local_invocation_id) local_id: vec3<u32>,
        @builtin(local_invocation_index) local_index: u32,
        @builtin(workgroup_id) group_id: vec3<u32>,
        @builtin(num_workgroups) num_groups: vec3<u32>
) {
    //Dispatch 1 thread per output element
    //dst_offset is index into the output buffer (1D)
    let x_offset = group_id.x * 64u;
    let dst_offset = (group_id.y * num_groups.x * 64u) + x_offset + local_index;
    if (dst_offset >= metadata.dst_numel) {
        return;
    }
    //Convert 1D offset into 4D index
    var dst_index = offsetToNdIndex(dst_offset, metadata.dst_stride);

    let dim = metadata.dim;
    if (dst_index[dim] < metadata.cum0) {
        //Inside cache, just copy from cache to DST
        let src_offset = ndIndexToOffset(dst_index, metadata.cache_stride);
        D[dst_offset] = C[src_offset];
        return;
    }

    if (dst_index[dim] < metadata.cum1) {
        //Inside src, copy from src to cache and then to DST
        let cache_offset = ndIndexToOffset(dst_index, metadata.cache_stride);
        dst_index[dim] -= metadata.cum0;
        let src_offset = ndIndexToOffset(dst_index, metadata.src_stride);
        let val = S[src_offset];
        C[cache_offset] = val; 
        D[dst_offset] = val; 
        return;
    }
}
